#![recursion_limit = "256"]

mod docs_files;

use axum::{
    body::Body,
    extract::DefaultBodyLimit,
    http::{HeaderValue, Request},
    response::Response,
    routing::get,
    Extension, Json, Router,
};
use renzora_api::{api_router, middleware::JwtSecret, AppState};
use renzora_web::shell::Shell;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
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

    // Set up S3 client (DigitalOcean Spaces)
    let s3_client = if let (Some(access), Some(secret)) = (&s3_access_key, &s3_secret_key) {
        let creds = aws_credential_types::Credentials::new(access, secret, None, None, "env");
        let config = aws_sdk_s3::Config::builder()
            .endpoint_url(&s3_endpoint)
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(creds)
            .force_path_style(true)
            .behavior_version_latest()
            .build();
        let client = aws_sdk_s3::Client::from_conf(config);
        tracing::info!("S3 storage configured: {s3_bucket}");
        Some(client)
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
        s3_client,
        s3_bucket,
        s3_public_url,
        stripe_secret_key,
        stripe_webhook_secret,
        site_url,
        ws_broadcast: std::sync::Arc::new(renzora_api::WsBroadcast::new()),
    };

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
    let ssr = move |req: Request<Body>| {
        let render = render.clone();
        async move { render(req).await }
    };

    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        // Serve uploaded files
        .nest_service("/uploads", ServeDir::new(&upload_dir))
        // Serve static assets (CSS, JS, images)
        .nest_service("/assets", ServeDir::new("assets"))
        // API routes (includes file-based docs)
        .nest("/api", api_router(state).merge(Router::new().nest("/docs", docs_files::router())))
        // Frontend pages — explicit SSR routes
        .route("/", get(ssr.clone()))
        .route("/download", get(ssr.clone()))
        .route("/login", get(ssr.clone()))
        .route("/register", get(ssr.clone()))
        .route("/docs", get(ssr.clone()))
        .route("/docs/game-dev", get(ssr.clone()))
        .route("/docs/developer", get(ssr.clone()))
        .route("/docs/*slug", get(ssr.clone()))
        .route("/marketplace", get(ssr.clone()))
        .route("/marketplace/sell", get(ssr.clone()))
        .route("/marketplace/upload", get(ssr.clone()))
        .route("/marketplace/asset/:slug", get(ssr.clone()))
        .route("/library", get(ssr.clone()))
        .route("/wallet", get(ssr.clone()))
        .route("/courses", get(ssr.clone()))
        .route("/courses/create", get(ssr.clone()))
        .route("/courses/:slug", get(ssr.clone()))
        .route("/courses/:slug/edit", get(ssr.clone()))
        .route("/courses/:slug/chapter/:chapter", get(ssr.clone()))
        .route("/community", get(ssr.clone()))
        .route("/forum", get(ssr.clone()))
        .route("/forum/new", get(ssr.clone()))
        .route("/forum/thread/:slug", get(ssr.clone()))
        .route("/forum/:slug", get(ssr.clone()))
        .route("/profile/:username", get(ssr.clone()))
        .route("/dashboard", get(ssr.clone()))
        .route("/settings", get(ssr.clone()))
        .route("/admin", get(ssr.clone()))
        // Layers
        .layer(Extension(JwtSecret(jwt_secret)))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = format!("{host}:{port}");
    tracing::info!("Server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
