use axum::{extract::{Extension, Path, State}, routing::{get, put}, Json, Router};
use renzora_models::notification::Notification;
use serde::Serialize;
use uuid::Uuid;
use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notifications))
        .route("/count", get(unread_count))
        .route("/read-all", put(mark_all_read))
        .route("/:id/read", put(mark_read))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

#[derive(Serialize)]
struct NotifListResponse { notifications: Vec<Notification>, unread: i64 }

async fn list_notifications(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<NotifListResponse>, ApiError> {
    let notifs = Notification::list_for_user(&state.db, auth.user_id, 50).await?;
    let unread = Notification::unread_count(&state.db, auth.user_id).await?;
    Ok(Json(NotifListResponse { notifications: notifs, unread }))
}

async fn unread_count(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let count = Notification::unread_count(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!({"count": count})))
}

async fn mark_read(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Notification::mark_read(&state.db, id, auth.user_id).await?;
    Ok(Json(serde_json::json!({"message": "ok"})))
}

async fn mark_all_read(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Notification::mark_all_read(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!({"message": "ok"})))
}
