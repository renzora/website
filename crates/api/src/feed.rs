use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{get, post, delete as delete_route},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use time::format_description::well_known::Rfc3339;

use crate::{error::ApiError, marketplace, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    // Public reads — viewable logged-out and indexable. optional_auth personalizes
    // (is_liked) when signed in but never rejects. These handlers ONLY ever return
    // visibility='public', non-hidden content.
    let public = Router::new()
        .route("/feed", get(get_feed))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id/comments", get(list_comments))
        .route("/users/:username/posts", get(user_posts))
        .route("/channels", get(list_channels))
        .layer(axum::middleware::from_fn(middleware::optional_auth));

    // Writes — require a signed-in user.
    let protected = Router::new()
        .route("/posts", post(create_post))
        .route("/posts/:id", delete_route(delete_post))
        .route("/posts/:id/like", post(toggle_like))
        .route("/posts/:id/report", post(report_post))
        .route("/posts/:id/request-review", post(request_review))
        .route("/posts/:id/comments", post(create_comment))
        .route("/posts/:id/reactions", post(toggle_reaction))
        .route("/comments/:id", delete_route(delete_comment))
        .route("/comments/:id/like", post(toggle_comment_like))
        .route("/channels/suggest", post(suggest_channel))
        .route("/upload", post(upload_media))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new().merge(public).merge(protected)
}

/// A post auto-hides once this many distinct users report it (tunable). No
/// downvotes feed this — reports only, so disagreement never hides a post.
const AUTO_HIDE_REPORTS: i64 = 4;

/// One row of the grouped reaction aggregation: (post_id, icon, count, reacted-by-viewer).
type ReactionRow = (Uuid, String, i64, bool);

/// Fetch aggregated reactions for a set of posts from `post_reactions` in a
/// single grouped query.
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

#[derive(Deserialize)]
struct FeedQuery {
    before: Option<Uuid>,
    limit: Option<i64>,
    /// Optional channel slug to filter the feed to one channel.
    channel: Option<String>,
}

async fn get_feed(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let viewer = auth.as_ref().map(|a| a.user_id);
    let limit = params.limit.unwrap_or(20).min(50);
    let channel_id = resolve_channel_id(&state, params.channel.as_deref(), false).await?;
    let posts = if let Some(cid) = channel_id {
        // A channel is a public topic space: everyone sees all public posts in it.
        renzora_models::post::Post::channel_public(&state.db, cid, limit, params.before, viewer).await?
    } else if let Some(uid) = viewer {
        // Signed-in home: personal following timeline (unchanged).
        renzora_models::post::Post::feed(&state.db, uid, limit, params.before, None).await?
    } else {
        // Logged-out home: recent public posts across channels.
        renzora_models::post::Post::public_recent(&state.db, limit, params.before, None).await?
    };
    let post_ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();
    let reactions = fetch_reactions(&state, "post_reactions", &post_ids, viewer).await?;
    let items: Vec<serde_json::Value> = posts.iter().map(|p| serialize_post(p, &reactions)).collect();
    Ok(Json(serde_json::json!(items)))
}

/// A single PUBLIC post by id — powers the indexable permalink page.
async fn get_post(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let viewer = auth.as_ref().map(|a| a.user_id);
    let post = renzora_models::post::Post::find_public_by_id(&state.db, id, viewer)
        .await?
        .ok_or(ApiError::NotFound)?;
    let reactions = fetch_reactions(&state, "post_reactions", &[post.id], viewer).await?;
    Ok(Json(serialize_post(&post, &reactions)))
}

#[derive(Deserialize)]
struct CreatePostBody {
    body: String,
    media_urls: Option<Vec<String>>,
    visibility: Option<String>,
    /// Optional channel slug to post into.
    channel: Option<String>,
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
    let channel_id = resolve_channel_id(&state, body.channel.as_deref(), true).await?;
    let post = renzora_models::post::Post::create(&state.db, auth.user_id, &body.body, &media, visibility, channel_id).await?;

    // Award XP for posting
    let _ = renzora_models::xp::award_xp(&state.db, auth.user_id, renzora_models::xp::XP_POST, "feed_post", Some(post.id)).await;

    // Broadcast to followers via WS
    let sender = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    state.ws_broadcast.broadcast("new_post", serde_json::json!({
        "post_id": post.id,
        "user_id": auth.user_id,
        "username": sender.username,
    }));

    // Notify @mentioned users
    let mut mention_names: Vec<String> = Vec::new();
    for seg in body.body.split('@').skip(1) {
        let name: String = seg.chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        if !name.is_empty() {
            let lower = name.to_lowercase();
            if !mention_names.contains(&lower) {
                mention_names.push(lower);
            }
        }
    }
    if !mention_names.is_empty() {
        let mentioned = sqlx::query_as::<_, (Uuid,)>(
            "SELECT id FROM users WHERE LOWER(username) = ANY($1)"
        ).bind(&mention_names).fetch_all(&state.db).await?;
        let snippet: String = body.body.chars().take(120).collect();
        for (uid,) in mentioned {
            if uid != auth.user_id {
                let _ = crate::notify::notify(&state, uid, "mention",
                    &format!("{} mentioned you", sender.username),
                    &snippet,
                    Some("/community"),
                    sender.avatar_url.as_deref(),
                ).await;
            }
        }
    }

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
    // Fetch first so we can permanently remove the attached media from storage
    // after the row is gone (deleting the row alone would orphan the files).
    let post = renzora_models::post::Post::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if post.user_id != auth.user_id {
        return Err(ApiError::NotFound);
    }
    let media = post.media_urls.clone();
    let deleted = renzora_models::post::Post::delete(&state.db, id, auth.user_id).await?;
    if !deleted { return Err(ApiError::NotFound); }
    for url in &media {
        let _ = marketplace::delete_from_storage(&state, url).await;
    }
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
    Extension(auth): Extension<Option<AuthUser>>,
    Path(post_id): Path<Uuid>,
    Query(params): Query<CommentQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let viewer = auth.as_ref().map(|a| a.user_id);
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let comments = renzora_models::post::PostComment::list_for_post(&state.db, post_id, viewer, limit, offset).await?;
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
    Extension(auth): Extension<Option<AuthUser>>,
    Path(username): Path<String>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let viewer = auth.as_ref().map(|a| a.user_id);
    let user = renzora_models::user::User::find_by_username(&state.db, &username).await?.ok_or(ApiError::NotFound)?;
    let limit = params.limit.unwrap_or(20).min(50);
    let posts = renzora_models::post::Post::list_by_user(&state.db, user.id, viewer, limit).await?;
    let post_ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();
    let reactions = fetch_reactions(&state, "post_reactions", &post_ids, viewer).await?;
    let items: Vec<serde_json::Value> = posts.iter().map(|p| serialize_post(p, &reactions)).collect();
    Ok(Json(serde_json::json!(items)))
}

async fn toggle_reaction(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReactionBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.icon.is_empty() || body.icon.len() > 40 {
        return Err(ApiError::Validation("Icon must be 1-40 characters".into()));
    }
    renzora_models::post::Post::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    let deleted = sqlx::query("DELETE FROM post_reactions WHERE post_id = $1 AND user_id = $2 AND icon = $3")
        .bind(id).bind(auth.user_id).bind(&body.icon).execute(&state.db).await?;
    if deleted.rows_affected() > 0 {
        return Ok(Json(serde_json::json!({"reacted": false})));
    }
    sqlx::query("INSERT INTO post_reactions (post_id, user_id, icon) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
        .bind(id).bind(auth.user_id).bind(&body.icon).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"reacted": true})))
}

/// Upload an image for a feed post. Mirrors the avatar upload in profiles.rs
/// but stores under the feed/ prefix.
async fn upload_media(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut image_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::Validation(format!("Failed to read upload: {e}"))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "image" {
            let filename = field.file_name().unwrap_or("image.png").to_string();
            let ext = filename.rsplit('.').next().map(|e| e.to_lowercase()).unwrap_or_default();
            if !["png", "jpg", "jpeg", "webp", "gif"].contains(&ext.as_str()) {
                return Err(ApiError::Validation("Image must be png, jpg, webp, or gif".into()));
            }
            let data = field.bytes().await.map_err(|e| {
                ApiError::Validation(format!("Failed to read file: {e}"))
            })?;

            // Max 5MB for feed images
            if data.len() > 5 * 1024 * 1024 {
                return Err(ApiError::Validation("Image must be under 5MB".into()));
            }

            image_url = Some(marketplace::upload_to_storage(&state, "feed", &filename, data.to_vec()).await?);
        }
    }

    let url = image_url.ok_or(ApiError::Validation("No image file provided".into()))?;
    Ok(Json(serde_json::json!({ "url": url })))
}

// ── Channels ──────────────────────────────────────────────────────────────────

/// Resolve a channel slug to its id. `require_approved` = only match live
/// channels (used when posting); the feed filter matches any existing channel.
async fn resolve_channel_id(state: &AppState, slug: Option<&str>, require_approved: bool) -> Result<Option<Uuid>, ApiError> {
    let Some(slug) = slug.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    let q = if require_approved {
        "SELECT id FROM channels WHERE slug = $1 AND approved = true"
    } else {
        "SELECT id FROM channels WHERE slug = $1"
    };
    Ok(sqlx::query_scalar::<_, Uuid>(q).bind(slug).fetch_optional(&state.db).await?)
}

async fn list_channels(
    State(state): State<AppState>,
    Extension(_auth): Extension<Option<AuthUser>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query_as::<_, (Uuid, String, String, String, String, i32)>(
        "SELECT id, name, slug, description, icon, post_count FROM channels WHERE approved = true ORDER BY sort_order, name"
    ).fetch_all(&state.db).await?;
    let items: Vec<serde_json::Value> = rows.iter().map(|(id, name, slug, desc, icon, pc)| serde_json::json!({
        "id": id, "name": name, "slug": slug, "description": desc, "icon": icon, "post_count": pc,
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

#[derive(Deserialize)]
struct SuggestChannelBody {
    name: String,
    description: Option<String>,
    icon: Option<String>,
}

async fn suggest_channel(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SuggestChannelBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() || name.chars().count() > 48 {
        return Err(ApiError::Validation("Channel name must be 1-48 characters".into()));
    }
    let slug = slugify(name);
    if slug.is_empty() {
        return Err(ApiError::Validation("Channel name must contain letters or numbers".into()));
    }
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM channels WHERE slug = $1)")
        .bind(&slug).fetch_one(&state.db).await?;
    if exists {
        return Err(ApiError::Validation("That channel already exists or is already suggested".into()));
    }
    let desc: String = body.description.unwrap_or_default().chars().take(200).collect();
    let icon = body.icon.filter(|i| !i.trim().is_empty()).unwrap_or_else(|| "ph-hash".to_string());
    // Suggested channels start unapproved (approved=false); an admin approves them.
    sqlx::query(
        "INSERT INTO channels (name, slug, description, icon, sort_order, approved, suggested_by) VALUES ($1, $2, $3, $4, 100, false, $5)"
    ).bind(name).bind(&slug).bind(&desc).bind(&icon).bind(auth.user_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"ok": true, "slug": slug, "pending": true})))
}

/// name → URL-safe single-token slug (matches the `channels.slug` CHECK:
/// lowercase letters/digits/hyphens/underscores, no spaces).
fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !out.is_empty() && !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').chars().take(48).collect()
}

// ── Reporting / review ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ReportBody {
    reason: Option<String>,
}

async fn report_post(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReportBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let post = renzora_models::post::Post::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if post.user_id == auth.user_id {
        return Err(ApiError::Validation("You can't report your own post".into()));
    }
    let reason: String = body.reason.unwrap_or_default().chars().take(200).collect();
    sqlx::query("INSERT INTO post_reports (post_id, reporter_id, reason) VALUES ($1, $2, $3) ON CONFLICT (post_id, reporter_id) DO NOTHING")
        .bind(id).bind(auth.user_id).bind(&reason).execute(&state.db).await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM post_reports WHERE post_id = $1")
        .bind(id).fetch_one(&state.db).await?;
    let now_hidden = count >= AUTO_HIDE_REPORTS;
    // Auto-hide at the threshold; never un-hide here (only a moderator restores).
    sqlx::query(
        "UPDATE posts SET report_count = $2, hidden = (hidden OR $3), \
         hidden_at = CASE WHEN $3 AND hidden_at IS NULL THEN NOW() ELSE hidden_at END WHERE id = $1"
    ).bind(id).bind(count as i32).bind(now_hidden).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"reported": true, "hidden": now_hidden})))
}

async fn request_review(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let r = sqlx::query("UPDATE posts SET review_requested = true WHERE id = $1 AND user_id = $2 AND hidden = true")
        .bind(id).bind(auth.user_id).execute(&state.db).await?;
    if r.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

fn serialize_post(p: &renzora_models::post::PostWithAuthor, reactions: &[(Uuid, String, i64, bool)]) -> serde_json::Value {
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
        "channel_slug": p.channel_slug,
        "channel_name": p.channel_name,
        "channel_icon": p.channel_icon,
        "hidden": p.hidden,
        "review_requested": p.review_requested,
        "reactions": reactions_for_post(reactions, p.id),
        "created_at": p.created_at.format(&Rfc3339).unwrap_or_default(),
    })
}
