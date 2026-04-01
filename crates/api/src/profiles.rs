use axum::{
    extract::{Extension, Multipart, Path, State},
    http::HeaderMap,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{error::ApiError, jwt, marketplace, middleware, middleware::AuthUser, middleware::JwtSecret, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/follow/:username", post(toggle_follow))
        .route("/friend/:username", post(toggle_friend))
        .route("/block/:username", post(block_user))
        .route("/avatar", put(upload_avatar))
        .route("/storefront", put(update_storefront))
        .route("/connections", get(list_connections))
        .route("/connections", post(add_connection))
        .route("/connections/:platform", delete(remove_connection))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/view/:username", get(get_profile))
        .route("/shop/:username", get(get_storefront))
        .route("/search", get(search_users))
        .merge(protected)
}

#[derive(serde::Deserialize)]
struct UserSearchQuery {
    q: Option<String>,
}

async fn search_users(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<UserSearchQuery>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let q = params.q.unwrap_or_default();
    if q.len() < 2 {
        return Ok(Json(vec![]));
    }
    let pattern = format!("%{}%", q);
    let rows = sqlx::query(
        "SELECT username, role, avatar_url FROM users WHERE username ILIKE $1 ORDER BY username LIMIT 8"
    )
    .bind(&pattern)
    .fetch_all(&state.db)
    .await?;

    let results: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "username": r.get::<String, _>("username"),
        "role": r.get::<String, _>("role"),
        "avatar_url": r.get::<Option<String>, _>("avatar_url"),
    })).collect();

    Ok(Json(results))
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
    headers: HeaderMap,
    Extension(jwt_secret): Extension<JwtSecret>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Try to extract viewer identity from optional Bearer token
    let viewer_id = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub);
    let row = sqlx::query("SELECT id, username, role, bio, website, location, gender, profile_color, banner_color, avatar_url, follower_count, following_count, post_count, credit_balance, created_at, storefront_enabled FROM users WHERE username=$1")
        .bind(&username)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    let id: Uuid = row.get("id");

    // Badges
    let badge_rows = sqlx::query("SELECT b.slug, b.name, b.description, b.icon, b.color FROM user_badges ub JOIN badges b ON b.id=ub.badge_id WHERE ub.user_id=$1")
        .bind(id)
        .fetch_all(&state.db)
        .await?;

    let badges: Vec<serde_json::Value> = badge_rows.iter().map(|r| serde_json::json!({
        "slug": r.get::<String, _>("slug"),
        "name": r.get::<String, _>("name"),
        "description": r.get::<String, _>("description"),
        "icon": r.get::<String, _>("icon"),
        "color": r.get::<String, _>("color"),
    })).collect();

    // Check if viewer follows this user
    let is_following = if let Some(vid) = viewer_id {
        let r: Option<(Uuid,)> = sqlx::query_as("SELECT follower_id FROM follows WHERE follower_id=$1 AND following_id=$2")
            .bind(vid).bind(id).fetch_optional(&state.db).await?;
        r.is_some()
    } else {
        false
    };

    // Social connections
    let connections = renzora_models::social_connection::SocialConnection::list_for_user(&state.db, id).await.unwrap_or_default();

    // Published assets by this user
    let asset_rows = sqlx::query(
        "SELECT id, name, slug, description, category, price_credits, thumbnail_url, version, downloads FROM assets WHERE creator_id=$1 AND published=true ORDER BY created_at DESC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let assets: Vec<serde_json::Value> = asset_rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "description": r.get::<String, _>("description"),
        "category": r.get::<String, _>("category"),
        "price_credits": r.get::<i64, _>("price_credits"),
        "thumbnail_url": r.get::<Option<String>, _>("thumbnail_url"),
        "version": r.get::<String, _>("version"),
        "downloads": r.get::<i64, _>("downloads"),
    })).collect();

    Ok(Json(serde_json::json!({
        "id": id,
        "username": row.get::<String, _>("username"),
        "role": row.get::<String, _>("role"),
        "bio": row.get::<String, _>("bio"),
        "website": row.get::<String, _>("website"),
        "location": row.get::<String, _>("location"),
        "gender": row.get::<String, _>("gender"),
        "profile_color": row.get::<String, _>("profile_color"),
        "banner_color": row.get::<String, _>("banner_color"),
        "avatar_url": row.get::<Option<String>, _>("avatar_url"),
        "follower_count": row.get::<i32, _>("follower_count"),
        "following_count": row.get::<i32, _>("following_count"),
        "post_count": row.get::<i32, _>("post_count"),
        "credit_balance": row.get::<i64, _>("credit_balance"),
        "is_following": is_following,
        "badges": badges,
        "assets": assets,
        "connections": connections,
        "storefront_enabled": row.get::<bool, _>("storefront_enabled"),
        "created_at": row.get::<time::OffsetDateTime, _>("created_at").to_string(),
    })))
}

/// Upload a profile avatar image to S3.
async fn upload_avatar(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut avatar_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::Validation(format!("Failed to read upload: {e}"))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "avatar" {
            let filename = field.file_name().unwrap_or("avatar.png").to_string();
            let data = field.bytes().await.map_err(|e| {
                ApiError::Validation(format!("Failed to read file: {e}"))
            })?;

            // Max 2MB for avatars
            if data.len() > 2 * 1024 * 1024 {
                return Err(ApiError::Validation("Avatar must be under 2MB".into()));
            }

            avatar_url = Some(marketplace::upload_to_storage(&state, "avatars", &filename, data.to_vec()).await?);
        }
    }

    let url = avatar_url.ok_or(ApiError::Validation("No avatar file provided".into()))?;

    sqlx::query("UPDATE users SET avatar_url = $1, updated_at = NOW() WHERE id = $2")
        .bind(&url)
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;

    Ok(Json(serde_json::json!({ "avatar_url": url })))
}

async fn toggle_follow(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let row: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE username=$1")
        .bind(&username).fetch_optional(&state.db).await?;
    let target_id = row.ok_or(ApiError::NotFound)?.0;

    if target_id == auth.user_id {
        return Err(ApiError::Validation("Cannot follow yourself".into()));
    }

    let following = renzora_models::profile::toggle_follow(&state.db, auth.user_id, target_id).await?;

    if following {
        let user = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.map(|u| u.username).unwrap_or_default();
        let _ = renzora_models::notification::Notification::create(&state.db, target_id, "follow",
            &format!("{user} started following you"), "", Some(&format!("/profile/{user}"))).await;
    }

    Ok(Json(serde_json::json!({"following": following})))
}

// ── Storefront ──

async fn get_storefront(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query(
        r#"SELECT id, username, role, bio, avatar_url, profile_color, banner_color,
            storefront_enabled, storefront_tagline, storefront_bg_color, storefront_bg_image,
            storefront_text_color, storefront_accent_color, storefront_card_bg, storefront_card_border,
            storefront_font, storefront_font_size, storefront_cursor, storefront_layout, storefront_css
           FROM users WHERE username=$1"#
    )
    .bind(&username)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    let id: Uuid = row.get("id");
    let enabled: bool = row.get("storefront_enabled");
    if !enabled {
        return Err(ApiError::NotFound);
    }

    // Published assets
    let asset_rows = sqlx::query(
        "SELECT id, name, slug, description, category, price_credits, thumbnail_url, version, downloads, views FROM assets WHERE creator_id=$1 AND published=true ORDER BY created_at DESC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let assets: Vec<serde_json::Value> = asset_rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "description": r.get::<String, _>("description"),
        "category": r.get::<String, _>("category"),
        "price_credits": r.get::<i64, _>("price_credits"),
        "thumbnail_url": r.get::<Option<String>, _>("thumbnail_url"),
        "version": r.get::<String, _>("version"),
        "downloads": r.get::<i64, _>("downloads"),
        "views": r.get::<i64, _>("views"),
    })).collect();

    // Published games
    let game_rows = sqlx::query(
        "SELECT id, name, slug, description, category, price_credits, thumbnail_url, version, downloads, views FROM games WHERE creator_id=$1 AND published=true ORDER BY created_at DESC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let games: Vec<serde_json::Value> = game_rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "description": r.get::<String, _>("description"),
        "category": r.get::<String, _>("category"),
        "price_credits": r.get::<i64, _>("price_credits"),
        "thumbnail_url": r.get::<Option<String>, _>("thumbnail_url"),
        "version": r.get::<String, _>("version"),
        "downloads": r.get::<i64, _>("downloads"),
        "views": r.get::<i64, _>("views"),
    })).collect();

    // Badges
    let badge_rows = sqlx::query("SELECT b.slug, b.name, b.description, b.icon, b.color FROM user_badges ub JOIN badges b ON b.id=ub.badge_id WHERE ub.user_id=$1")
        .bind(id).fetch_all(&state.db).await?;
    let badges: Vec<serde_json::Value> = badge_rows.iter().map(|r| serde_json::json!({
        "slug": r.get::<String, _>("slug"),
        "name": r.get::<String, _>("name"),
        "icon": r.get::<String, _>("icon"),
        "color": r.get::<String, _>("color"),
    })).collect();

    Ok(Json(serde_json::json!({
        "username": row.get::<String, _>("username"),
        "role": row.get::<String, _>("role"),
        "bio": row.get::<String, _>("bio"),
        "avatar_url": row.get::<Option<String>, _>("avatar_url"),
        "profile_color": row.get::<String, _>("profile_color"),
        "banner_color": row.get::<String, _>("banner_color"),
        "tagline": row.get::<String, _>("storefront_tagline"),
        "bg_color": row.get::<String, _>("storefront_bg_color"),
        "bg_image": row.get::<String, _>("storefront_bg_image"),
        "text_color": row.get::<String, _>("storefront_text_color"),
        "accent_color": row.get::<String, _>("storefront_accent_color"),
        "card_bg": row.get::<String, _>("storefront_card_bg"),
        "card_border": row.get::<String, _>("storefront_card_border"),
        "font": row.get::<String, _>("storefront_font"),
        "font_size": row.get::<String, _>("storefront_font_size"),
        "cursor": row.get::<String, _>("storefront_cursor"),
        "layout": row.get::<String, _>("storefront_layout"),
        "css": row.get::<String, _>("storefront_css"),
        "badges": badges,
        "assets": assets,
        "games": games,
    })))
}

#[derive(serde::Deserialize)]
struct UpdateStorefrontRequest {
    enabled: Option<bool>,
    tagline: Option<String>,
    bg_color: Option<String>,
    bg_image: Option<String>,
    text_color: Option<String>,
    accent_color: Option<String>,
    card_bg: Option<String>,
    card_border: Option<String>,
    font: Option<String>,
    font_size: Option<String>,
    cursor: Option<String>,
    layout: Option<String>,
    css: Option<String>,
}

async fn update_storefront(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<UpdateStorefrontRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Sanitize custom CSS: strip anything that could be used for XSS
    let safe_css = body.css.as_deref().map(|css| {
        css.replace("javascript:", "")
           .replace("expression(", "")
           .replace("url(", "url(")  // keep url() but could further restrict
           .chars()
           .filter(|c| *c != '<' && *c != '>')
           .collect::<String>()
    });

    // Limit custom CSS length
    if let Some(ref css) = safe_css {
        if css.len() > 4096 {
            return Err(ApiError::Validation("Custom CSS must be under 4KB".into()));
        }
    }

    sqlx::query(
        r#"UPDATE users SET
            storefront_enabled = COALESCE($2, storefront_enabled),
            storefront_tagline = COALESCE($3, storefront_tagline),
            storefront_bg_color = COALESCE($4, storefront_bg_color),
            storefront_bg_image = COALESCE($5, storefront_bg_image),
            storefront_text_color = COALESCE($6, storefront_text_color),
            storefront_accent_color = COALESCE($7, storefront_accent_color),
            storefront_card_bg = COALESCE($8, storefront_card_bg),
            storefront_card_border = COALESCE($9, storefront_card_border),
            storefront_font = COALESCE($10, storefront_font),
            storefront_font_size = COALESCE($11, storefront_font_size),
            storefront_cursor = COALESCE($12, storefront_cursor),
            storefront_layout = COALESCE($13, storefront_layout),
            storefront_css = COALESCE($14, storefront_css),
            updated_at = NOW()
        WHERE id = $1"#
    )
    .bind(auth.user_id)
    .bind(body.enabled)
    .bind(body.tagline.as_deref())
    .bind(body.bg_color.as_deref())
    .bind(body.bg_image.as_deref())
    .bind(body.text_color.as_deref())
    .bind(body.accent_color.as_deref())
    .bind(body.card_bg.as_deref())
    .bind(body.card_border.as_deref())
    .bind(body.font.as_deref())
    .bind(body.font_size.as_deref())
    .bind(body.cursor.as_deref())
    .bind(body.layout.as_deref())
    .bind(safe_css.as_deref())
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({"message": "Storefront updated"})))
}

async fn toggle_friend(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let target = renzora_models::user::User::find_by_username(&state.db, &username).await?.ok_or(ApiError::NotFound)?;
    if target.id == auth.user_id {
        return Err(ApiError::Validation("Cannot friend yourself".into()));
    }

    // Check current status
    let status = renzora_models::friend::Friend::status(&state.db, auth.user_id, target.id).await?;

    match status.as_deref() {
        Some("accepted") => {
            // Remove friend
            renzora_models::friend::Friend::remove(&state.db, auth.user_id, target.id).await?;
            Ok(Json(serde_json::json!({"status": "none"})))
        }
        Some("pending") => {
            // Already pending - could be incoming or outgoing
            // Check if we sent it or they sent it
            let incoming = renzora_models::friend::Friend::status(&state.db, target.id, auth.user_id).await?;
            if incoming.as_deref() == Some("pending") {
                // They sent us a request, accept it
                renzora_models::friend::Friend::accept(&state.db, auth.user_id, target.id).await?;
                // Notify them
                renzora_models::notification::Notification::create(
                    &state.db, target.id, "friend_accepted",
                    "Friend request accepted",
                    &format!("{} accepted your friend request", username),
                    Some(&format!("/profile/{}", username)),
                ).await?;
                Ok(Json(serde_json::json!({"status": "accepted"})))
            } else {
                // We already sent a request, cancel it
                renzora_models::friend::Friend::remove(&state.db, auth.user_id, target.id).await?;
                Ok(Json(serde_json::json!({"status": "none"})))
            }
        }
        Some("blocked") => {
            Err(ApiError::Validation("User is blocked".into()))
        }
        _ => {
            // Send friend request
            renzora_models::friend::Friend::send_request(&state.db, auth.user_id, target.id).await?;
            // Notify target
            let sender = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
            renzora_models::notification::Notification::create(
                &state.db, target.id, "friend_request",
                "Friend request",
                &format!("{} sent you a friend request", sender.username),
                Some(&format!("/profile/{}", sender.username)),
            ).await?;
            state.ws_broadcast.send_to_user(target.id, "notification", serde_json::json!({"type": "friend_request"}));
            Ok(Json(serde_json::json!({"status": "pending"})))
        }
    }
}

async fn block_user(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let target = renzora_models::user::User::find_by_username(&state.db, &username).await?.ok_or(ApiError::NotFound)?;
    if target.id == auth.user_id {
        return Err(ApiError::Validation("Cannot block yourself".into()));
    }
    renzora_models::friend::Friend::block(&state.db, auth.user_id, target.id).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Social Connections ──

async fn list_connections(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let conns = renzora_models::social_connection::SocialConnection::list_for_user(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!(conns)))
}

#[derive(Deserialize)]
struct AddConnectionBody {
    platform: String,
    username: String,
    url: Option<String>,
}

async fn add_connection(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<AddConnectionBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let valid = ["discord", "twitch", "steam", "xbox", "playstation", "epic", "kick", "youtube", "twitter", "github"];
    if !valid.contains(&body.platform.as_str()) {
        return Err(ApiError::Validation("Invalid platform".into()));
    }
    let conn = renzora_models::social_connection::SocialConnection::upsert(
        &state.db, auth.user_id, &body.platform, &body.username, body.url.as_deref(), None, false
    ).await?;
    Ok(Json(serde_json::json!(conn)))
}

async fn remove_connection(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(platform): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    renzora_models::social_connection::SocialConnection::delete(&state.db, auth.user_id, &platform).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}
