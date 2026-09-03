use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query, State},
    response::Response,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{jwt, AppState};

/// A live event broadcast to connected clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveEvent {
    /// Target user ID (None = broadcast to all).
    pub user_id: Option<Uuid>,
    /// Event type: "credit_update", "collab_invite", etc.
    pub event: String,
    /// JSON payload.
    pub data: serde_json::Value,
}

/// Shared state for WebSocket connections.
#[derive(Clone)]
pub struct WsBroadcast {
    pub tx: broadcast::Sender<LiveEvent>,
    /// Presence: user_id -> number of live connections.
    pub connected: Arc<Mutex<HashMap<Uuid, usize>>>,
}

impl WsBroadcast {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx, connected: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Whether the user has at least one live WebSocket connection.
    pub fn is_online(&self, user_id: Uuid) -> bool {
        self.connected.lock().map(|m| m.contains_key(&user_id)).unwrap_or(false)
    }

    /// Send an event to a specific user.
    pub fn send_to_user(&self, user_id: Uuid, event: &str, data: serde_json::Value) {
        let _ = self.tx.send(LiveEvent {
            user_id: Some(user_id),
            event: event.to_string(),
            data,
        });
    }

    /// Broadcast an event to all connected clients.
    pub fn broadcast(&self, event: &str, data: serde_json::Value) {
        let _ = self.tx.send(LiveEvent {
            user_id: None,
            event: event.to_string(),
            data,
        });
    }
}

#[derive(Deserialize)]
pub struct WsQuery {
    token: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/live", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsQuery>,
) -> Response {
    // Validate token
    let user_id = match jwt::validate_token(&params.token, &state.jwt_secret) {
        Ok(claims) if claims.token_type == "access" => claims.sub,
        _ => {
            return Response::builder()
                .status(401)
                .body("Unauthorized".into())
                .unwrap();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, user_id, state))
}

async fn handle_socket(mut socket: WebSocket, user_id: Uuid, state: AppState) {
    let broadcast = state.ws_broadcast.clone();
    let mut rx = broadcast.tx.subscribe();

    // Presence: register this connection; announce on the first one.
    let first_connection = {
        let mut map = broadcast.connected.lock().unwrap();
        let count = map.entry(user_id).or_insert(0);
        *count += 1;
        *count == 1
    };
    if first_connection {
        announce_presence(&state, user_id, true).await;
    }

    // Send a welcome message
    let welcome = serde_json::json!({"event": "connected", "data": {"user_id": user_id}});
    let _ = socket.send(Message::Text(welcome.to_string())).await;

    loop {
        tokio::select! {
            // Receive broadcast events and forward to this client
            Ok(event) = rx.recv() => {
                // Send if it's for this user or a global broadcast
                if event.user_id.is_none() || event.user_id == Some(user_id) {
                    let msg = serde_json::json!({
                        "event": event.event,
                        "data": event.data,
                    });
                    if socket.send(Message::Text(msg.to_string())).await.is_err() {
                        break; // Client disconnected
                    }
                }
            }
            // Handle incoming messages from client (ping/pong, etc.)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }

    // Presence: unregister; announce when the last connection closes.
    let last_connection = {
        let mut map = broadcast.connected.lock().unwrap();
        match map.get_mut(&user_id) {
            Some(count) => {
                *count -= 1;
                if *count == 0 { map.remove(&user_id); true } else { false }
            }
            None => false,
        }
    };
    if last_connection {
        let _ = sqlx::query("UPDATE users SET last_seen_at = NOW() WHERE id = $1")
            .bind(user_id).execute(&state.db).await;
        announce_presence(&state, user_id, false).await;
    }
}

/// Notify a user's friends that they went online/offline, respecting their
/// `online_status_visible` privacy setting.
async fn announce_presence(state: &AppState, user_id: Uuid, online: bool) {
    let user: Option<(bool, String)> = sqlx::query_as(
        "SELECT online_status_visible, username FROM users WHERE id = $1"
    ).bind(user_id).fetch_optional(&state.db).await.ok().flatten();

    let Some((visible, username)) = user else { return };
    if !visible {
        return;
    }

    let friend_ids: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT friend_id FROM friends WHERE user_id = $1 AND status = 'accepted'"
    ).bind(user_id).fetch_all(&state.db).await.unwrap_or_default();

    let event = if online { "friend_online" } else { "friend_offline" };
    let data = serde_json::json!({"user_id": user_id, "username": username});
    for (friend_id,) in friend_ids {
        state.ws_broadcast.send_to_user(friend_id, event, data.clone());
    }
}
