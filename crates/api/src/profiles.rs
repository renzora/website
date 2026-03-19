use axum::{extract::{Extension, Path, State}, routing::{get, post}, Json, Router};
use sqlx::Row;
use uuid::Uuid;
use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/follow/:username", post(toggle_follow))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/view/:username", get(get_profile))
        .merge(protected)
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query("SELECT id, username, role, bio, website, location, gender, profile_color, banner_color, avatar_url, follower_count, following_count, post_count, credit_balance, created_at FROM users WHERE username=$1")
        .bind(&username)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    let id: Uuid = row.get("id");

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
        "is_following": false,
        "badges": badges,
        "created_at": row.get::<time::OffsetDateTime, _>("created_at").to_string(),
    })))
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
