use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Query, State},
    response::Response,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{jwt, AppState};

/// A live event broadcast to connected clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveEvent {
    /// Target user ID (None = broadcast to all).
    pub user_id: Option<Uuid>,
    /// Event type: "notification", "credit_update", "new_post", "new_thread", etc.
    pub event: String,
    /// JSON payload.
    pub data: serde_json::Value,
}

/// Shared state for WebSocket connections.
#[derive(Clone)]
pub struct WsBroadcast {
    pub tx: broadcast::Sender<LiveEvent>,
}

impl WsBroadcast {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx }
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

    let broadcast = state.ws_broadcast.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, user_id, broadcast))
}

async fn handle_socket(mut socket: WebSocket, user_id: Uuid, broadcast: Arc<WsBroadcast>) {
    let mut rx = broadcast.tx.subscribe();

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
}
