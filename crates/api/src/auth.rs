use axum::{
    extract::{Extension, Query, State},
    response::Redirect,
    routing::{get, post, delete},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::user::User;
use serde::Deserialize;

use crate::{error::ApiError, jwt, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(me).put(update_me))
        .route("/discord/link", get(discord_link))
        .route("/discord/callback", get(discord_callback))
        .route("/discord/unlink", delete(discord_unlink))
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

    // Resolve referral code to referrer user ID
    let referred_by = if let Some(ref code) = body.referral_code {
        let code = code.trim();
        if !code.is_empty() {
            User::find_by_referral_code(&state.db, code)
                .await?
                .map(|u| u.id)
        } else {
            None
        }
    } else {
        None
    };

    let user = User::create_with_referral(
        &state.db,
        &body.username,
        &body.email,
        &body.password,
        referred_by,
    )
    .await?;

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

// ── Discord OAuth2 ──

/// Returns the Discord OAuth2 authorization URL.
/// The frontend/launcher should redirect the user to this URL.
async fn discord_link(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let client_id = std::env::var("DISCORD_CLIENT_ID")
        .map_err(|_| ApiError::Internal("DISCORD_CLIENT_ID not configured".into()))?;
    let redirect_uri = std::env::var("DISCORD_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/discord/callback", state.site_url));

    // Encode the user's JWT into the state param so we can identify them on callback
    let token = jwt::create_access_token(auth.user_id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Simple percent-encoding for URL params
    let encode = |s: &str| -> String {
        s.bytes().map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                String::from(b as char)
            }
            _ => format!("%{:02X}", b),
        }).collect()
    };

    let url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify&state={}",
        client_id,
        encode(&redirect_uri),
        encode(&token),
    );

    Ok(Json(serde_json::json!({ "url": url })))
}

#[derive(Deserialize)]
struct DiscordCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct DiscordTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    avatar: Option<String>,
}

/// Discord OAuth2 callback — exchanges the code for a token, fetches the Discord user,
/// and links it to the authenticated Renzora user.
async fn discord_callback(
    State(state): State<AppState>,
    Query(params): Query<DiscordCallbackQuery>,
) -> Result<Redirect, ApiError> {
    let client_id = std::env::var("DISCORD_CLIENT_ID")
        .map_err(|_| ApiError::Internal("DISCORD_CLIENT_ID not configured".into()))?;
    let client_secret = std::env::var("DISCORD_CLIENT_SECRET")
        .map_err(|_| ApiError::Internal("DISCORD_CLIENT_SECRET not configured".into()))?;
    let redirect_uri = std::env::var("DISCORD_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/discord/callback", state.site_url));

    // Validate the state param to get the user ID
    let claims = jwt::validate_token(&params.state, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    // Exchange code for Discord access token
    let http = reqwest::Client::new();
    let token_resp = http
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Discord token exchange failed: {}", e)))?;

    if !token_resp.status().is_success() {
        return Err(ApiError::Internal("Discord token exchange failed".into()));
    }

    let token_data: DiscordTokenResponse = token_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Discord token: {}", e)))?;

    // Fetch Discord user info
    let user_resp = http
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token_data.access_token))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch Discord user: {}", e)))?;

    let discord_user: DiscordUser = user_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Discord user: {}", e)))?;

    // Build avatar URL
    let avatar_url = discord_user.avatar.as_ref().map(|hash| {
        format!("https://cdn.discordapp.com/avatars/{}/{}.png", discord_user.id, hash)
    });

    // Check if this Discord account is already linked to another user
    if let Some(existing) = User::find_by_discord_id(&state.db, &discord_user.id).await? {
        if existing.id != claims.sub {
            return Err(ApiError::Validation("This Discord account is already linked to another user".into()));
        }
    }

    // Link Discord to user
    User::link_discord(
        &state.db,
        claims.sub,
        &discord_user.id,
        &discord_user.username,
        avatar_url.as_deref(),
    )
    .await?;

    // Redirect back to the site settings page
    Ok(Redirect::to(&format!("{}/settings?discord=linked", state.site_url)))
}

/// Unlink Discord from the authenticated user's account.
async fn discord_unlink(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<MessageResponse>, ApiError> {
    User::unlink_discord(&state.db, auth.user_id).await?;
    Ok(Json(MessageResponse {
        message: "Discord unlinked".into(),
    }))
}

fn user_to_profile(user: &User) -> UserProfile {
    UserProfile {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        credit_balance: user.credit_balance,
        discord_username: user.discord_username.clone(),
        discord_avatar: user.discord_avatar.clone(),
    }
}
