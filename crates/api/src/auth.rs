use axum::{
    extract::{Extension, Query, State},
    response::Redirect,
    routing::{get, post, delete},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::user::User;
use serde::Deserialize;
use totp_rs::{Algorithm, TOTP, Secret};

use crate::{error::ApiError, jwt, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(me).put(update_me))
        .route("/discord/link", get(discord_link))
        .route("/discord/unlink", delete(discord_unlink))
        .route("/twitch/link", get(twitch_link))
        .route("/github/link", get(github_link))
        .route("/steam/link", get(steam_link))
        .route("/2fa/setup", post(totp_setup))
        .route("/2fa/verify", post(totp_verify_setup))
        .route("/2fa/disable", post(totp_disable))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/login/2fa", post(login_2fa))
        .route("/refresh", post(refresh))
        .route("/forgot", post(forgot_password))
        .route("/discord/callback", get(discord_callback))
        .route("/twitch/callback", get(twitch_callback))
        .route("/github/callback", get(github_callback))
        .route("/steam/callback", get(steam_callback))
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
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_email(&state.db, &body.email)
        .await?
        .ok_or(ApiError::InvalidCredentials)?;

    if !user.verify_password(&body.password) {
        return Err(ApiError::InvalidCredentials);
    }

    // If 2FA is enabled, return a temporary token instead of full auth
    if user.totp_enabled {
        let temp_token = jwt::create_access_token(user.id, &state.jwt_secret)
            .map_err(|e| ApiError::Internal(e.to_string()))?;
        return Ok(Json(serde_json::json!({
            "requires_2fa": true,
            "temp_token": temp_token,
        })));
    }

    // If role requires 2FA but not set up yet, flag it but still allow login
    let needs_2fa_setup = user.role_requires_2fa() && !user.totp_enabled;

    let access_token =
        jwt::create_access_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh_token =
        jwt::create_refresh_token(user.id, &state.jwt_secret).map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "user": user_to_profile(&user),
        "needs_2fa_setup": needs_2fa_setup,
    })))
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

// ── Shared helpers ──

/// Simple percent-encoding for URL parameters.
fn percent_encode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                String::from(b as char)
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
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

    let url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify&state={}",
        client_id,
        percent_encode(&redirect_uri),
        percent_encode(&token),
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

    // Assign Discord role if user has an active subscription
    crate::discord::on_discord_link(&state.db, claims.sub, &discord_user.id).await;

    // Redirect back to the site settings page
    Ok(Redirect::to(&format!("{}/settings?discord=linked", state.site_url)))
}

/// Unlink Discord from the authenticated user's account.
async fn discord_unlink(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<MessageResponse>, ApiError> {
    // Get Discord ID before unlinking so we can remove roles
    let user = User::find_by_id(&state.db, auth.user_id).await?
        .ok_or(ApiError::Internal("User not found".into()))?;
    if let Some(discord_id) = &user.discord_id {
        crate::discord::on_discord_unlink(discord_id).await;
    }

    User::unlink_discord(&state.db, auth.user_id).await?;
    Ok(Json(MessageResponse {
        message: "Discord unlinked".into(),
    }))
}

// ── Twitch OAuth2 ──

/// Returns the Twitch OAuth2 authorization URL.
async fn twitch_link(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let client_id = std::env::var("TWITCH_CLIENT_ID")
        .map_err(|_| ApiError::Internal("TWITCH_CLIENT_ID not configured".into()))?;
    let redirect_uri = std::env::var("TWITCH_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/twitch/callback", state.site_url));

    let token = jwt::create_access_token(auth.user_id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!(
        "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=user:read:email&state={}",
        client_id,
        percent_encode(&redirect_uri),
        percent_encode(&token),
    );

    Ok(Json(serde_json::json!({ "url": url })))
}

#[derive(Deserialize)]
struct TwitchCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct TwitchTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct TwitchUsersResponse {
    data: Vec<TwitchUser>,
}

#[derive(Deserialize)]
struct TwitchUser {
    id: String,
    login: String,
    display_name: String,
    profile_image_url: Option<String>,
}

/// Twitch OAuth2 callback — exchanges code for token, fetches user, links account.
async fn twitch_callback(
    State(state): State<AppState>,
    Query(params): Query<TwitchCallbackQuery>,
) -> Result<Redirect, ApiError> {
    let client_id = std::env::var("TWITCH_CLIENT_ID")
        .map_err(|_| ApiError::Internal("TWITCH_CLIENT_ID not configured".into()))?;
    let client_secret = std::env::var("TWITCH_CLIENT_SECRET")
        .map_err(|_| ApiError::Internal("TWITCH_CLIENT_SECRET not configured".into()))?;
    let redirect_uri = std::env::var("TWITCH_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/twitch/callback", state.site_url));

    let claims = jwt::validate_token(&params.state, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    // Exchange code for Twitch access token
    let http = reqwest::Client::new();
    let token_resp = http
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Twitch token exchange failed: {}", e)))?;

    if !token_resp.status().is_success() {
        return Err(ApiError::Internal("Twitch token exchange failed".into()));
    }

    let token_data: TwitchTokenResponse = token_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Twitch token: {}", e)))?;

    // Fetch Twitch user info
    let user_resp = http
        .get("https://api.twitch.tv/helix/users")
        .header("Authorization", format!("Bearer {}", token_data.access_token))
        .header("Client-Id", &client_id)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch Twitch user: {}", e)))?;

    let twitch_users: TwitchUsersResponse = user_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Twitch user: {}", e)))?;

    let twitch_user = twitch_users.data.into_iter().next()
        .ok_or_else(|| ApiError::Internal("No Twitch user returned".into()))?;

    renzora_models::social_connection::SocialConnection::upsert(
        &state.db,
        claims.sub,
        "twitch",
        &twitch_user.display_name,
        twitch_user.profile_image_url.as_deref(),
        Some(&twitch_user.id),
        true,
    )
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to link Twitch: {}", e)))?;

    Ok(Redirect::to(&format!("{}/settings?twitch=linked", state.site_url)))
}

// ── GitHub OAuth2 ──

/// Returns the GitHub OAuth2 authorization URL.
async fn github_link(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let client_id = std::env::var("GITHUB_CLIENT_ID")
        .map_err(|_| ApiError::Internal("GITHUB_CLIENT_ID not configured".into()))?;
    let redirect_uri = std::env::var("GITHUB_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/github/callback", state.site_url));

    let token = jwt::create_access_token(auth.user_id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user&state={}",
        client_id,
        percent_encode(&redirect_uri),
        percent_encode(&token),
    );

    Ok(Json(serde_json::json!({ "url": url })))
}

#[derive(Deserialize)]
struct GitHubCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    avatar_url: Option<String>,
}

/// GitHub OAuth2 callback — exchanges code for token, fetches user, links account.
async fn github_callback(
    State(state): State<AppState>,
    Query(params): Query<GitHubCallbackQuery>,
) -> Result<Redirect, ApiError> {
    let client_id = std::env::var("GITHUB_CLIENT_ID")
        .map_err(|_| ApiError::Internal("GITHUB_CLIENT_ID not configured".into()))?;
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET")
        .map_err(|_| ApiError::Internal("GITHUB_CLIENT_SECRET not configured".into()))?;
    let redirect_uri = std::env::var("GITHUB_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/github/callback", state.site_url));

    let claims = jwt::validate_token(&params.state, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    // Exchange code for GitHub access token
    let http = reqwest::Client::new();
    let token_resp = http
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", &params.code),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("GitHub token exchange failed: {}", e)))?;

    if !token_resp.status().is_success() {
        return Err(ApiError::Internal("GitHub token exchange failed".into()));
    }

    let token_data: GitHubTokenResponse = token_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse GitHub token: {}", e)))?;

    // Fetch GitHub user info
    let user_resp = http
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token_data.access_token))
        .header("User-Agent", "renzora")
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch GitHub user: {}", e)))?;

    let github_user: GitHubUser = user_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse GitHub user: {}", e)))?;

    renzora_models::social_connection::SocialConnection::upsert(
        &state.db,
        claims.sub,
        "github",
        &github_user.login,
        github_user.avatar_url.as_deref(),
        Some(&github_user.id.to_string()),
        true,
    )
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to link GitHub: {}", e)))?;

    Ok(Redirect::to(&format!("{}/settings?github=linked", state.site_url)))
}

// ── Steam OpenID ──

/// Returns the Steam OpenID authorization URL.
async fn steam_link(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let redirect_uri = std::env::var("STEAM_REDIRECT_URI")
        .unwrap_or_else(|_| format!("{}/api/auth/steam/callback", state.site_url));

    let token = jwt::create_access_token(auth.user_id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!(
        "https://steamcommunity.com/openid/login\
         ?openid.mode=checkid_setup\
         &openid.ns=http://specs.openid.net/auth/2.0\
         &openid.return_to={}?state={}\
         &openid.realm={}\
         &openid.identity=http://specs.openid.net/auth/2.0/identifier_select\
         &openid.claimed_id=http://specs.openid.net/auth/2.0/identifier_select",
        percent_encode(&redirect_uri),
        percent_encode(&token),
        percent_encode(&state.site_url),
    );

    Ok(Json(serde_json::json!({ "url": url })))
}

#[derive(Deserialize)]
struct SteamCallbackQuery {
    state: String,
    #[serde(rename = "openid.claimed_id")]
    openid_claimed_id: String,
}

#[derive(Deserialize)]
struct SteamPlayerSummariesResponse {
    response: SteamPlayersWrapper,
}

#[derive(Deserialize)]
struct SteamPlayersWrapper {
    players: Vec<SteamPlayer>,
}

#[derive(Deserialize)]
struct SteamPlayer {
    steamid: String,
    personaname: String,
    avatarfull: Option<String>,
}

/// Steam OpenID callback — extracts Steam ID from claimed_id, fetches profile, links account.
async fn steam_callback(
    State(state): State<AppState>,
    Query(params): Query<SteamCallbackQuery>,
) -> Result<Redirect, ApiError> {
    let api_key = std::env::var("STEAM_API_KEY")
        .map_err(|_| ApiError::Internal("STEAM_API_KEY not configured".into()))?;

    let claims = jwt::validate_token(&params.state, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    // Extract Steam ID from claimed_id URL
    // Format: https://steamcommunity.com/openid/id/STEAMID64
    let steam_id = params
        .openid_claimed_id
        .rsplit('/')
        .next()
        .ok_or_else(|| ApiError::Internal("Invalid Steam claimed_id".into()))?
        .to_string();

    // Fetch Steam player summary
    let http = reqwest::Client::new();
    let user_resp = http
        .get(format!(
            "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&steamids={}",
            api_key, steam_id
        ))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch Steam user: {}", e)))?;

    let summary: SteamPlayerSummariesResponse = user_resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Steam user: {}", e)))?;

    let player = summary.response.players.into_iter().next()
        .ok_or_else(|| ApiError::Internal("No Steam player returned".into()))?;

    renzora_models::social_connection::SocialConnection::upsert(
        &state.db,
        claims.sub,
        "steam",
        &player.personaname,
        player.avatarfull.as_deref(),
        Some(&player.steamid),
        true,
    )
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to link Steam: {}", e)))?;

    Ok(Redirect::to(&format!("{}/settings?steam=linked", state.site_url)))
}

// ── Two-Factor Authentication ──

#[derive(Deserialize)]
struct TotpVerifyRequest {
    code: String,
}

#[derive(Deserialize)]
struct Login2faRequest {
    temp_token: String,
    code: String,
}

/// POST /auth/login/2fa — Complete login with TOTP code
async fn login_2fa(
    State(state): State<AppState>,
    Json(body): Json<Login2faRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate temp token to get user
    let claims = jwt::validate_token(&body.temp_token, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    let user = User::find_by_id(&state.db, claims.sub)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !user.totp_enabled {
        return Err(ApiError::Validation("2FA is not enabled".into()));
    }

    let secret = user.totp_secret.as_ref().ok_or(ApiError::Internal("No TOTP secret".into()))?;

    // Try TOTP code first
    let totp = build_totp(secret, &user.email)?;
    let valid = totp.check_current(&body.code).unwrap_or(false);

    // If TOTP fails, try backup code
    if !valid {
        let used = User::use_backup_code(&state.db, user.id, &body.code).await?;
        if !used {
            return Err(ApiError::Validation("Invalid 2FA code".into()));
        }
    }

    let access_token = jwt::create_access_token(user.id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh_token = jwt::create_refresh_token(user.id, &state.jwt_secret)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "user": user_to_profile(&user),
    })))
}

/// POST /auth/2fa/setup — Generate TOTP secret and QR code URL
async fn totp_setup(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if user.totp_enabled {
        return Err(ApiError::Validation("2FA is already enabled".into()));
    }

    // Generate secret
    let secret = Secret::generate_secret();
    let secret_base32 = secret.to_encoded().to_string();

    // Save secret (not yet enabled)
    User::set_totp_secret(&state.db, auth.user_id, &secret_base32).await?;

    // Generate QR code URL
    let totp = build_totp(&secret_base32, &user.email)?;
    let qr_url = totp.get_url();

    Ok(Json(serde_json::json!({
        "secret": secret_base32,
        "qr_url": qr_url,
    })))
}

/// POST /auth/2fa/verify — Verify TOTP code to enable 2FA
async fn totp_verify_setup(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<TotpVerifyRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let secret = user.totp_secret.as_ref()
        .ok_or(ApiError::Validation("Run 2FA setup first".into()))?;

    let totp = build_totp(secret, &user.email)?;

    if !totp.check_current(&body.code).unwrap_or(false) {
        return Err(ApiError::Validation("Invalid code. Try again.".into()));
    }

    // Generate backup codes
    let backup_codes: Vec<String> = (0..8)
        .map(|_| {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            format!("{:08}", rng.gen_range(10000000u32..99999999u32))
        })
        .collect();

    User::enable_totp(&state.db, auth.user_id, &backup_codes).await?;

    Ok(Json(serde_json::json!({
        "enabled": true,
        "backup_codes": backup_codes,
    })))
}

/// POST /auth/2fa/disable — Disable 2FA (requires current TOTP code)
async fn totp_disable(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<TotpVerifyRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !user.totp_enabled {
        return Err(ApiError::Validation("2FA is not enabled".into()));
    }

    // Must verify current code to disable
    let secret = user.totp_secret.as_ref().ok_or(ApiError::Internal("No TOTP secret".into()))?;
    let totp = build_totp(secret, &user.email)?;

    if !totp.check_current(&body.code).unwrap_or(false) {
        return Err(ApiError::Validation("Invalid code".into()));
    }

    // Check if role requires 2FA — prevent disabling if enforced
    if user.role_requires_2fa() {
        return Err(ApiError::Validation("2FA is required for your role and cannot be disabled".into()));
    }

    User::disable_totp(&state.db, auth.user_id).await?;

    Ok(Json(MessageResponse { message: "2FA disabled".into() }))
}

fn build_totp(secret: &str, email: &str) -> Result<TOTP, ApiError> {
    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret.to_string()).to_bytes().map_err(|e| ApiError::Internal(format!("Invalid TOTP secret: {}", e)))?,
        Some("Renzora".to_string()),
        email.to_string(),
    )
    .map_err(|e| ApiError::Internal(format!("TOTP error: {}", e)))
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
        totp_enabled: user.totp_enabled,
    }
}
