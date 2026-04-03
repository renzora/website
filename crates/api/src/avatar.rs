use axum::{
    extract::{Extension, Path, State},
    routing::{get, post, put},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::avatar::{AvatarPart, UserAvatar, UserAvatarPart};
use renzora_models::user::User;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(get_my_avatar))
        .route("/me", put(save_avatar))
        .route("/my-parts", get(get_my_parts))
        .route("/purchase/{part_id}", post(purchase_part))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/parts", get(list_parts))
        .route("/user/{user_id}", get(get_user_avatar))
        .merge(protected)
}

/// List all avatar parts grouped by slot. If authenticated, includes ownership info.
async fn list_parts(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<AvatarPartsListResponse>, ApiError> {
    let all_parts = AvatarPart::list_all(&state.db).await?;

    // Try to get user ID from token
    let user_id = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| crate::jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub);

    let owned_ids = if let Some(uid) = user_id {
        UserAvatarPart::list_owned_ids(&state.db, uid).await.unwrap_or_default()
    } else {
        vec![]
    };

    let mut grouped: HashMap<String, Vec<AvatarPartResponse>> = HashMap::new();
    for part in all_parts {
        let owned = part.is_default || owned_ids.contains(&part.id);
        grouped.entry(part.slot.clone()).or_default().push(AvatarPartResponse {
            id: part.id,
            slot: part.slot,
            name: part.name,
            slug: part.slug,
            part_data: part.part_data,
            price_credits: part.price_credits,
            is_default: part.is_default,
            owned,
        });
    }

    Ok(Json(AvatarPartsListResponse { parts: grouped }))
}

/// Get a specific user's avatar config.
async fn get_user_avatar(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserAvatarResponse>, ApiError> {
    let avatar = UserAvatar::find_by_user(&state.db, user_id).await?;
    match avatar {
        Some(a) => Ok(Json(UserAvatarResponse {
            skin_color: a.skin_color,
            eye_color: a.eye_color,
            hair_color: a.hair_color,
            equipped_parts: a.equipped_parts,
        })),
        None => Ok(Json(UserAvatarResponse {
            skin_color: "#C68642".into(),
            eye_color: "#4A90D9".into(),
            hair_color: "#3B2F2F".into(),
            equipped_parts: serde_json::json!({}),
        })),
    }
}

/// Get current user's avatar.
async fn get_my_avatar(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserAvatarResponse>, ApiError> {
    let avatar = UserAvatar::find_by_user(&state.db, auth.user_id).await?;
    match avatar {
        Some(a) => Ok(Json(UserAvatarResponse {
            skin_color: a.skin_color,
            eye_color: a.eye_color,
            hair_color: a.hair_color,
            equipped_parts: a.equipped_parts,
        })),
        None => Ok(Json(UserAvatarResponse {
            skin_color: "#C68642".into(),
            eye_color: "#4A90D9".into(),
            hair_color: "#3B2F2F".into(),
            equipped_parts: serde_json::json!({}),
        })),
    }
}

/// Save avatar config. Validates that user owns all equipped parts.
async fn save_avatar(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SaveAvatarRequest>,
) -> Result<Json<UserAvatarResponse>, ApiError> {
    // Validate hex colors
    for color in [&body.skin_color, &body.eye_color, &body.hair_color] {
        if !color.starts_with('#') || color.len() != 7 {
            return Err(ApiError::Validation("Invalid color format".into()));
        }
    }

    // Validate ownership of all equipped parts
    if let Some(obj) = body.equipped_parts.as_object() {
        let owned_ids = UserAvatarPart::list_owned_ids(&state.db, auth.user_id).await?;
        for (_slot, part_id_val) in obj {
            if let Some(part_id_str) = part_id_val.as_str() {
                if let Ok(part_id) = Uuid::parse_str(part_id_str) {
                    // Check if default or owned
                    let part = AvatarPart::find_by_id(&state.db, part_id).await?
                        .ok_or(ApiError::Validation("Unknown part".into()))?;
                    if !part.is_default && !owned_ids.contains(&part_id) {
                        return Err(ApiError::Validation(format!("You don't own part '{}'", part.name)));
                    }
                }
            }
        }
    }

    let avatar = UserAvatar::upsert(
        &state.db,
        auth.user_id,
        &body.skin_color,
        &body.eye_color,
        &body.hair_color,
        body.equipped_parts,
    ).await?;

    Ok(Json(UserAvatarResponse {
        skin_color: avatar.skin_color,
        eye_color: avatar.eye_color,
        hair_color: avatar.hair_color,
        equipped_parts: avatar.equipped_parts,
    }))
}

/// Purchase a part with credits.
async fn purchase_part(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(part_id): Path<Uuid>,
) -> Result<Json<PurchasePartResponse>, ApiError> {
    let part = AvatarPart::find_by_id(&state.db, part_id).await?
        .ok_or(ApiError::NotFound)?;

    if part.is_default || part.price_credits == 0 {
        // Free part, just grant it
        UserAvatarPart::grant(&state.db, auth.user_id, part_id).await?;
        let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
        return Ok(Json(PurchasePartResponse {
            message: "Part unlocked!".into(),
            new_balance: user.credit_balance,
        }));
    }

    // Check already owned
    if UserAvatarPart::owns_part(&state.db, auth.user_id, part_id).await? {
        return Err(ApiError::Validation("You already own this part".into()));
    }

    // Check balance
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    if user.credit_balance < part.price_credits {
        return Err(ApiError::Validation("Not enough credits".into()));
    }

    // Deduct credits
    sqlx::query("UPDATE users SET credit_balance = credit_balance - $1 WHERE id = $2")
        .bind(part.price_credits)
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;

    // Record transaction
    sqlx::query("INSERT INTO transactions (user_id, type, amount, description) VALUES ($1, 'purchase', $2, $3)")
        .bind(auth.user_id)
        .bind(-part.price_credits)
        .bind(format!("Avatar part: {}", part.name))
        .execute(&state.db)
        .await?;

    // Grant ownership
    UserAvatarPart::grant(&state.db, auth.user_id, part_id).await?;

    let updated_user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(PurchasePartResponse {
        message: format!("Purchased '{}'!", part.name),
        new_balance: updated_user.credit_balance,
    }))
}

/// Get all parts owned by the current user (including defaults).
async fn get_my_parts(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<AvatarPartResponse>>, ApiError> {
    let all_parts = AvatarPart::list_all(&state.db).await?;
    let owned_ids = UserAvatarPart::list_owned_ids(&state.db, auth.user_id).await?;

    let result: Vec<AvatarPartResponse> = all_parts
        .into_iter()
        .filter(|p| p.is_default || owned_ids.contains(&p.id))
        .map(|p| AvatarPartResponse {
            id: p.id,
            slot: p.slot,
            name: p.name,
            slug: p.slug,
            part_data: p.part_data,
            price_credits: p.price_credits,
            is_default: p.is_default,
            owned: true,
        })
        .collect();

    Ok(Json(result))
}
