use renzora_models::notification::Notification;
use uuid::Uuid;

use crate::AppState;

/// Create a notification row and push it to the recipient over the live WebSocket.
///
/// Client contract: the WS `notification` event payload is always the full
/// notification row (same shape as `GET /api/notifications`).
pub async fn notify(
    state: &AppState,
    user_id: Uuid,
    ntype: &str,
    title: &str,
    body: &str,
    link: Option<&str>,
) -> Result<Notification, sqlx::Error> {
    let notification = Notification::create(&state.db, user_id, ntype, title, body, link).await?;
    if let Ok(data) = serde_json::to_value(&notification) {
        state.ws_broadcast.send_to_user(user_id, "notification", data);
    }
    Ok(notification)
}
