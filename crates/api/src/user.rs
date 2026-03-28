use axum::{
    extract::{Extension, State},
    routing::{get, post},
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
