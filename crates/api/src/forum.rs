use axum::{extract::{Extension, Path, Query, State}, routing::{get, post}, Json, Router};
use renzora_models::forum::*;
use renzora_models::notification::Notification;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/threads", post(create_thread))
        .route("/threads/:slug}/reply", post(create_reply))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/categories", get(list_categories))
        .route("/categories/:slug", get(get_category_threads))
        .route("/threads/:slug", get(get_thread))
        .merge(protected)
}

async fn list_categories(State(state): State<AppState>) -> Result<Json<Vec<ForumCategory>>, ApiError> {
    let cats = ForumCategory::list(&state.db).await?;
    Ok(Json(cats))
}

#[derive(Deserialize)]
struct ThreadsQuery { page: Option<i64> }

#[derive(Serialize)]
struct CategoryThreadsResponse { category: ForumCategory, threads: Vec<ThreadWithAuthor>, total: i64 }

async fn get_category_threads(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<ThreadsQuery>,
) -> Result<Json<CategoryThreadsResponse>, ApiError> {
    let cat = ForumCategory::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    let (threads, total) = ForumThread::list_by_category(&state.db, cat.id, params.page.unwrap_or(1)).await?;
    Ok(Json(CategoryThreadsResponse { category: cat, threads, total }))
}

#[derive(Serialize)]
struct ThreadResponse { thread: ForumThread, posts: Vec<PostWithAuthor>, total_posts: i64 }

async fn get_thread(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<ThreadsQuery>,
) -> Result<Json<ThreadResponse>, ApiError> {
    let thread = ForumThread::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    ForumThread::increment_views(&state.db, thread.id).await?;
    let (posts, total) = ForumPost::list_for_thread(&state.db, thread.id, params.page.unwrap_or(1)).await?;
    Ok(Json(ThreadResponse { thread, posts, total_posts: total }))
}

#[derive(Deserialize)]
struct CreateThreadBody { category_slug: String, title: String, content: String }

async fn create_thread(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateThreadBody>,
) -> Result<Json<ForumThread>, ApiError> {
    // Check ban
    if let Some(ban) = renzora_models::role::is_banned(&state.db, auth.user_id).await? {
        return Err(ApiError::Validation(format!("You are banned: {}", ban.reason)));
    }
    if body.title.is_empty() || body.title.len() > 255 { return Err(ApiError::Validation("Title must be 1-255 chars".into())); }
    if body.content.is_empty() { return Err(ApiError::Validation("Content required".into())); }
    let cat = ForumCategory::find_by_slug(&state.db, &body.category_slug).await?.ok_or(ApiError::NotFound)?;
    let (thread, _post) = ForumThread::create(&state.db, cat.id, auth.user_id, &body.title, &body.content).await?;

    // Award XP for forum post
    let _ = renzora_models::xp::award_xp(&state.db, auth.user_id, renzora_models::xp::XP_FORUM_POST, "forum_thread", Some(thread.id)).await;

    // Broadcast new thread to all connected clients
    state.ws_broadcast.broadcast("new_thread", serde_json::json!({
        "thread_id": thread.id,
        "title": thread.title,
        "slug": thread.slug,
        "category_slug": body.category_slug,
    }));

    Ok(Json(thread))
}

#[derive(Deserialize)]
struct ReplyBody { content: String }

async fn create_reply(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(slug): Path<String>,
    Json(body): Json<ReplyBody>,
) -> Result<Json<ForumPost>, ApiError> {
    if let Some(ban) = renzora_models::role::is_banned(&state.db, auth.user_id).await? {
        return Err(ApiError::Validation(format!("You are banned: {}", ban.reason)));
    }
    if body.content.is_empty() { return Err(ApiError::Validation("Content required".into())); }
    let thread = ForumThread::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if thread.locked { return Err(ApiError::Validation("Thread is locked".into())); }
    let post = ForumPost::create_reply(&state.db, thread.id, auth.user_id, &body.content).await?;

    // Award XP for reply
    let _ = renzora_models::xp::award_xp(&state.db, auth.user_id, renzora_models::xp::XP_FORUM_POST, "forum_reply", Some(post.id)).await;

    // Notify thread author if someone else replied
    if thread.author_id != auth.user_id {
        let user = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.map(|u| u.username).unwrap_or_default();
        let _ = Notification::create(&state.db, thread.author_id, "reply",
            &format!("{user} replied to your thread"),
            &format!("New reply in: {}", thread.title),
            Some(&format!("/forum/thread/{}", thread.slug)),
        ).await;

        // Live notification to thread author
        state.ws_broadcast.send_to_user(thread.author_id, "notification", serde_json::json!({
            "title": format!("{user} replied to your thread"),
            "link": format!("/forum/thread/{}", thread.slug),
        }));
    }

    // Broadcast new post to everyone viewing the thread
    state.ws_broadcast.broadcast("new_post", serde_json::json!({
        "thread_slug": slug,
        "post_id": post.id,
    }));

    Ok(Json(post))
}
