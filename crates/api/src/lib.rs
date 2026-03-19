pub mod articles;
pub mod auth;
pub mod creator;
pub mod credits;
pub mod docs;
pub mod error;
pub mod jwt;
pub mod marketplace;
pub mod middleware;

use axum::Router;
use sqlx::PgPool;

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    /// Base path for asset file storage (local uploads directory).
    pub upload_dir: String,
    /// Public base URL for serving uploaded files.
    pub upload_base_url: String,
    /// Stripe secret API key.
    pub stripe_secret_key: Option<String>,
    /// Stripe webhook signing secret.
    pub stripe_webhook_secret: Option<String>,
    /// Public site URL (for Stripe redirect URLs).
    pub site_url: String,
}

/// Build the API router with all routes.
pub fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/auth", auth::router())
        .nest("/api/marketplace", marketplace::router())
        .nest("/api/credits", credits::router())
        .nest("/api/creator", creator::router())
        .nest("/api/docs", docs::router())
        .nest("/api/articles", articles::router())
        .with_state(state)
}
