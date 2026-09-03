//! Server-rendered content pages (SEO). Each handler fetches its data from the
//! DB, injects it into Leptos context, and renders the app so the page
//! component can emit crawlable HTML + per-page meta. Only content pages that
//! benefit from SEO are wired here; app/auth pages keep the generic SSR handler.

use axum::{body::Body, http::Request, response::Response};
use leptos::prelude::provide_context;
use sqlx::PgPool;

use renzora_common::ssr::{AssetSsr, DocSsr};
use renzora_models::asset::Asset;
use renzora_models::user::User;
use renzora_web::shell::Shell;

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
            dto.content_html = crate::docs_files::optimize_doc_images(&html);
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
