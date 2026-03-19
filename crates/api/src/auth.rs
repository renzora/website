use axum::{
    extract::{Extension, State},
    routing::{get, post},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::user::User;
use serde::Deserialize;

use crate::{error::ApiError, jwt, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(me).put(update_me))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/forgot", post(forgot_password))
        .merge(protected)
}

async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    if body.username.len() < 3 || body.username.len() > 32 {
        return Err(ApiError::Validation(
            "Username must be 3-32 characters".into(),
        ));
    }
    if !body.email.contains('@') {
        return Err(ApiError::Validation("Invalid email".into()));
    }
    if body.password.len() < 8 {
        return Err(ApiError::Validation(
            "Password must be at least 8 characters".into(),
        ));
    }

    if User::find_by_email(&state.db, &body.email)
        .await?
        .is_some()
    {
        return Err(ApiError::UserAlreadyExists);
    }

    let user = User::create(&state.db, &body.username, &body.email, &body.password).await?;

    let access_token =
        jwt::create_access_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh_token =
        jwt::create_refresh_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: user_to_profile(&user),
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let user = User::find_by_email(&state.db, &body.email)
        .await?
        .ok_or(ApiError::InvalidCredentials)?;

    if !user.verify_password(&body.password) {
        return Err(ApiError::InvalidCredentials);
    }

    let access_token =
        jwt::create_access_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh_token =
        jwt::create_refresh_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: user_to_profile(&user),
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let claims = jwt::validate_token(&body.refresh_token, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    if claims.token_type != "refresh" {
        return Err(ApiError::Unauthorized);
    }

    let user = User::find_by_id(&state.db, claims.sub)
        .await?
        .ok_or(ApiError::NotFound)?;

    let access_token =
        jwt::create_access_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh_token =
        jwt::create_refresh_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: user_to_profile(&user),
    }))
}

async fn forgot_password(
    State(_state): State<AppState>,
    Json(_body): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    // Always return success to prevent email enumeration
    Ok(Json(MessageResponse {
        message: "If an account with that email exists, a reset link has been sent.".into(),
    }))
}

async fn me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserProfile>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user_to_profile(&user)))
}

#[derive(Deserialize)]
struct UpdateProfileRequest {
    username: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    gender: Option<String>,
    website: Option<String>,
    profile_color: Option<String>,
    banner_color: Option<String>,
}

async fn update_me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfile>, ApiError> {
    sqlx::query(
        "UPDATE users SET username=COALESCE($2,username), email=COALESCE($3,email), bio=COALESCE($4,bio), location=COALESCE($5,location), gender=COALESCE($6,gender), website=COALESCE($7,website), profile_color=COALESCE($8,profile_color), banner_color=COALESCE($9,banner_color), updated_at=NOW() WHERE id=$1"
    )
    .bind(auth.user_id)
    .bind(body.username.as_deref())
    .bind(body.email.as_deref())
    .bind(body.bio.as_deref())
    .bind(body.location.as_deref())
    .bind(body.gender.as_deref())
    .bind(body.website.as_deref())
    .bind(body.profile_color.as_deref())
    .bind(body.banner_color.as_deref())
    .execute(&state.db)
    .await?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user_to_profile(&user)))
}

fn user_to_profile(user: &User) -> UserProfile {
    UserProfile {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        credit_balance: user.credit_balance,
    }
}
