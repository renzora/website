pub mod admin;
pub mod articles;
pub mod auth;
pub mod creator;
pub mod courses;
pub mod credits;
pub mod docs;
pub mod error;
pub mod forum;
pub mod jwt;
pub mod marketplace;
pub mod middleware;
pub mod notifications;
pub mod profiles;
pub mod ws;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub use ws::WsBroadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub upload_dir: String,
    pub upload_base_url: String,
    pub s3_bucket: Option<Arc<Box<s3::Bucket>>>,
    pub s3_public_url: String,
    pub stripe_secret_key: Option<String>,
    pub stripe_webhook_secret: Option<String>,
    pub site_url: String,
    pub ws_broadcast: Arc<WsBroadcast>,
}

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/marketplace", marketplace::router())
        .nest("/courses", courses::router())
        .nest("/credits", credits::router())
        .nest("/creator", creator::router())
        // docs are served from static files, not DB (see server/docs_files.rs)
        .nest("/articles", articles::router())
        .nest("/forum", forum::router())
        .nest("/notifications", notifications::router())
        .nest("/profiles", profiles::router())
        .nest("/admin", admin::router())
        .nest("/ws", ws::router())
        .with_state(state)
}
