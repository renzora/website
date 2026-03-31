use axum::{
    extract::{Extension, Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use renzora_models::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_me))
        .route("/owned", post(check_owned))
        .route("/privacy", put(update_privacy))
        .route("/blocked", get(list_blocked))
        .route("/blocked/:user_id", delete(unblock_user))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

#[derive(Serialize)]
struct UserMeResponse {
    id: Uuid,
    username: String,
    email: String,
    credit_balance: i64,
    role: String,
    avatar_url: Option<String>,
}

async fn user_me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserMeResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(UserMeResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        credit_balance: user.credit_balance,
        role: user.role,
        avatar_url: user.avatar_url,
    }))
}

#[derive(Deserialize)]
struct CheckOwnedRequest {
    asset_ids: Vec<Uuid>,
}

#[derive(Serialize)]
struct CheckOwnedResponse {
    owned_ids: Vec<Uuid>,
}

/// Check which of the given asset IDs the user owns (via purchase or creation)
async fn check_owned(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CheckOwnedRequest>,
) -> Result<Json<CheckOwnedResponse>, ApiError> {
    if body.asset_ids.is_empty() {
        return Ok(Json(CheckOwnedResponse { owned_ids: vec![] }));
    }

    // Check purchased (user_assets) + created in one query, limited to the requested IDs
    let owned: Vec<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT id FROM (
            SELECT asset_id AS id FROM user_assets
            WHERE user_id = $1 AND asset_id = ANY($2)
            UNION
            SELECT id FROM assets
            WHERE creator_id = $1 AND id = ANY($2)
        ) sub
        "#
    )
    .bind(auth.user_id)
    .bind(&body.asset_ids)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(CheckOwnedResponse {
        owned_ids: owned.into_iter().map(|r| r.0).collect(),
    }))
}

#[derive(Deserialize)]
struct PrivacyBody {
    message_privacy: Option<String>,
    online_status_visible: Option<bool>,
    profile_visibility: Option<String>,
}

async fn update_privacy(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<PrivacyBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    let msg_priv = body.message_privacy.as_deref().unwrap_or(&user.message_privacy);
    let vis = body.profile_visibility.as_deref().unwrap_or(&user.profile_visibility);
    let online = body.online_status_visible.unwrap_or(user.online_status_visible);

    if !["everyone", "friends", "nobody"].contains(&msg_priv) {
        return Err(ApiError::Validation("message_privacy must be everyone, friends, or nobody".into()));
    }
    if !["public", "friends_only"].contains(&vis) {
        return Err(ApiError::Validation("profile_visibility must be public or friends_only".into()));
    }

    User::update_privacy(&state.db, auth.user_id, msg_priv, online, vis).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn list_blocked(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT f.friend_id, u.username, u.avatar_url FROM friends f JOIN users u ON u.id = f.friend_id WHERE f.user_id = $1 AND f.status = 'blocked' ORDER BY f.created_at DESC"
    ).bind(auth.user_id).fetch_all(&state.db).await?;

    let items: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "user_id": r.0,
        "username": r.1,
        "avatar_url": r.2,
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

async fn unblock_user(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("DELETE FROM friends WHERE user_id = $1 AND friend_id = $2 AND status = 'blocked'")
        .bind(auth.user_id).bind(user_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}
