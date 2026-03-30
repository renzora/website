//! Game Services API — achievements, leaderboards, stats, friends.
//!
//! App developers authenticate with `rza_` prefixed tokens.
//! User data is only accessible when the user has granted the app the required scope.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use renzora_models::achievement::{AppAchievement, PlayerAchievement};
use renzora_models::developer_app::{AppUserGrant, DeveloperApp, AppToken, VALID_SCOPES};
use renzora_models::friend::Friend;
use renzora_models::leaderboard::{Leaderboard, LeaderboardEntry};
use renzora_models::player_stats::PlayerStat;
use renzora_models::user::User;

use crate::{error::ApiError, middleware, middleware::{AuthUser, AppAuth}, AppState};

pub fn router() -> Router<AppState> {
    // ── App management (owner, uses JWT or rz_ token) ──
    let app_mgmt = Router::new()
        .route("/apps", get(list_my_apps))
        .route("/apps", post(register_app))
        .route("/apps/:app_id", delete(delete_app))
        .route("/apps/:app_id/tokens", get(list_app_tokens))
        .route("/apps/:app_id/tokens", post(create_app_token))
        .route("/apps/:app_id/tokens/:token_id", delete(revoke_app_token))
        // Achievement definitions (app owner)
        .route("/apps/:app_id/achievements", get(list_achievements))
        .route("/apps/:app_id/achievements", post(create_achievement))
        .route("/apps/:app_id/achievements/:ach_id", put(update_achievement))
        .route("/apps/:app_id/achievements/:ach_id", delete(delete_achievement))
        // Leaderboard definitions (app owner)
        .route("/apps/:app_id/leaderboards", get(list_leaderboards))
        .route("/apps/:app_id/leaderboards", post(create_leaderboard))
        .route("/apps/:app_id/leaderboards/:lb_id", delete(delete_leaderboard))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    // ── User-facing: grant/revoke permissions ──
    let user_grants = Router::new()
        .route("/grants", get(list_my_grants))
        .route("/grants", post(grant_permissions))
        .route("/grants/:app_id", delete(revoke_grant))
        // Friends (user's own, any auth)
        .route("/friends", get(list_friends))
        .route("/friends/requests", get(list_friend_requests))
        .route("/friends/add", post(send_friend_request))
        .route("/friends/accept", post(accept_friend_request))
        .route("/friends/remove", post(remove_friend))
        .route("/friends/block", post(block_user))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    // ── Game client API (uses rza_ app token, checks user grants) ──
    let game_api = Router::new()
        // Player achievements
        .route("/player/:user_id/achievements", get(get_player_achievements))
        .route("/player/:user_id/achievements/unlock", post(unlock_achievement))
        // Player stats
        .route("/player/:user_id/stats", get(get_player_stats))
        .route("/player/:user_id/stats", post(set_player_stat))
        .route("/player/:user_id/stats/increment", post(increment_player_stat))
        // Leaderboards
        .route("/leaderboard/:key/scores", get(get_leaderboard_scores))
        .route("/leaderboard/:key/submit", post(submit_score))
        // Player profile
        .route("/player/:user_id/profile", get(get_player_profile))
        .route("/player/:user_id/friends", get(get_player_friends))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    // Public
    let public = Router::new()
        .route("/scopes", get(list_scopes))
        .route("/apps/:app_id/info", get(get_app_info));

    Router::new()
        .merge(app_mgmt)
        .merge(user_grants)
        .merge(game_api)
        .merge(public)
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Check that the request has AppAuth with the required scope, AND the target user
/// has granted that scope to this app.
async fn require_scope(
    state: &AppState,
    app_auth: &AppAuth,
    user_id: Uuid,
    scope: &str,
) -> Result<(), ApiError> {
    // Check token has the scope
    if !app_auth.scopes.iter().any(|s| s == scope) {
        return Err(ApiError::Validation(format!("Token missing required scope: {scope}")));
    }
    // Check user has granted the scope to this app
    let grant = AppUserGrant::find(&state.db, app_auth.app_id, user_id).await?;
    match grant {
        Some(g) if g.has_scope(scope) => Ok(()),
        _ => Err(ApiError::Validation(format!("User has not granted '{scope}' to this app"))),
    }
}

fn generate_client_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    format!("rz_app_{}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

fn generate_client_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    format!("rza_{}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

fn generate_app_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    format!("rza_{}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

// ═══════════════════════════════════════════════════════════════════════════
// Public
// ═══════════════════════════════════════════════════════════════════════════

async fn list_scopes() -> Json<Vec<serde_json::Value>> {
    let scopes: Vec<serde_json::Value> = VALID_SCOPES.iter().map(|s| {
        let desc = match *s {
            "profile:read" => "Read username and avatar",
            "friends:read" => "Read friend list",
            "friends:write" => "Send and accept friend requests",
            "achievements:read" => "Read player achievements",
            "achievements:write" => "Unlock achievements for players",
            "stats:read" => "Read player stats",
            "stats:write" => "Update player stats",
            "leaderboards:read" => "Read leaderboard scores",
            "leaderboards:write" => "Submit leaderboard scores",
            "inventory:read" => "Read purchased assets and games",
            _ => "",
        };
        serde_json::json!({ "scope": s, "description": desc })
    }).collect();
    Json(scopes)
}

async fn get_app_info(
    State(state): State<AppState>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(serde_json::json!({
        "id": app.id,
        "name": app.name,
        "slug": app.slug,
        "description": app.description,
        "website_url": app.website_url,
        "icon_url": app.icon_url,
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// App Management
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct RegisterAppBody {
    name: String,
    description: String,
    website_url: String,
    #[serde(default)]
    redirect_uri: String,
}

async fn register_app(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<RegisterAppBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::Validation("App name must be 1-128 characters".into()));
    }

    let existing = DeveloperApp::list_by_owner(&state.db, auth.user_id).await?;
    if existing.len() >= 10 {
        return Err(ApiError::Validation("Maximum 10 apps per developer".into()));
    }

    let client_id = generate_client_id();
    let client_secret = generate_client_secret();
    let secret_hash = middleware::hash_api_token(&client_secret);

    let app = DeveloperApp::create(
        &state.db,
        auth.user_id,
        name,
        body.description.trim(),
        body.website_url.trim(),
        body.redirect_uri.trim(),
        &client_id,
        &secret_hash,
    ).await?;

    Ok(Json(serde_json::json!({
        "id": app.id,
        "name": app.name,
        "slug": app.slug,
        "client_id": app.client_id,
        "client_secret": client_secret,
        "message": "Save the client_secret — it will not be shown again.",
    })))
}

async fn list_my_apps(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let apps = DeveloperApp::list_by_owner(&state.db, auth.user_id).await?;
    let result: Vec<serde_json::Value> = apps.iter().map(|a| serde_json::json!({
        "id": a.id,
        "name": a.name,
        "slug": a.slug,
        "description": a.description,
        "website_url": a.website_url,
        "client_id": a.client_id,
        "icon_url": a.icon_url,
        "approved": a.approved,
        "created_at": a.created_at.to_string(),
    })).collect();
    Ok(Json(result))
}

async fn delete_app(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = DeveloperApp::delete(&state.db, app_id, auth.user_id).await?;
    if !deleted { return Err(ApiError::NotFound); }
    Ok(Json(serde_json::json!({ "deleted": true })))
}

// ── App Tokens ──

#[derive(Deserialize)]
struct CreateAppTokenBody {
    name: String,
    scopes: Vec<String>,
    #[serde(default)]
    expires_in_days: Option<i64>,
}

async fn create_app_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
    Json(body): Json<CreateAppTokenBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    // Validate scopes
    for scope in &body.scopes {
        if !VALID_SCOPES.contains(&scope.as_str()) {
            return Err(ApiError::Validation(format!("Invalid scope: '{scope}'. Valid: {}", VALID_SCOPES.join(", "))));
        }
    }
    if body.scopes.is_empty() {
        return Err(ApiError::Validation("At least one scope is required".into()));
    }

    let raw_token = generate_app_token();
    let token_hash = middleware::hash_api_token(&raw_token);
    let prefix = &raw_token[..8];

    let expires_at = body.expires_in_days.map(|d| time::OffsetDateTime::now_utc() + time::Duration::days(d));

    let token = AppToken::create(&state.db, app_id, &body.name, &token_hash, prefix, &body.scopes, expires_at).await?;

    Ok(Json(serde_json::json!({
        "id": token.id,
        "name": token.name,
        "token": raw_token,
        "prefix": token.prefix,
        "scopes": token.scopes,
        "expires_at": token.expires_at.map(|t| t.to_string()),
        "message": "Save this token — it will not be shown again.",
    })))
}

async fn list_app_tokens(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }

    let tokens = AppToken::list_by_app(&state.db, app_id).await?;
    let result: Vec<serde_json::Value> = tokens.iter().map(|t| serde_json::json!({
        "id": t.id,
        "name": t.name,
        "prefix": t.prefix,
        "scopes": t.scopes,
        "last_used_at": t.last_used_at.map(|t| t.to_string()),
        "expires_at": t.expires_at.map(|t| t.to_string()),
        "created_at": t.created_at.to_string(),
    })).collect();
    Ok(Json(result))
}

async fn revoke_app_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((app_id, token_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }
    AppToken::delete(&state.db, token_id, app_id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Achievement Definitions (app owner)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct CreateAchievementBody {
    key: String,
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    icon_url: Option<String>,
    #[serde(default)]
    points: i32,
    #[serde(default)]
    hidden: bool,
}

async fn create_achievement(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
    Json(body): Json<CreateAchievementBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }

    let ach = AppAchievement::create(&state.db, app_id, &body.key, &body.name, &body.description, body.icon_url.as_deref(), body.points, body.hidden).await?;
    Ok(Json(serde_json::json!({
        "id": ach.id, "key": ach.achievement_key, "name": ach.name,
        "description": ach.description, "points": ach.points, "hidden": ach.hidden,
    })))
}

async fn list_achievements(
    State(state): State<AppState>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let achs = AppAchievement::list_by_app(&state.db, app_id).await?;
    let result: Vec<serde_json::Value> = achs.iter().map(|a| serde_json::json!({
        "id": a.id, "key": a.achievement_key, "name": a.name, "description": a.description,
        "icon_url": a.icon_url, "points": a.points, "hidden": a.hidden,
    })).collect();
    Ok(Json(result))
}

#[derive(Deserialize)]
struct UpdateAchievementBody {
    name: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    points: Option<i32>,
    hidden: Option<bool>,
}

async fn update_achievement(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((app_id, ach_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateAchievementBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }
    let ach = AppAchievement::update(&state.db, ach_id, app_id, body.name.as_deref(), body.description.as_deref(), body.icon_url.as_deref(), body.points, body.hidden).await?;
    Ok(Json(serde_json::json!({ "id": ach.id, "name": ach.name })))
}

async fn delete_achievement(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((app_id, ach_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }
    AppAchievement::delete(&state.db, ach_id, app_id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Leaderboard Definitions (app owner)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct CreateLeaderboardBody {
    key: String,
    name: String,
    #[serde(default = "default_desc")]
    sort_order: String,
}
fn default_desc() -> String { "desc".into() }

async fn create_leaderboard(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
    Json(body): Json<CreateLeaderboardBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }
    if body.sort_order != "asc" && body.sort_order != "desc" {
        return Err(ApiError::Validation("sort_order must be 'asc' or 'desc'".into()));
    }
    let lb = Leaderboard::create(&state.db, app_id, &body.key, &body.name, &body.sort_order).await?;
    Ok(Json(serde_json::json!({ "id": lb.id, "key": lb.leaderboard_key, "name": lb.name, "sort_order": lb.sort_order })))
}

async fn list_leaderboards(
    State(state): State<AppState>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let lbs = Leaderboard::list_by_app(&state.db, app_id).await?;
    let result: Vec<serde_json::Value> = lbs.iter().map(|l| serde_json::json!({
        "id": l.id, "key": l.leaderboard_key, "name": l.name, "sort_order": l.sort_order,
    })).collect();
    Ok(Json(result))
}

async fn delete_leaderboard(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((app_id, lb_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let app = DeveloperApp::find_by_id(&state.db, app_id).await?.ok_or(ApiError::NotFound)?;
    if app.owner_id != auth.user_id { return Err(ApiError::Unauthorized); }
    Leaderboard::delete(&state.db, lb_id, app_id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

// ═══════════════════════════════════════════════════════════════════════════
// User Grants
// ═══════════════════════════════════════════════════════════════════════════

async fn list_my_grants(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let grants = AppUserGrant::list_by_user(&state.db, auth.user_id).await?;
    let mut result = Vec::new();
    for g in &grants {
        let app = DeveloperApp::find_by_id(&state.db, g.app_id).await?;
        if let Some(app) = app {
            result.push(serde_json::json!({
                "app_id": app.id,
                "app_name": app.name,
                "app_slug": app.slug,
                "app_icon_url": app.icon_url,
                "website_url": app.website_url,
                "scopes_granted": g.scopes_granted,
                "granted_at": g.granted_at.to_string(),
            }));
        }
    }
    Ok(Json(result))
}

#[derive(Deserialize)]
struct GrantBody {
    app_id: Uuid,
    scopes: Vec<String>,
}

async fn grant_permissions(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<GrantBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate app exists
    let _app = DeveloperApp::find_by_id(&state.db, body.app_id).await?.ok_or(ApiError::NotFound)?;

    // Validate scopes
    for scope in &body.scopes {
        if !VALID_SCOPES.contains(&scope.as_str()) {
            return Err(ApiError::Validation(format!("Invalid scope: '{scope}'")));
        }
    }

    let grant = AppUserGrant::grant(&state.db, body.app_id, auth.user_id, &body.scopes).await?;
    Ok(Json(serde_json::json!({
        "app_id": grant.app_id,
        "scopes_granted": grant.scopes_granted,
        "granted_at": grant.granted_at.to_string(),
    })))
}

async fn revoke_grant(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(app_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    AppUserGrant::revoke(&state.db, app_id, auth.user_id).await?;
    Ok(Json(serde_json::json!({ "revoked": true })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Friends (user's own)
// ═══════════════════════════════════════════════════════════════════════════

async fn list_friends(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let friends = Friend::list_friends(&state.db, auth.user_id).await?;
    let result: Vec<serde_json::Value> = friends.iter().map(|f| serde_json::json!({
        "user_id": f.friend_id,
        "username": f.friend_username,
        "avatar_url": f.friend_avatar_url,
        "since": f.created_at.to_string(),
    })).collect();
    Ok(Json(result))
}

async fn list_friend_requests(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let requests = Friend::list_incoming_requests(&state.db, auth.user_id).await?;
    let result: Vec<serde_json::Value> = requests.iter().map(|f| serde_json::json!({
        "from_user_id": f.user_id,
        "username": f.friend_username,
        "avatar_url": f.friend_avatar_url,
        "sent_at": f.created_at.to_string(),
    })).collect();
    Ok(Json(result))
}

#[derive(Deserialize)]
struct FriendBody { user_id: Uuid }

async fn send_friend_request(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<FriendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.user_id == auth.user_id {
        return Err(ApiError::Validation("Cannot friend yourself".into()));
    }
    let _user = User::find_by_id(&state.db, body.user_id).await?.ok_or(ApiError::NotFound)?;
    Friend::send_request(&state.db, auth.user_id, body.user_id).await?;
    Ok(Json(serde_json::json!({ "status": "pending" })))
}

async fn accept_friend_request(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<FriendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let accepted = Friend::accept(&state.db, auth.user_id, body.user_id).await?;
    if !accepted { return Err(ApiError::NotFound); }
    Ok(Json(serde_json::json!({ "status": "accepted" })))
}

async fn remove_friend(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<FriendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Friend::remove(&state.db, auth.user_id, body.user_id).await?;
    Ok(Json(serde_json::json!({ "removed": true })))
}

async fn block_user(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<FriendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.user_id == auth.user_id {
        return Err(ApiError::Validation("Cannot block yourself".into()));
    }
    Friend::block(&state.db, auth.user_id, body.user_id).await?;
    Ok(Json(serde_json::json!({ "blocked": true })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Game Client API (requires rza_ token + user grant)
// ═══════════════════════════════════════════════════════════════════════════

async fn get_player_profile(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, user_id, "profile:read").await?;
    let user = User::find_by_id(&state.db, user_id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(serde_json::json!({
        "id": user.id,
        "username": user.username,
        "avatar_url": user.avatar_url,
    })))
}

async fn get_player_friends(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    require_scope(&state, &app_auth, user_id, "friends:read").await?;
    let friends = Friend::list_friends(&state.db, user_id).await?;
    let result: Vec<serde_json::Value> = friends.iter().map(|f| serde_json::json!({
        "user_id": f.friend_id,
        "username": f.friend_username,
        "avatar_url": f.friend_avatar_url,
    })).collect();
    Ok(Json(result))
}

async fn get_player_achievements(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, user_id, "achievements:read").await?;

    let all = AppAchievement::list_by_app(&state.db, app_auth.app_id).await?;
    let unlocked = PlayerAchievement::list_for_player(&state.db, app_auth.app_id, user_id).await?;
    let unlocked_ids: std::collections::HashSet<Uuid> = unlocked.iter().map(|u| u.achievement_id).collect();

    let achievements: Vec<serde_json::Value> = all.iter().filter(|a| !a.hidden || unlocked_ids.contains(&a.id)).map(|a| {
        let is_unlocked = unlocked_ids.contains(&a.id);
        let unlocked_at = unlocked.iter().find(|u| u.achievement_id == a.id).map(|u| u.unlocked_at.to_string());
        serde_json::json!({
            "key": a.achievement_key, "name": a.name, "description": a.description,
            "icon_url": a.icon_url, "points": a.points, "unlocked": is_unlocked,
            "unlocked_at": unlocked_at,
        })
    }).collect();

    Ok(Json(serde_json::json!({
        "achievements": achievements,
        "total": all.len(),
        "unlocked": unlocked.len(),
    })))
}

#[derive(Deserialize)]
struct UnlockBody {
    achievement_key: String,
    #[serde(default)]
    metadata: serde_json::Value,
}

async fn unlock_achievement(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UnlockBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, user_id, "achievements:write").await?;

    let ach = AppAchievement::find_by_key(&state.db, app_auth.app_id, &body.achievement_key).await?
        .ok_or(ApiError::Validation(format!("Unknown achievement: '{}'", body.achievement_key)))?;

    let result = PlayerAchievement::unlock(&state.db, app_auth.app_id, user_id, ach.id, body.metadata).await?;

    Ok(Json(serde_json::json!({
        "achievement": ach.achievement_key,
        "name": ach.name,
        "points": ach.points,
        "newly_unlocked": result.is_some(),
    })))
}

async fn get_player_stats(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    require_scope(&state, &app_auth, user_id, "stats:read").await?;
    let stats = PlayerStat::list_for_player(&state.db, app_auth.app_id, user_id).await?;
    let result: Vec<serde_json::Value> = stats.iter().map(|s| serde_json::json!({
        "key": s.stat_key, "value_int": s.value_int, "value_float": s.value_float,
        "updated_at": s.updated_at.to_string(),
    })).collect();
    Ok(Json(result))
}

#[derive(Deserialize)]
struct SetStatBody {
    key: String,
    #[serde(default)]
    value_int: i64,
    #[serde(default)]
    value_float: f64,
}

async fn set_player_stat(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<SetStatBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, user_id, "stats:write").await?;
    let stat = PlayerStat::set(&state.db, app_auth.app_id, user_id, &body.key, body.value_int, body.value_float).await?;
    Ok(Json(serde_json::json!({
        "key": stat.stat_key, "value_int": stat.value_int, "value_float": stat.value_float,
    })))
}

#[derive(Deserialize)]
struct IncrementStatBody {
    key: String,
    #[serde(default = "default_one")]
    delta: i64,
}
fn default_one() -> i64 { 1 }

async fn increment_player_stat(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<IncrementStatBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, user_id, "stats:write").await?;
    let stat = PlayerStat::increment(&state.db, app_auth.app_id, user_id, &body.key, body.delta).await?;
    Ok(Json(serde_json::json!({
        "key": stat.stat_key, "value_int": stat.value_int,
    })))
}

#[derive(Deserialize)]
struct LeaderboardQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn get_leaderboard_scores(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(key): Path<String>,
    Query(params): Query<LeaderboardQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Leaderboard reads don't require user grant — they're public within the app
    let lb = Leaderboard::find_by_key(&state.db, app_auth.app_id, &key).await?
        .ok_or(ApiError::NotFound)?;

    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let entries = LeaderboardEntry::top(&state.db, lb.id, &lb.sort_order, limit, offset).await?;

    let scores: Vec<serde_json::Value> = entries.iter().enumerate().map(|(i, e)| serde_json::json!({
        "rank": offset + i as i64 + 1,
        "user_id": e.user_id,
        "username": e.username,
        "avatar_url": e.avatar_url,
        "score": e.score,
        "metadata": e.metadata,
        "submitted_at": e.submitted_at.to_string(),
    })).collect();

    Ok(Json(serde_json::json!({
        "leaderboard": lb.name,
        "sort_order": lb.sort_order,
        "entries": scores,
    })))
}

#[derive(Deserialize)]
struct SubmitScoreBody {
    user_id: Uuid,
    score: i64,
    #[serde(default)]
    metadata: serde_json::Value,
}

async fn submit_score(
    State(state): State<AppState>,
    Extension(app_auth): Extension<AppAuth>,
    Path(key): Path<String>,
    Json(body): Json<SubmitScoreBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_scope(&state, &app_auth, body.user_id, "leaderboards:write").await?;

    let lb = Leaderboard::find_by_key(&state.db, app_auth.app_id, &key).await?
        .ok_or(ApiError::NotFound)?;

    let entry = LeaderboardEntry::submit(&state.db, lb.id, body.user_id, body.score, body.metadata, &lb.sort_order).await?;

    Ok(Json(serde_json::json!({
        "leaderboard": lb.name,
        "score": entry.score,
        "submitted_at": entry.submitted_at.to_string(),
    })))
}
