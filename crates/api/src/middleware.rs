use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::jwt;

/// Extension inserted by auth middleware containing the authenticated user's ID.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Extension that carries the JWT secret into middleware. Inserted by the server on startup.
#[derive(Clone)]
pub struct JwtSecret(pub String);

/// Extension that carries the DB pool for API token lookups.
#[derive(Clone)]
pub struct DbPool(pub PgPool);

/// Middleware that extracts and validates the Bearer token from the Authorization header.
/// Supports both JWT tokens and API tokens (prefixed with "rz_").
pub async fn require_auth(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let jwt_secret = req
        .extensions()
        .get::<JwtSecret>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .clone();

    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // API token path: tokens prefixed with "rz_"
    if token.starts_with("rz_") {
        let db = req
            .extensions()
            .get::<DbPool>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .clone();

        let token_hash = hash_api_token(token);
        let api_token = renzora_models::api_token::ApiToken::find_by_hash(&db.0, &token_hash)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check expiry
        if let Some(expires) = api_token.expires_at {
            if expires < time::OffsetDateTime::now_utc() {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }

        // Check daily rate limit
        let (count, limit) = renzora_models::subscription::check_and_increment_usage(&db.0, api_token.user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if count > limit {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        // Update last used (fire and forget)
        let _ = renzora_models::api_token::ApiToken::touch_last_used(&db.0, api_token.id).await;

        req.extensions_mut().insert(AuthUser {
            user_id: api_token.user_id,
        });

        return Ok(next.run(req).await);
    }

    // JWT path
    let claims = jwt::validate_token(token, &jwt_secret.0)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.token_type != "access" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(AuthUser {
        user_id: claims.sub,
    });

    Ok(next.run(req).await)
}

/// Hash an API token using SHA-256 for storage comparison.
pub fn hash_api_token(token: &str) -> String {
    use sha2::{Sha256, Digest};
    use std::fmt::Write;
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in result {
        let _ = write!(hex, "{:02x}", byte);
    }
    hex
}
