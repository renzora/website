use axum::{
    extract::{Extension, State},
    routing::get,
    Json, Router,
};
use renzora_models::user::User;
use serde::Serialize;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_me))
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
    owned_asset_ids: Vec<Uuid>,
}

async fn user_me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserMeResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Get all owned asset IDs
    let owned: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT asset_id FROM asset_ownership WHERE user_id = $1"
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(UserMeResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        credit_balance: user.credit_balance,
        role: user.role,
        avatar_url: user.avatar_url,
        owned_asset_ids: owned.into_iter().map(|r| r.0).collect(),
    }))
}
