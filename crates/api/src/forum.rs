use axum::{extract::{Extension, Path, Query, State}, http::HeaderMap, routing::{get, post}, Json, Router};
use renzora_models::forum::*;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;
use crate::{error::ApiError, jwt, middleware, middleware::AuthUser, middleware::JwtSecret, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/threads", post(create_thread))
        .route("/threads/:slug/reply", post(create_reply))
        .route("/posts/:id/reactions", post(toggle_post_reaction))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/categories", get(list_categories))
        .route("/categories/:slug", get(get_category_threads))
        .route("/threads/:slug", get(get_thread))
        .route("/search", get(search_forum))
        .merge(protected)
}

async fn list_categories(State(state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let cats = ForumCategory::list(&state.db).await?;

    // Latest post per category in one query (no N+1)
    let last_posts = sqlx::query_as::<_, (Uuid, String, String, String, time::OffsetDateTime)>(
        "SELECT DISTINCT ON (t.category_id) t.category_id, t.title as thread_title, t.slug as thread_slug, u.username, p.created_at \
         FROM forum_posts p \
         JOIN forum_threads t ON t.id = p.thread_id \
         JOIN users u ON u.id = p.author_id \
         ORDER BY t.category_id, p.created_at DESC"
    ).fetch_all(&state.db).await?;

    let items: Vec<serde_json::Value> = cats.iter().map(|c| {
        let mut v = serde_json::to_value(c).unwrap_or_default();
        let last_post = last_posts.iter().find(|lp| lp.0 == c.id).map(|lp| serde_json::json!({
            "thread_title": lp.1,
            "thread_slug": lp.2,
            "username": lp.3,
            "created_at": lp.4.format(&Rfc3339).unwrap_or_default(),
        }));
        if let Some(lp) = last_post {
            v["last_post"] = lp;
        }
        v
    }).collect();

    Ok(Json(items))
}

#[derive(Deserialize)]
struct ForumSearchQuery { q: Option<String> }

async fn search_forum(
    State(state): State<AppState>,
    Query(params): Query<ForumSearchQuery>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let q = params.q.unwrap_or_default();
    if q.trim().is_empty() {
        return Ok(Json(vec![]));
    }
    let pattern = format!("%{}%", q);

    let rows = sqlx::query_as::<_, (Uuid, String, String, String, String, i32, time::OffsetDateTime, time::OffsetDateTime)>(
        "SELECT t.id, t.title, t.slug, c.slug as category_slug, u.username as author_name, t.post_count, t.last_post_at, t.created_at \
         FROM forum_threads t \
         JOIN forum_categories c ON c.id = t.category_id \
         JOIN users u ON u.id = t.author_id \
         WHERE t.title ILIKE $1 OR EXISTS (SELECT 1 FROM forum_posts p WHERE p.thread_id = t.id AND p.content ILIKE $1) \
         ORDER BY t.last_post_at DESC LIMIT 25"
    ).bind(&pattern).fetch_all(&state.db).await?;

    let items: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.0,
        "title": r.1,
        "slug": r.2,
        "category_slug": r.3,
        "author_name": r.4,
        "post_count": r.5,
        "last_post_at": r.6.format(&Rfc3339).unwrap_or_default(),
        "created_at": r.7.format(&Rfc3339).unwrap_or_default(),
    })).collect();
    Ok(Json(items))
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
struct ThreadResponse { thread: ForumThread, posts: Vec<serde_json::Value>, total_posts: i64 }

async fn get_thread(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<ThreadsQuery>,
    headers: HeaderMap,
    Extension(jwt_secret): Extension<JwtSecret>,
) -> Result<Json<ThreadResponse>, ApiError> {
    // Optional viewer identity for "reacted" flags
    let viewer_id = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub);

    let thread = ForumThread::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    ForumThread::increment_views(&state.db, thread.id).await?;
    let (posts, total) = ForumPost::list_for_thread(&state.db, thread.id, params.page.unwrap_or(1)).await?;

    // Aggregate reactions for all posts on this page in one query
    let post_ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();
    let reactions = fetch_reactions(&state, "forum_post_reactions", &post_ids, viewer_id).await?;

    let posts: Vec<serde_json::Value> = posts.iter().map(|p| {
        let mut v = serde_json::to_value(p).unwrap_or_default();
        v["reactions"] = serde_json::json!(reactions_for_post(&reactions, p.id));
        v
    }).collect();

    Ok(Json(ThreadResponse { thread, posts, total_posts: total }))
}

/// One row of the grouped reaction aggregation: (post_id, icon, count, reacted-by-viewer).
type ReactionRow = (Uuid, String, i64, bool);

/// Fetch aggregated reactions for a set of posts from `post_reactions` or
/// `forum_post_reactions` in a single grouped query.
pub(crate) async fn fetch_reactions(
    state: &AppState,
    table: &str,
    post_ids: &[Uuid],
    viewer_id: Option<Uuid>,
) -> Result<Vec<ReactionRow>, ApiError> {
    if post_ids.is_empty() {
        return Ok(vec![]);
    }
    let query = format!(
        "SELECT post_id, icon, COUNT(*)::bigint as count, BOOL_OR(user_id = $2) as reacted \
         FROM {table} WHERE post_id = ANY($1) GROUP BY post_id, icon ORDER BY MIN(created_at)"
    );
    let rows = sqlx::query_as::<_, ReactionRow>(&query)
        .bind(post_ids)
        .bind(viewer_id.unwrap_or(Uuid::nil()))
        .fetch_all(&state.db)
        .await?;
    Ok(rows)
}

/// Build the serialized "reactions" array for one post from the grouped rows.
pub(crate) fn reactions_for_post(rows: &[ReactionRow], post_id: Uuid) -> Vec<serde_json::Value> {
    rows.iter().filter(|r| r.0 == post_id).map(|r| serde_json::json!({
        "icon": r.1,
        "count": r.2,
        "reacted": r.3,
    })).collect()
}

#[derive(Deserialize)]
pub(crate) struct ReactionBody { pub icon: String }

async fn toggle_post_reaction(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReactionBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.icon.is_empty() || body.icon.len() > 40 {
        return Err(ApiError::Validation("Icon must be 1-40 characters".into()));
    }
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM forum_posts WHERE id = $1")
        .bind(id).fetch_optional(&state.db).await?;
    if exists.is_none() {
        return Err(ApiError::NotFound);
    }

    let deleted = sqlx::query("DELETE FROM forum_post_reactions WHERE post_id = $1 AND user_id = $2 AND icon = $3")
        .bind(id).bind(auth.user_id).bind(&body.icon).execute(&state.db).await?;
    if deleted.rows_affected() > 0 {
        return Ok(Json(serde_json::json!({"reacted": false})));
    }
    sqlx::query("INSERT INTO forum_post_reactions (post_id, user_id, icon) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
        .bind(id).bind(auth.user_id).bind(&body.icon).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"reacted": true})))
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
        let replier = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?;
        let user = replier.as_ref().map(|u| u.username.as_str()).unwrap_or_default();
        let _ = crate::notify::notify(&state, thread.author_id, "reply",
            &format!("{user} replied to your thread"),
            &format!("New reply in: {}", thread.title),
            Some(&format!("/forum/thread/{}", thread.slug)),
            replier.as_ref().and_then(|u| u.avatar_url.as_deref()),
        ).await;
    }

    // Broadcast new post to everyone viewing the thread
    state.ws_broadcast.broadcast("new_post", serde_json::json!({
        "thread_slug": slug,
        "post_id": post.id,
    }));

    Ok(Json(post))
}
