use axum::{
    extract::{Extension, Multipart, Path, State},
    http::HeaderMap,
    routing::{get, post, put},
    Json, Router,
};
use sqlx::Row;
use uuid::Uuid;

use crate::{error::ApiError, jwt, marketplace, middleware, middleware::AuthUser, middleware::JwtSecret, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/follow/:username", post(toggle_follow))
        .route("/avatar", put(upload_avatar))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/view/:username", get(get_profile))
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
    let row = sqlx::query("SELECT id, username, role, bio, website, location, gender, profile_color, banner_color, avatar_url, follower_count, following_count, post_count, credit_balance, created_at FROM users WHERE username=$1")
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

            let stored_name = format!("{}-{}", Uuid::new_v4(), filename);
            let s3_key = format!("avatars/{}", stored_name);
            avatar_url = Some(marketplace::upload_to_storage(&state, &s3_key, data.to_vec()).await?);
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
