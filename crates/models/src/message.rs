use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Conversation {
    pub id: Uuid,
    pub kind: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub team_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ConversationParticipant {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub last_read_at: OffsetDateTime,
    pub muted: bool,
    pub joined_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub body: String,
    pub reply_to_id: Option<Uuid>,
    pub edited_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct MessageAttachment {
    pub id: Uuid,
    pub message_id: Uuid,
    pub file_url: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: OffsetDateTime,
}

/// For listing conversations with preview info.
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ConversationPreview {
    pub id: Uuid,
    pub kind: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub updated_at: OffsetDateTime,
    // Last message info
    pub last_message_body: Option<String>,
    pub last_message_sender: Option<String>,
    pub last_message_at: Option<OffsetDateTime>,
    // Unread count for this user
    pub unread_count: i64,
}

/// For message listing with sender info.
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct MessageWithSender {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub sender_avatar_url: Option<String>,
    pub body: String,
    pub reply_to_id: Option<Uuid>,
    pub edited_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl Conversation {
    /// Find or create a DM conversation between two users.
    pub async fn find_or_create_dm(pool: &PgPool, user_a: Uuid, user_b: Uuid) -> Result<Uuid, sqlx::Error> {
        // Check for existing DM
        let existing = sqlx::query_as::<_, (Uuid,)>(
            "SELECT c.id FROM conversations c \
             JOIN conversation_participants p1 ON p1.conversation_id = c.id AND p1.user_id = $1 \
             JOIN conversation_participants p2 ON p2.conversation_id = c.id AND p2.user_id = $2 \
             WHERE c.kind = 'dm' LIMIT 1"
        ).bind(user_a).bind(user_b).fetch_optional(pool).await?;

        if let Some((id,)) = existing {
            return Ok(id);
        }

        // Create new DM
        let conv: (Uuid,) = sqlx::query_as(
            "INSERT INTO conversations (kind, created_by) VALUES ('dm', $1) RETURNING id"
        ).bind(user_a).fetch_one(pool).await?;

        sqlx::query("INSERT INTO conversation_participants (conversation_id, user_id, role) VALUES ($1, $2, 'member'), ($1, $3, 'member')")
            .bind(conv.0).bind(user_a).bind(user_b).execute(pool).await?;

        Ok(conv.0)
    }

    /// Create a group conversation.
    pub async fn create_group(pool: &PgPool, creator_id: Uuid, name: &str, member_ids: &[Uuid]) -> Result<Uuid, sqlx::Error> {
        let conv: (Uuid,) = sqlx::query_as(
            "INSERT INTO conversations (kind, name, created_by) VALUES ('group', $1, $2) RETURNING id"
        ).bind(name).bind(creator_id).fetch_one(pool).await?;

        // Add creator as owner
        sqlx::query("INSERT INTO conversation_participants (conversation_id, user_id, role) VALUES ($1, $2, 'owner')")
            .bind(conv.0).bind(creator_id).execute(pool).await?;

        // Add members
        for member_id in member_ids {
            if *member_id != creator_id {
                sqlx::query("INSERT INTO conversation_participants (conversation_id, user_id, role) VALUES ($1, $2, 'member') ON CONFLICT DO NOTHING")
                    .bind(conv.0).bind(member_id).execute(pool).await?;
            }
        }

        Ok(conv.0)
    }

    /// Get or create the admin staff chat (singleton).
    pub async fn find_or_create_admin_chat(pool: &PgPool) -> Result<Uuid, sqlx::Error> {
        let existing = sqlx::query_as::<_, (Uuid,)>(
            "SELECT id FROM conversations WHERE kind = 'admin_staff' LIMIT 1"
        ).fetch_optional(pool).await?;

        if let Some((id,)) = existing {
            return Ok(id);
        }

        let conv: (Uuid,) = sqlx::query_as(
            "INSERT INTO conversations (kind, name) VALUES ('admin_staff', 'Staff Chat') RETURNING id"
        ).fetch_one(pool).await?;

        Ok(conv.0)
    }

    /// List conversations for a user with preview info.
    pub async fn list_for_user(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<ConversationPreview>, sqlx::Error> {
        sqlx::query_as::<_, ConversationPreview>(
            "SELECT c.id, c.kind, \
                COALESCE(c.name, (SELECT u.username FROM conversation_participants cp2 JOIN users u ON u.id = cp2.user_id WHERE cp2.conversation_id = c.id AND cp2.user_id != $1 LIMIT 1)) as name, \
                COALESCE(c.avatar_url, (SELECT u.avatar_url FROM conversation_participants cp3 JOIN users u ON u.id = cp3.user_id WHERE cp3.conversation_id = c.id AND cp3.user_id != $1 LIMIT 1)) as avatar_url, \
                c.updated_at, \
                m.body as last_message_body, \
                su.username as last_message_sender, \
                m.created_at as last_message_at, \
                (SELECT COUNT(*) FROM messages m2 WHERE m2.conversation_id = c.id AND m2.created_at > cp.last_read_at AND m2.sender_id != $1 AND m2.deleted_at IS NULL)::bigint as unread_count \
             FROM conversations c \
             JOIN conversation_participants cp ON cp.conversation_id = c.id AND cp.user_id = $1 \
             LEFT JOIN LATERAL (SELECT body, sender_id, created_at FROM messages WHERE conversation_id = c.id AND deleted_at IS NULL ORDER BY created_at DESC LIMIT 1) m ON true \
             LEFT JOIN users su ON su.id = m.sender_id \
             ORDER BY COALESCE(m.created_at, c.updated_at) DESC LIMIT $2"
        ).bind(user_id).bind(limit).fetch_all(pool).await
    }

    /// Check if user is a participant.
    pub async fn is_participant(pool: &PgPool, conversation_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*)::bigint FROM conversation_participants WHERE conversation_id = $1 AND user_id = $2"
        ).bind(conversation_id).bind(user_id).fetch_one(pool).await?;
        Ok(row.0 > 0)
    }

    /// Get participant user IDs for a conversation.
    pub async fn participant_ids(pool: &PgPool, conversation_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (Uuid,)>(
            "SELECT user_id FROM conversation_participants WHERE conversation_id = $1"
        ).bind(conversation_id).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }
}

impl Message {
    pub async fn create(pool: &PgPool, conversation_id: Uuid, sender_id: Uuid, body: &str, reply_to_id: Option<Uuid>) -> Result<Self, sqlx::Error> {
        // Update conversation timestamp
        sqlx::query("UPDATE conversations SET updated_at = NOW() WHERE id = $1")
            .bind(conversation_id).execute(pool).await?;

        sqlx::query_as::<_, Self>(
            "INSERT INTO messages (conversation_id, sender_id, body, reply_to_id) VALUES ($1, $2, $3, $4) RETURNING *"
        ).bind(conversation_id).bind(sender_id).bind(body).bind(reply_to_id)
        .fetch_one(pool).await
    }

    pub async fn list_for_conversation(pool: &PgPool, conversation_id: Uuid, limit: i64, before_id: Option<Uuid>) -> Result<Vec<MessageWithSender>, sqlx::Error> {
        if let Some(before) = before_id {
            sqlx::query_as::<_, MessageWithSender>(
                "SELECT m.id, m.conversation_id, m.sender_id, u.username as sender_username, u.avatar_url as sender_avatar_url, \
                 m.body, m.reply_to_id, m.edited_at, m.deleted_at, m.created_at \
                 FROM messages m JOIN users u ON u.id = m.sender_id \
                 WHERE m.conversation_id = $1 AND m.created_at < (SELECT created_at FROM messages WHERE id = $2) \
                 ORDER BY m.created_at DESC LIMIT $3"
            ).bind(conversation_id).bind(before).bind(limit).fetch_all(pool).await
        } else {
            sqlx::query_as::<_, MessageWithSender>(
                "SELECT m.id, m.conversation_id, m.sender_id, u.username as sender_username, u.avatar_url as sender_avatar_url, \
                 m.body, m.reply_to_id, m.edited_at, m.deleted_at, m.created_at \
                 FROM messages m JOIN users u ON u.id = m.sender_id \
                 WHERE m.conversation_id = $1 \
                 ORDER BY m.created_at DESC LIMIT $2"
            ).bind(conversation_id).bind(limit).fetch_all(pool).await
        }
    }

    pub async fn edit(pool: &PgPool, id: Uuid, sender_id: Uuid, new_body: &str) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE messages SET body = $1, edited_at = NOW() WHERE id = $2 AND sender_id = $3 AND deleted_at IS NULL")
            .bind(new_body).bind(id).bind(sender_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn soft_delete(pool: &PgPool, id: Uuid, sender_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE messages SET deleted_at = NOW(), body = '' WHERE id = $1 AND sender_id = $2 AND deleted_at IS NULL")
            .bind(id).bind(sender_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn mark_read(pool: &PgPool, conversation_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE conversation_participants SET last_read_at = NOW() WHERE conversation_id = $1 AND user_id = $2")
            .bind(conversation_id).bind(user_id).execute(pool).await?;
        Ok(())
    }

    pub async fn total_unread(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COALESCE(SUM(cnt), 0)::bigint FROM ( \
                SELECT COUNT(*) as cnt FROM conversation_participants cp \
                JOIN messages m ON m.conversation_id = cp.conversation_id \
                WHERE cp.user_id = $1 AND m.created_at > cp.last_read_at AND m.sender_id != $1 AND m.deleted_at IS NULL \
            ) sub"
        ).bind(user_id).fetch_one(pool).await?;
        Ok(row.0)
    }
}
