#![recursion_limit = "256"]

mod docs_files;
mod ssr_pages;

use axum::{
    body::Body,
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Request},
    response::Response,
    routing::get,
    Extension, Json, Router,
};
use renzora_api::{api_router, middleware::JwtSecret, AppState};
use renzora_web::shell::{Shell, EmbedShell};
use sqlx::postgres::PgPoolOptions;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

/// Service worker source, served at `/sw.js` (root scope). Network-first for
/// HTML navigations so it can NEVER mask a deploy with a stale page — the cache
/// is purely an offline fallback. It deliberately does not cache static assets:
/// those sit at stable URLs governed by the CDN cache (which you can purge),
/// whereas an on-device SW cache can't be purged remotely.
const SW_JS: &str = r#"const CACHE = 'renzora-v1';
self.addEventListener('install', () => self.skipWaiting());
self.addEventListener('activate', (e) => {
  e.waitUntil((async () => {
    for (const k of await caches.keys()) if (k !== CACHE) await caches.delete(k);
    await self.clients.claim();
  })());
});
self.addEventListener('fetch', (e) => {
  const req = e.request;
  if (req.method !== 'GET' || req.mode !== 'navigate') return;
  const url = new URL(req.url);
  if (url.origin !== self.location.origin || url.pathname.startsWith('/api/')) return;
  e.respondWith((async () => {
    try {
      const res = await fetch(req);
      if (res && res.ok) (await caches.open(CACHE)).put(req, res.clone());
      return res;
    } catch (err) {
      return (await caches.match(req)) || Response.error();
    }
  })());
});
"#;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".into());
    let upload_base_url =
        std::env::var("UPLOAD_BASE_URL").unwrap_or_else(|_| "/uploads".into());
    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").ok();
    let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").ok();
    let site_url =
        std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".into());
    let s3_access_key = std::env::var("S3_ACCESS_KEY").ok();
    let s3_secret_key = std::env::var("S3_SECRET_KEY").ok();
    let s3_endpoint = std::env::var("S3_ENDPOINT").unwrap_or_default();
    let s3_bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "renzora-assets".into());
    let s3_public_url = std::env::var("S3_PUBLIC_URL").unwrap_or_default();

    // Set up S3 bucket (Cloudflare R2 via rust-s3)
    let s3_bucket_obj = if let (Some(access), Some(secret)) = (&s3_access_key, &s3_secret_key) {
        let region = s3::Region::Custom {
            region: "auto".to_string(),
            endpoint: s3_endpoint.clone(),
        };
        let creds = s3::creds::Credentials::new(Some(access), Some(secret), None, None, None)
            .expect("Failed to create S3 credentials");
        let bucket = s3::Bucket::new(&s3_bucket, region, creds)
            .expect("Failed to create S3 bucket")
            .with_path_style();
        tracing::info!("S3 storage configured: {s3_bucket}");
        Some(std::sync::Arc::new(bucket))
    } else {
        tracing::warn!("S3 not configured — using local storage");
        None
    };

    // Ensure upload directories exist
    tokio::fs::create_dir_all(format!("{upload_dir}/assets"))
        .await
        .expect("Failed to create upload/assets directory");
    tokio::fs::create_dir_all(format!("{upload_dir}/thumbnails"))
        .await
        .expect("Failed to create upload/thumbnails directory");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations applied successfully");

    let state = AppState {
        db: pool,
        jwt_secret: jwt_secret.clone(),
        upload_dir: upload_dir.clone(),
        upload_base_url,
        s3_bucket: s3_bucket_obj,
        s3_public_url,
        stripe_secret_key,
        stripe_webhook_secret,
        site_url,
        ws_broadcast: std::sync::Arc::new(renzora_api::WsBroadcast::new()),
    };

    // Background task: renew or expire Supporter subscriptions whose period
    // has ended. Runs shortly after boot, then hourly.
    let renewal_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
        loop {
            interval.tick().await;
            renzora_api::subscriptions::process_due_renewals(&renewal_state).await;
        }
    });

    // CORS
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|o| o.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Leptos SSR handler for frontend pages
    let render = leptos_axum::render_app_to_stream(Shell);
    let render_home = render.clone();
    let ssr = move |req: Request<Body>| {
        let render = render.clone();
        async move { render(req).await }
    };

    // Separate SSR handler for embed pages (no nav/shell)
    let embed_render = leptos_axum::render_app_to_stream(EmbedShell);
    let embed_ssr = move |req: Request<Body>| {
        let render = embed_render.clone();
        async move { render(req).await }
    };

    let db_pool_ext = renzora_api::middleware::DbPool(state.db.clone());

    // SEO endpoints (robots.txt + dynamic sitemap.xml) — capture what they need
    // before `state` is moved into the API router below.
    let seo_site_url = state.site_url.clone();
    let seo_db = state.db.clone();

    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        // Service worker (root scope) — offline fallback, network-first
        .route("/sw.js", get(service_worker))
        // SEO: robots + sitemap
        .route("/robots.txt", get({
            let s = seo_site_url.clone();
            move || { let s = s.clone(); async move { robots_txt(&s) } }
        }))
        .route("/sitemap.xml", get({
            let s = seo_site_url.clone();
            let db = seo_db.clone();
            move || { let s = s.clone(); let db = db.clone(); async move { sitemap_xml(&db, &s).await } }
        }))
        // Serve uploaded files
        .nest_service("/uploads", ServeDir::new(&upload_dir))
        // Serve static assets (CSS, JS, images)
        .nest_service("/assets", ServeDir::new("assets"))
        // API routes (includes file-based docs)
        .nest("/api", api_router(state).merge(Router::new().nest("/docs", docs_files::router())))
        // Frontend pages — explicit SSR routes
        // `/` is the marketing landing for logged-out visitors, and the community
        // feed for signed-in users, rendered under the bare `/` URL (no redirect).
        .route("/", get({
            let db = seo_db.clone();
            let render_home = render_home.clone();
            move |req: Request<Body>| {
                let db = db.clone();
                let render_home = render_home.clone();
                async move {
                    let signed_in = req.headers()
                        .get(axum::http::header::COOKIE)
                        .and_then(|c| c.to_str().ok())
                        .map(|c| c.split(';').any(|kv| {
                            let kv = kv.trim();
                            kv.starts_with("token=") && kv.len() > "token=".len()
                        }))
                        .unwrap_or(false);
                    let mut resp = if signed_in {
                        let (mut parts, body) = req.into_parts();
                        parts.uri = axum::http::Uri::from_static("/community");
                        ssr_pages::community_page(db, None, Request::from_parts(parts, body)).await
                    } else {
                        render_home(req).await
                    };
                    // `/` now depends on the auth cookie (feed vs landing), so it must never be
                    // cached as a shared/static page, otherwise sign-in/out serves stale content.
                    let h = resp.headers_mut();
                    h.insert(
                        axum::http::header::CACHE_CONTROL,
                        axum::http::HeaderValue::from_static("private, no-store, max-age=0, must-revalidate"),
                    );
                    h.insert(axum::http::header::VARY, axum::http::HeaderValue::from_static("Cookie"));
                    resp
                }
            }
        }))
        .route("/download", get(ssr.clone()))
        .route("/game", get(ssr.clone()))
        .route("/login", get(ssr.clone()))
        .route("/register", get(ssr.clone()))
        .route("/docs", get(ssr.clone()))
        .route("/docs/*slug", get({
            let db = seo_db.clone();
            move |axum::extract::Path(slug): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::doc_article(db, slug, req).await }
            }
        }))
        .route("/marketplace", get(ssr.clone()))
        .route("/marketplace/sell", get(ssr.clone()))
        .route("/marketplace/upload", get(ssr.clone()))
        .route("/marketplace/asset/:slug", get({
            // SEO: server-render the asset's content + meta from the DB
            let db = seo_db.clone();
            move |axum::extract::Path(slug): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::asset_detail(db, slug, req).await }
            }
        }))
        .route("/articles/:slug", get({
            let db = seo_db.clone();
            move |axum::extract::Path(slug): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::article_detail(db, slug, req).await }
            }
        }))
        .route("/courses/:slug", get({
            let db = seo_db.clone();
            move |axum::extract::Path(slug): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::course_detail(db, slug, req).await }
            }
        }))
        .route("/profile/:username", get({
            let db = seo_db.clone();
            move |axum::extract::Path(username): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::profile_page(db, username, req).await }
            }
        }))
        .route("/games", get(ssr.clone()))
        .route("/games/upload", get(ssr.clone()))
        .route("/games/:slug", get(ssr.clone()))
        .route("/library", get(ssr.clone()))
        .route("/wallet", get(ssr.clone()))
        .route("/courses", get(ssr.clone()))
        .route("/courses/create", get(ssr.clone()))
        .route("/courses/:slug/edit", get(ssr.clone()))
        .route("/courses/:slug/chapter/:chapter", get(ssr.clone()))
        .route("/community", get({
            // SEO: server-render the hub with recent public posts
            let db = seo_db.clone();
            move |req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::community_page(db, None, req).await }
            }
        }))
        .route("/community/channel/:slug", get({
            // SEO: server-render the channel with its public posts + title
            let db = seo_db.clone();
            move |axum::extract::Path(slug): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::community_page(db, Some(slug), req).await }
            }
        }))
        .route("/community/post/:id", get({
            // SEO: server-render the discussion + comments from the DB
            let db = seo_db.clone();
            move |axum::extract::Path(id): axum::extract::Path<String>, req: Request<Body>| {
                let db = db.clone();
                async move { ssr_pages::post_detail(db, id, req).await }
            }
        }))
        .route("/articles", get(ssr.clone()))
        .route("/articles/write", get(ssr.clone()))
        .route("/friends", get(ssr.clone()))
        .route("/notifications", get(ssr.clone()))
        .route("/shop/:username", get(ssr.clone()))
        .route("/marketplace/asset/:slug/edit", get(ssr.clone()))
        .route("/dashboard", get(ssr.clone()))
        .route("/developers", get(ssr.clone()))
        .route("/subscription", get(ssr.clone()))
        .route("/teams", get(ssr.clone()))
        .route("/messages", get(ssr.clone()))
        .route("/feed", get(ssr.clone()))
        .route("/donate", get(ssr.clone()))
        .route("/gifts", get(ssr.clone()))
        .route("/terms", get(ssr.clone()))
        .route("/privacy", get(ssr.clone()))
        .route("/settings", get(ssr.clone()))
        .route("/embed/preview/:slug", get(embed_ssr.clone()))
        // Layers
        .layer(Extension(JwtSecret(jwt_secret)))
        .layer(Extension(db_pool_ext))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // Edge-cacheable HTML: SSR pages are byte-identical for every visitor
        // (auth is Authorization-header only, so a browser navigation never
        // carries it) — safe for a shared cache. A short s-maxage + long
        // stale-while-revalidate lets Cloudflare serve pages from the edge and
        // refresh them in the background, while max-age=0 keeps the browser
        // revalidating for freshness. Only text/html matches; API JSON and
        // static assets keep their own caching untouched.
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            |res: &Response| {
                let html = res
                    .headers()
                    .get(header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .is_some_and(|ct| ct.starts_with("text/html"));
                html.then(|| {
                    HeaderValue::from_static("public, max-age=0, s-maxage=60, stale-while-revalidate=86400")
                })
            },
        ))
        // A `Link: preload` header for the icon-font subset, so Cloudflare Early
        // Hints can replay it as a 103 before the HTML body. Early Hints reads
        // HTTP Link headers, not the in-<head> <link rel=preload> tag. Only
        // text/html carries it.
        .layer(SetResponseHeaderLayer::if_not_present(
            header::LINK,
            |res: &Response| {
                let html = res
                    .headers()
                    .get(header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .is_some_and(|ct| ct.starts_with("text/html"));
                html.then(|| {
                    HeaderValue::from_static(
                        "</assets/fonts/phosphor-regular.woff2>; rel=preload; as=font; crossorigin",
                    )
                })
            },
        ))
        // Outermost: gzip/brotli-compress responses (HTML, CSS, JS, JSON, sitemap).
        // The default predicate skips already-compressed types (images) and tiny
        // bodies. Harmless behind nginx (which won't re-compress an encoded body).
        .layer(CompressionLayer::new());

    let addr = format!("{host}:{port}");
    tracing::info!("Server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Serve the service worker at the origin root so its scope covers the whole
/// site. `no-cache` makes the browser revalidate it on every load, so a new SW
/// ships promptly; `Service-Worker-Allowed: /` permits the root scope.
async fn service_worker() -> Response {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-cache")
        .header("Service-Worker-Allowed", "/")
        .body(Body::from(SW_JS))
        .unwrap()
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// `GET /robots.txt` — allow crawling of public pages, keep app/private routes
/// out of the index, and advertise the sitemap.
fn robots_txt(site_url: &str) -> Response {
    let base = site_url.trim_end_matches('/');
    let body = format!(
        "User-agent: *\nAllow: /\n\n# Private / app-only routes\nDisallow: /login\nDisallow: /register\nDisallow: /settings\nDisallow: /wallet\nDisallow: /messages\nDisallow: /notifications\nDisallow: /friends\nDisallow: /dashboard\nDisallow: /gifts\nDisallow: /avatar/\n\nSitemap: {base}/sitemap.xml\n"
    );
    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from(body))
        .unwrap()
}

fn push_url(xml: &mut String, loc: &str, lastmod: Option<&str>, changefreq: &str, priority: &str) {
    xml.push_str("<url><loc>");
    xml.push_str(&xml_escape(loc));
    xml.push_str("</loc>");
    if let Some(lm) = lastmod {
        xml.push_str("<lastmod>");
        xml.push_str(lm);
        xml.push_str("</lastmod>");
    }
    xml.push_str("<changefreq>");
    xml.push_str(changefreq);
    xml.push_str("</changefreq><priority>");
    xml.push_str(priority);
    xml.push_str("</priority></url>");
}

/// Recursively collect every `"slug"` string in a docs `_sidebar.json` tree.
fn collect_slugs(v: &serde_json::Value, out: &mut Vec<String>) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(s)) = map.get("slug") {
                out.push(s.clone());
            }
            for val in map.values() {
                collect_slugs(val, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr {
                collect_slugs(val, out);
            }
        }
        _ => {}
    }
}

/// `GET /sitemap.xml` — static public routes plus published marketplace assets,
/// articles and the current docs pages. Absolute URLs use the runtime SITE_URL.
async fn sitemap_xml(db: &sqlx::PgPool, site_url: &str) -> Response {
    let base = site_url.trim_end_matches('/').to_string();
    let mut xml = String::with_capacity(16 * 1024);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">");

    // Static public routes: (path, changefreq, priority)
    let statics: &[(&str, &str, &str)] = &[
        ("/", "daily", "1.0"),
        ("/download", "weekly", "0.9"),
        ("/marketplace", "daily", "0.9"),
        ("/docs", "weekly", "0.8"),
        ("/community", "daily", "0.7"),
        ("/articles", "weekly", "0.6"),
        ("/courses", "weekly", "0.6"),
        ("/developers", "monthly", "0.5"),
        ("/donate", "monthly", "0.4"),
        ("/subscription", "monthly", "0.3"),
        ("/terms", "yearly", "0.2"),
        ("/privacy", "yearly", "0.2"),
    ];
    for (path, cf, pr) in statics {
        push_url(&mut xml, &format!("{base}{path}"), None, cf, pr);
    }

    // Published marketplace assets
    let assets: Vec<(String, String)> = sqlx::query_as(
        "SELECT slug, to_char(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') \
         FROM assets WHERE published = true ORDER BY updated_at DESC LIMIT 5000",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default();
    for (slug, lm) in &assets {
        push_url(&mut xml, &format!("{base}/marketplace/asset/{slug}"), Some(lm), "weekly", "0.6");
    }

    // Published articles
    let articles: Vec<(String, String)> = sqlx::query_as(
        "SELECT slug, to_char(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') \
         FROM articles WHERE published = true ORDER BY updated_at DESC LIMIT 5000",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default();
    for (slug, lm) in &articles {
        push_url(&mut xml, &format!("{base}/articles/{slug}"), Some(lm), "weekly", "0.5");
    }

    // Published courses
    let courses: Vec<(String,)> = sqlx::query_as("SELECT slug FROM courses WHERE published = true")
        .fetch_all(db)
        .await
        .unwrap_or_default();
    for (slug,) in &courses {
        push_url(&mut xml, &format!("{base}/courses/{slug}"), None, "weekly", "0.5");
    }

    // Community channels (public topic pages)
    let channels: Vec<(String,)> = sqlx::query_as("SELECT slug FROM channels ORDER BY slug")
        .fetch_all(db)
        .await
        .unwrap_or_default();
    for (slug,) in &channels {
        push_url(&mut xml, &format!("{base}/community/channel/{slug}"), None, "daily", "0.6");
    }

    // Public community discussions (permalink pages) — public, non-hidden only
    let posts: Vec<(uuid::Uuid, String)> = sqlx::query_as(
        "SELECT id, to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') \
         FROM posts WHERE visibility = 'public' AND hidden = false \
         ORDER BY created_at DESC LIMIT 10000",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default();
    for (id, lm) in &posts {
        push_url(&mut xml, &format!("{base}/community/post/{id}"), Some(lm), "weekly", "0.5");
    }

    // Docs (default version) — read the version config, then its sidebar slugs
    let default_version = std::fs::read_to_string("docs/_versions.json")
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v.get("default").and_then(|d| d.as_str()).map(|s| s.to_string()))
        .unwrap_or_else(|| "r1-alpha6".to_string());
    push_url(&mut xml, &format!("{base}/docs/{default_version}"), None, "weekly", "0.7");
    if let Ok(raw) = std::fs::read_to_string(format!("docs/{default_version}/_sidebar.json")) {
        if let Ok(sidebar) = serde_json::from_str::<serde_json::Value>(&raw) {
            let mut slugs = Vec::new();
            collect_slugs(&sidebar, &mut slugs);
            for slug in slugs {
                push_url(&mut xml, &format!("{base}/docs/{default_version}/{slug}"), None, "monthly", "0.6");
            }
        }
    }

    xml.push_str("</urlset>");

    Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(Body::from(xml))
        .unwrap()
}

