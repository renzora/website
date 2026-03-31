pub mod admin;
pub mod api_tokens;
pub mod articles;
pub mod auth;
pub mod creator;
pub mod courses;
pub mod discord;
pub mod credits;
pub mod docs;
pub mod error;
pub mod feed;
pub mod forum;
pub mod games;
pub mod gameservices;
pub mod jwt;
pub mod library;
pub mod marketplace;
pub mod messages;
pub mod preview;
pub mod middleware;
pub mod notifications;
pub mod profiles;
pub mod subscriptions;
pub mod teams;
pub mod user;
pub mod ws;

use axum::{extract::State, Json, Router};
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
        .nest("/games", games::router())
        .nest("/gameservices", gameservices::router())
        .nest("/courses", courses::router())
        .nest("/credits", credits::router())
        .nest("/creator", creator::router())
        // docs are served from static files, not DB (see server/docs_files.rs)
        .nest("/articles", articles::router())
        .nest("/forum", forum::router())
        .nest("/notifications", notifications::router())
        .nest("/feed", feed::router())
        .nest("/profiles", profiles::router())
        .nest("/admin", admin::router())
        .nest("/api-tokens", api_tokens::router())
        .nest("/subscriptions", subscriptions::router())
        .nest("/teams", teams::router())
        .nest("/library", library::router())
        .nest("/messages", messages::router())
        .nest("/user", user::router())
        .nest("/ws", ws::router())
        .route("/launcher/download", axum::routing::post(launcher_download_track))
        .with_state(state)
}

async fn launcher_download_track(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> axum::http::StatusCode {
    let platform = body.get("platform").and_then(|v| v.as_str()).unwrap_or("unknown");
    let version = body.get("version").and_then(|v| v.as_str());
    let user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok());

    let _ = renzora_models::launcher_download::LauncherDownload::record(
        &state.db, platform, version, None, user_agent, None,
    ).await;

    axum::http::StatusCode::OK
}
