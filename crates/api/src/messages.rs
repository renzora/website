use axum::{
    extract::{Extension, Path, Query, State},
    routing::{get, post, put, delete as delete_route},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use time::format_description::well_known::Rfc3339;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/dm/:user_id", post(get_or_create_dm))
        .route("/conversations/group", post(create_group))
        .route("/conversations/:id/messages", get(list_messages))
        .route("/conversations/:id/messages", post(send_message))
        .route("/conversations/:id/messages/:mid", put(edit_message))
        .route("/conversations/:id/messages/:mid", delete_route(delete_message))
        .route("/conversations/:id/read", post(mark_read))
        .route("/conversations/:id/participants", get(list_participants))
        .route("/unread-count", get(unread_count))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

async fn list_conversations(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let convos = renzora_models::message::Conversation::list_for_user(&state.db, auth.user_id, 50).await?;
    let items: Vec<serde_json::Value> = convos.iter().map(|c| serde_json::json!({
        "id": c.id,
        "kind": c.kind,
        "name": c.name,
        "avatar_url": c.avatar_url,
        "updated_at": c.updated_at.format(&Rfc3339).unwrap_or_default(),
        "last_message_body": c.last_message_body,
        "last_message_sender": c.last_message_sender,
        "last_message_at": c.last_message_at.map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "unread_count": c.unread_count,
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

async fn get_or_create_dm(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(other_user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if auth.user_id == other_user_id {
        return Err(ApiError::Validation("Cannot message yourself".into()));
    }
    let conv_id = renzora_models::message::Conversation::find_or_create_dm(&state.db, auth.user_id, other_user_id).await?;
    Ok(Json(serde_json::json!({"conversation_id": conv_id})))
}

#[derive(Deserialize)]
struct CreateGroupBody {
    name: String,
    member_ids: Vec<Uuid>,
}

async fn create_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateGroupBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.name.is_empty() || body.name.len() > 100 {
        return Err(ApiError::Validation("Name must be 1-100 characters".into()));
    }
    if body.member_ids.is_empty() {
        return Err(ApiError::Validation("Must include at least one member".into()));
    }
    let conv_id = renzora_models::message::Conversation::create_group(&state.db, auth.user_id, &body.name, &body.member_ids).await?;
    Ok(Json(serde_json::json!({"conversation_id": conv_id})))
}

#[derive(Deserialize)]
struct MessageQuery {
    before: Option<Uuid>,
    limit: Option<i64>,
}

async fn list_messages(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(conversation_id): Path<Uuid>,
    Query(params): Query<MessageQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !renzora_models::message::Conversation::is_participant(&state.db, conversation_id, auth.user_id).await? {
        return Err(ApiError::Unauthorized);
    }
    let limit = params.limit.unwrap_or(50).min(100);
    let messages = renzora_models::message::Message::list_for_conversation(&state.db, conversation_id, limit, params.before).await?;
    let items: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({
        "id": m.id,
        "conversation_id": m.conversation_id,
        "sender_id": m.sender_id,
        "sender_username": m.sender_username,
        "sender_avatar_url": m.sender_avatar_url,
        "body": if m.deleted_at.is_some() { "" } else { &m.body },
        "reply_to_id": m.reply_to_id,
        "edited_at": m.edited_at.map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "deleted": m.deleted_at.is_some(),
        "created_at": m.created_at.format(&Rfc3339).unwrap_or_default(),
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

#[derive(Deserialize)]
struct SendMessageBody {
    body: String,
    reply_to_id: Option<Uuid>,
}

async fn send_message(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(conversation_id): Path<Uuid>,
    Json(body): Json<SendMessageBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !renzora_models::message::Conversation::is_participant(&state.db, conversation_id, auth.user_id).await? {
        return Err(ApiError::Unauthorized);
    }
    if body.body.trim().is_empty() {
        return Err(ApiError::Validation("Message cannot be empty".into()));
    }
    let msg = renzora_models::message::Message::create(&state.db, conversation_id, auth.user_id, &body.body, body.reply_to_id).await?;

    // Get sender username for WS broadcast
    let sender = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;

    // Broadcast to all participants via WebSocket
    let participants = renzora_models::message::Conversation::participant_ids(&state.db, conversation_id).await?;
    let event_data = serde_json::json!({
        "conversation_id": conversation_id,
        "message_id": msg.id,
        "sender_id": auth.user_id,
        "sender_username": sender.username,
        "body": msg.body,
        "reply_to_id": msg.reply_to_id,
        "created_at": msg.created_at.format(&Rfc3339).unwrap_or_default(),
    });
    for pid in &participants {
        if *pid != auth.user_id {
            state.ws_broadcast.send_to_user(*pid, "new_message", event_data.clone());
        }
    }

    Ok(Json(serde_json::json!({
        "id": msg.id,
        "conversation_id": msg.conversation_id,
        "body": msg.body,
        "created_at": msg.created_at.format(&Rfc3339).unwrap_or_default(),
    })))
}

async fn edit_message(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((conversation_id, message_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<SendMessageBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let edited = renzora_models::message::Message::edit(&state.db, message_id, auth.user_id, &body.body).await?;
    if !edited {
        return Err(ApiError::NotFound);
    }
    // Broadcast edit
    let participants = renzora_models::message::Conversation::participant_ids(&state.db, conversation_id).await?;
    for pid in &participants {
        if *pid != auth.user_id {
            state.ws_broadcast.send_to_user(*pid, "message_edited", serde_json::json!({
                "conversation_id": conversation_id,
                "message_id": message_id,
                "body": body.body,
            }));
        }
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn delete_message(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((conversation_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deleted = renzora_models::message::Message::soft_delete(&state.db, message_id, auth.user_id).await?;
    if !deleted {
        return Err(ApiError::NotFound);
    }
    let participants = renzora_models::message::Conversation::participant_ids(&state.db, conversation_id).await?;
    for pid in &participants {
        if *pid != auth.user_id {
            state.ws_broadcast.send_to_user(*pid, "message_deleted", serde_json::json!({
                "conversation_id": conversation_id,
                "message_id": message_id,
            }));
        }
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn mark_read(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    renzora_models::message::Message::mark_read(&state.db, conversation_id, auth.user_id).await?;
    // Broadcast read receipt
    let participants = renzora_models::message::Conversation::participant_ids(&state.db, conversation_id).await?;
    for pid in &participants {
        if *pid != auth.user_id {
            state.ws_broadcast.send_to_user(*pid, "read_receipt", serde_json::json!({
                "conversation_id": conversation_id,
                "user_id": auth.user_id,
            }));
        }
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn list_participants(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !renzora_models::message::Conversation::is_participant(&state.db, conversation_id, auth.user_id).await? {
        return Err(ApiError::Unauthorized);
    }
    let rows = sqlx::query_as::<_, (Uuid, String, Option<String>, String, time::OffsetDateTime)>(
        "SELECT cp.user_id, u.username, u.avatar_url, cp.role, cp.joined_at FROM conversation_participants cp JOIN users u ON u.id = cp.user_id WHERE cp.conversation_id = $1 ORDER BY cp.joined_at"
    ).bind(conversation_id).fetch_all(&state.db).await?;
    let items: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "user_id": r.0,
        "username": r.1,
        "avatar_url": r.2,
        "role": r.3,
        "joined_at": r.4.format(&Rfc3339).unwrap_or_default(),
    })).collect();
    Ok(Json(serde_json::json!(items)))
}

async fn unread_count(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let count = renzora_models::message::Message::total_unread(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!({"count": count})))
}
