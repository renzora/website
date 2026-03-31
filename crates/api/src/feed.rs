use axum::{
    extract::{Extension, Path, Query, State},
    routing::{get, post, delete as delete_route},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use time::format_description::well_known::Rfc3339;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/feed", get(get_feed))
        .route("/posts", post(create_post))
        .route("/posts/:id", delete_route(delete_post))
        .route("/posts/:id/like", post(toggle_like))
        .route("/posts/:id/comments", get(list_comments))
        .route("/posts/:id/comments", post(create_comment))
        .route("/comments/:id", delete_route(delete_comment))
        .route("/comments/:id/like", post(toggle_comment_like))
        .route("/users/:username/posts", get(user_posts))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .merge(protected)
}

#[derive(Deserialize)]
struct FeedQuery {
    before: Option<Uuid>,
    limit: Option<i64>,
}

async fn get_feed(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let limit = params.limit.unwrap_or(20).min(50);
    let posts = renzora_models::post::Post::feed(&state.db, auth.user_id, limit, params.before).await?;
    let items: Vec<serde_json::Value> = posts.iter().map(|p| serialize_post(p)).collect();
    Ok(Json(serde_json::json!(items)))
}

#[derive(Deserialize)]
struct CreatePostBody {
    body: String,
    media_urls: Option<Vec<String>>,
    visibility: Option<String>,
}

async fn create_post(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreatePostBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.body.trim().is_empty() && body.media_urls.as_ref().map_or(true, |m| m.is_empty()) {
        return Err(ApiError::Validation("Post must have text or media".into()));
    }
    if body.body.len() > 5000 {
        return Err(ApiError::Validation("Post too long (max 5000 chars)".into()));
    }
    let visibility = body.visibility.as_deref().unwrap_or("public");
    if !["public", "followers", "friends"].contains(&visibility) {
        return Err(ApiError::Validation("Invalid visibility".into()));
    }
    let media = body.media_urls.unwrap_or_default();
    let post = renzora_models::post::Post::create(&state.db, auth.user_id, &body.body, &media, visibility).await?;

    // Broadcast to followers via WS
    let sender = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    state.ws_broadcast.broadcast("new_post", serde_json::json!({
        "post_id": post.id,
        "user_id": auth.user_id,
        "username": sender.username,
    }));

    Ok(Json(serde_json::json!({
        "id": post.id,
        "created_at": post.created_at.format(&Rfc3339).unwrap_or_default(),
    })))
}

async fn delete_post(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = renzora_models::post::Post::delete(&state.db, id, auth.user_id).await?;
    if !deleted { return Err(ApiError::NotFound); }
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn toggle_like(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let liked = renzora_models::post::Post::toggle_like(&state.db, id, auth.user_id).await?;

    // Notify post author
    if liked {
        let post = renzora_models::post::Post::find_by_id(&state.db, id).await?;
        if let Some(p) = post {
            if p.user_id != auth.user_id {
                state.ws_broadcast.send_to_user(p.user_id, "post_liked", serde_json::json!({
                    "post_id": id,
                    "user_id": auth.user_id,
                }));
            }
        }
    }

    Ok(Json(serde_json::json!({"liked": liked})))
}

#[derive(Deserialize)]
struct CommentQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_comments(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
    Query(params): Query<CommentQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let comments = renzora_models::post::PostComment::list_for_post(&state.db, post_id, Some(auth.user_id), limit, offset).await?;
    let items: Vec<serde_json::Value> = comments.iter().map(|c| serde_json::json!({
        "id": c.id,
        "post_id": c.post_id,
        "user_id": c.user_id,
        "username": c.username,
        "avatar_url": c.avatar_url,
        "body": c.body,
        "parent_id": c.parent_id,
        "like_count": c.like_count,
        "is_liked": c.is_liked,
        "created_at": c.created_at.format(&Rfc3339).unwrap_or_default(),
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

#[derive(Deserialize)]
struct CreateCommentBody {
    body: String,
    parent_id: Option<Uuid>,
}

async fn create_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
    Json(body): Json<CreateCommentBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.body.trim().is_empty() {
        return Err(ApiError::Validation("Comment cannot be empty".into()));
    }
    let comment = renzora_models::post::PostComment::create(&state.db, post_id, auth.user_id, &body.body, body.parent_id).await?;

    // Notify post author
    let post = renzora_models::post::Post::find_by_id(&state.db, post_id).await?;
    if let Some(p) = post {
        if p.user_id != auth.user_id {
            let sender = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?;
            if let Some(s) = sender {
                state.ws_broadcast.send_to_user(p.user_id, "new_comment", serde_json::json!({
                    "post_id": post_id,
                    "comment_id": comment.id,
                    "user_id": auth.user_id,
                    "username": s.username,
                }));
            }
        }
    }

    Ok(Json(serde_json::json!({
        "id": comment.id,
        "created_at": comment.created_at.format(&Rfc3339).unwrap_or_default(),
    })))
}

async fn delete_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = renzora_models::post::PostComment::delete(&state.db, id, auth.user_id).await?;
    if !deleted { return Err(ApiError::NotFound); }
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn toggle_comment_like(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let liked = renzora_models::post::PostComment::toggle_like(&state.db, id, auth.user_id).await?;
    Ok(Json(serde_json::json!({"liked": liked})))
}

async fn user_posts(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(username): Path<String>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = renzora_models::user::User::find_by_username(&state.db, &username).await?.ok_or(ApiError::NotFound)?;
    let limit = params.limit.unwrap_or(20).min(50);
    let posts = renzora_models::post::Post::list_by_user(&state.db, user.id, Some(auth.user_id), limit).await?;
    let items: Vec<serde_json::Value> = posts.iter().map(|p| serialize_post(p)).collect();
    Ok(Json(serde_json::json!(items)))
}

fn serialize_post(p: &renzora_models::post::PostWithAuthor) -> serde_json::Value {
    serde_json::json!({
        "id": p.id,
        "user_id": p.user_id,
        "username": p.username,
        "avatar_url": p.avatar_url,
        "role": p.role,
        "body": p.body,
        "media_urls": p.media_urls,
        "visibility": p.visibility,
        "like_count": p.like_count,
        "comment_count": p.comment_count,
        "is_liked": p.is_liked,
        "created_at": p.created_at.format(&Rfc3339).unwrap_or_default(),
    })
}
