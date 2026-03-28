use axum::{
    extract::{Extension, Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use renzora_models::api_token::ApiToken;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tokens))
        .route("/", post(create_token))
        .route("/:id", delete(revoke_token))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

#[derive(Debug, Deserialize)]
struct CreateTokenRequest {
    name: String,
    /// Optional expiry in days. None = never expires.
    expires_in_days: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateTokenResponse {
    id: Uuid,
    name: String,
    /// The raw token — only shown once at creation time.
    token: String,
    prefix: String,
    scopes: Vec<String>,
    expires_at: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct TokenListItem {
    id: Uuid,
    name: String,
    prefix: String,
    scopes: Vec<String>,
    last_used_at: Option<String>,
    expires_at: Option<String>,
    created_at: String,
}

/// Generate a random API token with the "rz_" prefix.
fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    format!("rz_{hex}")
}

async fn create_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, ApiError> {
    if body.name.is_empty() || body.name.len() > 128 {
        return Err(ApiError::Validation("Token name must be 1-128 characters".into()));
    }

    // Check token limit per user
    let existing = ApiToken::list_by_user(&state.db, auth.user_id).await?;
    if existing.len() >= 10 {
        return Err(ApiError::Validation("Maximum 10 API tokens per user".into()));
    }

    let raw_token = generate_token();
    let token_hash = middleware::hash_api_token(&raw_token);
    let prefix = &raw_token[..7]; // "rz_xxxx"

    let expires_at = body.expires_in_days.map(|days| {
        time::OffsetDateTime::now_utc() + time::Duration::days(days)
    });

    let scopes = vec!["marketplace:write".to_string()];

    let token = ApiToken::create(
        &state.db,
        auth.user_id,
        &body.name,
        &token_hash,
        prefix,
        &scopes,
        expires_at,
    )
    .await?;

    Ok(Json(CreateTokenResponse {
        id: token.id,
        name: token.name,
        token: raw_token,
        prefix: token.prefix,
        scopes: token.scopes,
        expires_at: token.expires_at.map(|t| t.to_string()),
        created_at: token.created_at.to_string(),
    }))
}

async fn list_tokens(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<TokenListItem>>, ApiError> {
    let tokens = ApiToken::list_by_user(&state.db, auth.user_id).await?;

    Ok(Json(
        tokens
            .into_iter()
            .map(|t| TokenListItem {
                id: t.id,
                name: t.name,
                prefix: t.prefix,
                scopes: t.scopes,
                last_used_at: t.last_used_at.map(|t| t.to_string()),
                expires_at: t.expires_at.map(|t| t.to_string()),
                created_at: t.created_at.to_string(),
            })
            .collect(),
    ))
}

async fn revoke_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = ApiToken::delete(&state.db, id, auth.user_id).await?;
    if !deleted {
        return Err(ApiError::NotFound);
    }
    Ok(Json(serde_json::json!({ "deleted": true })))
}
