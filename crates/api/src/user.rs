use axum::{
    extract::{Extension, State},
    routing::{get, post, put},
    Json, Router,
};
use renzora_models::user::User;
use renzora_models::xp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_me))
        .route("/summary", get(summary))
        .route("/owned", post(check_owned))
        .route("/communication", get(get_communication).put(update_communication))
        .route("/signature", put(update_signature))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// Consolidated nav bootstrap — credits, level/XP and creator status in ONE
/// request (replaces the separate calls the nav used to fire on every page
/// load).
#[derive(Serialize)]
struct SummaryResponse {
    credit_balance: i64,
    level: i32,
    level_progress_percent: f64,
    total_xp: i64,
    creator_policy_accepted: bool,
}

async fn summary(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<SummaryResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let cur = xp::xp_for_level(user.level);
    let next = xp::xp_for_level(user.level + 1);
    let range = next - cur;
    let level_progress_percent = if range > 0 {
        ((user.total_xp - cur) as f64 / range as f64 * 100.0).clamp(0.0, 100.0)
    } else {
        100.0
    };

    Ok(Json(SummaryResponse {
        credit_balance: user.credit_balance,
        level: user.level,
        level_progress_percent,
        total_xp: user.total_xp,
        creator_policy_accepted: user.creator_policy_accepted_at.is_some(),
    }))
}

#[derive(Serialize)]
struct UserMeResponse {
    id: Uuid,
    username: String,
    email: String,
    credit_balance: i64,
    role: String,
    avatar_url: Option<String>,
    banner_url: Option<String>,
    online_status_visible: bool,
    signature: String,
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
        banner_url: user.banner_url,
        online_status_visible: user.online_status_visible,
        signature: user.signature,
    }))
}

#[derive(Serialize)]
struct CommunicationResponse {
    product_updates: bool,
    marketplace: bool,
    comments: bool,
    security: bool,
}

async fn get_communication(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<CommunicationResponse>, ApiError> {
    let row: (bool, bool, bool, bool) = sqlx::query_as(
        "SELECT email_product_updates, email_marketplace, email_comments, email_security FROM users WHERE id = $1"
    )
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(CommunicationResponse {
        product_updates: row.0,
        marketplace: row.1,
        comments: row.2,
        security: row.3,
    }))
}

#[derive(Deserialize)]
struct CommunicationBody {
    product_updates: Option<bool>,
    marketplace: Option<bool>,
    comments: Option<bool>,
    security: Option<bool>,
}

async fn update_communication(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CommunicationBody>,
) -> Result<Json<CommunicationResponse>, ApiError> {
    let row: (bool, bool, bool, bool) = sqlx::query_as(
        "UPDATE users SET \
            email_product_updates = COALESCE($2, email_product_updates), \
            email_marketplace = COALESCE($3, email_marketplace), \
            email_comments = COALESCE($4, email_comments), \
            email_security = COALESCE($5, email_security), \
            updated_at = NOW() \
         WHERE id = $1 \
         RETURNING email_product_updates, email_marketplace, email_comments, email_security"
    )
    .bind(auth.user_id)
    .bind(body.product_updates)
    .bind(body.marketplace)
    .bind(body.comments)
    .bind(body.security)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(CommunicationResponse {
        product_updates: row.0,
        marketplace: row.1,
        comments: row.2,
        security: row.3,
    }))
}

#[derive(Deserialize)]
struct SignatureBody {
    signature: String,
}

async fn update_signature(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SignatureBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.signature.chars().count() > 300 {
        return Err(ApiError::Validation("Signature must be 300 characters or less".into()));
    }
    sqlx::query("UPDATE users SET signature = $1, updated_at = NOW() WHERE id = $2")
        .bind(&body.signature)
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({"ok": true})))
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
