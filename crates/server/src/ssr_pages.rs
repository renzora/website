//! Server-rendered content pages (SEO). Each handler fetches its data from the
//! DB, injects it into Leptos context, and renders the app so the page
//! component can emit crawlable HTML + per-page meta. Only content pages that
//! benefit from SEO are wired here; app/auth pages keep the generic SSR handler.

use axum::{body::Body, http::Request, response::Response};
use leptos::prelude::provide_context;
use sqlx::PgPool;

use renzora_common::ssr::{
    ArticleSsr, AssetSsr, CommunitySsr, CourseSsr, DocSsr, PostSsr, ProfileSsr, SsrComment,
    SsrPostItem, SsrReaction,
};
use renzora_models::article::Article;
use renzora_models::asset::Asset;
use renzora_models::course::Course;
use renzora_models::post::{Post, PostComment, PostWithAuthor};
use renzora_models::user::User;
use renzora_web::shell::Shell;
use uuid::Uuid;

/// Short relative time, mirroring the client's `timeAgo` so the SSR card header
/// stays a single line (the full ISO timestamp wraps it to two, growing the
/// card and shifting the feed on the client re-render).
fn time_ago(dt: &time::OffsetDateTime) -> String {
    let s = (time::OffsetDateTime::now_utc() - *dt).whole_seconds();
    if s < 60 {
        return "just now".into();
    }
    let m = s / 60;
    if m < 60 {
        return format!("{m}m");
    }
    let h = m / 60;
    if h < 24 {
        return format!("{h}h");
    }
    let d = h / 24;
    if d < 7 {
        return format!("{d}d");
    }
    const MON: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!("{} {}", MON[(dt.month() as u8 - 1) as usize], dt.day())
}

/// Map a DB post to the SSR listing item (body clamped to match the client's
/// 500-char truncation so the crawlable card and the client card are the same
/// height). Reactions are attached separately (see `build_items`).
fn to_item(p: PostWithAuthor, reactions: Vec<SsrReaction>) -> SsrPostItem {
    let long = p.body.chars().count() > 500;
    let body: String = p.body.chars().take(500).collect();
    SsrPostItem {
        id: p.id.to_string(),
        body: if long { format!("{body}…") } else { body },
        username: p.username,
        channel_slug: p.channel_slug,
        channel_name: p.channel_name,
        like_count: p.like_count,
        comment_count: p.comment_count,
        created_at: time_ago(&p.created_at),
        reactions,
    }
}

/// Fetch aggregated reactions for a set of posts (mirrors the API's grouped
/// query) so the SSR card can render the same reaction chips the client does.
async fn ssr_reactions(
    db: &PgPool,
    post_ids: &[Uuid],
) -> std::collections::HashMap<Uuid, Vec<SsrReaction>> {
    let mut map: std::collections::HashMap<Uuid, Vec<SsrReaction>> = std::collections::HashMap::new();
    if post_ids.is_empty() {
        return map;
    }
    let rows: Vec<(Uuid, String, i64)> = sqlx::query_as(
        "SELECT post_id, icon, COUNT(*)::bigint FROM post_reactions \
         WHERE post_id = ANY($1) GROUP BY post_id, icon ORDER BY MIN(created_at)",
    )
    .bind(post_ids)
    .fetch_all(db)
    .await
    .unwrap_or_default();
    for (pid, icon, count) in rows {
        map.entry(pid).or_default().push(SsrReaction { icon, count });
    }
    map
}

/// Turn fetched posts into SSR items with their reactions attached.
async fn build_items(db: &PgPool, posts: Vec<PostWithAuthor>) -> Vec<SsrPostItem> {
    let ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();
    let mut reactions = ssr_reactions(db, &ids).await;
    posts
        .into_iter()
        .map(|p| {
            let r = reactions.remove(&p.id).unwrap_or_default();
            to_item(p, r)
        })
        .collect()
}

/// Format an `OffsetDateTime` as a UTC ISO-8601 string without pulling in the
/// `time` formatting feature (inherent accessors only).
fn iso(dt: &time::OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        dt.year(), dt.month() as u8, dt.day(), dt.hour(), dt.minute(), dt.second()
    )
}

/// Render the Leptos app with `ctx` available via `use_context` in components.
async fn render_with<T>(ctx: T, req: Request<Body>) -> Response
where
    T: Clone + Send + Sync + 'static,
{
    let render = leptos_axum::render_app_to_stream_with_context(
        move || provide_context(ctx.clone()),
        Shell,
    );
    render(req).await
}

/// `GET /articles/:slug` — server-render a published article.
pub async fn article_detail(db: PgPool, slug: String, req: Request<Body>) -> Response {
    let dto = match Article::find_by_slug(&db, &slug).await {
        Ok(Some(a)) if a.published => {
            let author = User::find_by_id(&db, a.author_id)
                .await
                .ok()
                .flatten()
                .map(|u| u.username)
                .unwrap_or_default();
            ArticleSsr {
                found: true,
                title: a.title,
                slug: a.slug,
                summary: a.summary,
                content_html: a.content,
                cover_image_url: a.cover_image_url,
                author,
            }
        }
        _ => ArticleSsr::default(),
    };
    render_with(dto, req).await
}

/// `GET /courses/:slug` — server-render a course landing.
pub async fn course_detail(db: PgPool, slug: String, req: Request<Body>) -> Response {
    let dto = match Course::find_by_slug(&db, &slug).await {
        Ok(Some(c)) => CourseSsr {
            found: true,
            title: c.title,
            slug: c.slug,
            description: c.description,
        },
        _ => CourseSsr::default(),
    };
    render_with(dto, req).await
}

/// `GET /profile/:username` — server-render the profile heading + title.
pub async fn profile_page(db: PgPool, username: String, req: Request<Body>) -> Response {
    let dto = match User::find_by_username(&db, &username).await {
        Ok(Some(u)) => ProfileSsr {
            found: true,
            username: u.username,
            role: u.role,
        },
        _ => ProfileSsr::default(),
    };
    render_with(dto, req).await
}

/// `GET /docs/:version/*slug` — read the markdown file, render it, and SSR it.
pub async fn doc_article(db_unused: PgPool, full_slug: String, req: Request<Body>) -> Response {
    let _ = db_unused; // docs are file-based; keep the uniform handler shape
    let full = full_slug.trim_matches('/');
    let (version, slug) = match full.split_once('/') {
        Some((v, s)) => (v.to_string(), s.to_string()),
        None => (full.to_string(), String::new()),
    };
    let valid = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '/' | '.')) && !s.contains("..");
    let mut dto = DocSsr { version: version.clone(), slug: slug.clone(), ..Default::default() };
    if valid(&version) && !slug.is_empty() && valid(&slug) {
        let path = std::path::Path::new("docs").join(&version).join(format!("{slug}.md"));
        if let Ok(md) = tokio::fs::read_to_string(&path).await {
            let title = md
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l.trim_start_matches("# ").trim().to_string())
                .unwrap_or_else(|| slug.replace(['-', '/'], " "));
            let mut opts = pulldown_cmark::Options::empty();
            opts.insert(pulldown_cmark::Options::ENABLE_TABLES);
            opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
            let parser = pulldown_cmark::Parser::new_ext(&md, opts);
            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);
            dto.found = true;
            dto.is_page = true;
            dto.title = title;
            dto.content_html = html;
        }
    }
    render_with(dto, req).await
}

/// `GET /marketplace/asset/:slug` — SEO landing for a marketplace asset.
pub async fn asset_detail(db: PgPool, slug: String, req: Request<Body>) -> Response {
    let dto = match Asset::find_by_slug(&db, &slug).await {
        Ok(Some(a)) => {
            let seller = User::find_by_id(&db, a.creator_id)
                .await
                .ok()
                .flatten()
                .map(|u| u.username)
                .unwrap_or_default();
            AssetSsr {
                found: true,
                name: a.name,
                slug: a.slug,
                description: a.description,
                category: a.category,
                price_credits: a.price_credits,
                thumbnail_url: a.thumbnail_url,
                downloads: a.downloads,
                rating_count: a.rating_count,
                seller,
            }
        }
        _ => AssetSsr::default(),
    };
    render_with(dto, req).await
}

/// `GET /community` and `/community/channel/:slug` — server-render the hub or a
/// channel with its public posts so it's indexable with a real title.
pub async fn community_page(db: PgPool, slug: Option<String>, req: Request<Body>) -> Response {
    let mut dto = CommunitySsr::default();
    match slug {
        Some(slug) => {
            // Channel page — resolve the approved channel, then its public posts.
            let row: Option<(String, String, Uuid)> = sqlx::query_as(
                "SELECT name, description, id FROM channels WHERE slug = $1 AND approved = true",
            )
            .bind(&slug)
            .fetch_optional(&db)
            .await
            .ok()
            .flatten();
            if let Some((name, description, cid)) = row {
                let posts = Post::channel_public(&db, cid, 30, None, None)
                    .await
                    .unwrap_or_default();
                dto = CommunitySsr {
                    found: true,
                    is_channel: true,
                    slug,
                    name,
                    description,
                    posts: build_items(&db, posts).await,
                };
            }
        }
        None => {
            // Hub — recent public posts across all channels.
            let posts = Post::public_recent(&db, 30, None, None)
                .await
                .unwrap_or_default();
            dto = CommunitySsr {
                found: true,
                is_channel: false,
                slug: String::new(),
                name: "Community".into(),
                description: "Discussions about the Renzora engine and general game development.".into(),
                posts: build_items(&db, posts).await,
            };
        }
    }
    render_with(dto, req).await
}

/// `GET /community/post/:id` — indexable permalink for a public discussion.
pub async fn post_detail(db: PgPool, id_str: String, req: Request<Body>) -> Response {
    let dto = match Uuid::parse_str(&id_str) {
        Ok(id) => match Post::find_public_by_id(&db, id, None).await {
            Ok(Some(p)) => {
                let comments = PostComment::list_for_post(&db, id, None, 100, 0)
                    .await
                    .unwrap_or_default();
                PostSsr {
                    found: true,
                    id: p.id.to_string(),
                    body: p.body,
                    username: p.username,
                    channel_slug: p.channel_slug,
                    channel_name: p.channel_name,
                    like_count: p.like_count,
                    comment_count: p.comment_count,
                    created_at: iso(&p.created_at),
                    comments: comments
                        .into_iter()
                        .map(|c| SsrComment {
                            username: c.username,
                            body: c.body,
                            created_at: iso(&c.created_at),
                        })
                        .collect(),
                }
            }
            _ => PostSsr::default(),
        },
        Err(_) => PostSsr::default(),
    };
    render_with(dto, req).await
}
