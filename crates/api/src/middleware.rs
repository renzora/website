use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
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

/// Middleware that extracts and validates the Bearer token from the Authorization header.
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
