use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Friend {
    pub id: Uuid,
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub status: String, // pending, accepted, blocked
    pub created_at: OffsetDateTime,
}

/// Friend with username/avatar for display.
#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct FriendWithProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub friend_username: String,
    pub friend_avatar_url: Option<String>,
}

impl Friend {
    /// Send a friend request.
    pub async fn send_request(pool: &PgPool, user_id: Uuid, friend_id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO friends (user_id, friend_id, status) VALUES ($1, $2, 'pending') ON CONFLICT (user_id, friend_id) DO NOTHING RETURNING *"
        )
        .bind(user_id).bind(friend_id)
        .fetch_one(pool)
        .await
    }

    /// Accept a friend request (updates both directions).
    pub async fn accept(pool: &PgPool, user_id: Uuid, friend_id: Uuid) -> Result<bool, sqlx::Error> {
        // Update the incoming request
        let r = sqlx::query(
            "UPDATE friends SET status = 'accepted' WHERE user_id = $1 AND friend_id = $2 AND status = 'pending'"
        )
        .bind(friend_id).bind(user_id) // the request was FROM friend_id TO user_id
        .execute(pool).await?;

        if r.rows_affected() == 0 {
            return Ok(false);
        }

        // Create the reverse record
        sqlx::query(
            "INSERT INTO friends (user_id, friend_id, status) VALUES ($1, $2, 'accepted') ON CONFLICT (user_id, friend_id) DO UPDATE SET status = 'accepted'"
        )
        .bind(user_id).bind(friend_id)
        .execute(pool).await?;

        Ok(true)
    }

    /// Remove a friend (both directions).
    pub async fn remove(pool: &PgPool, user_id: Uuid, friend_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM friends WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)")
            .bind(user_id).bind(friend_id)
            .execute(pool).await?;
        Ok(())
    }

    /// Block a user.
    pub async fn block(pool: &PgPool, user_id: Uuid, blocked_id: Uuid) -> Result<(), sqlx::Error> {
        // Remove existing friendship
        sqlx::query("DELETE FROM friends WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)")
            .bind(user_id).bind(blocked_id).execute(pool).await?;
        // Insert block
        sqlx::query("INSERT INTO friends (user_id, friend_id, status) VALUES ($1, $2, 'blocked') ON CONFLICT (user_id, friend_id) DO UPDATE SET status = 'blocked'")
            .bind(user_id).bind(blocked_id).execute(pool).await?;
        Ok(())
    }

    /// List accepted friends with profile info.
    pub async fn list_friends(pool: &PgPool, user_id: Uuid) -> Result<Vec<FriendWithProfile>, sqlx::Error> {
        sqlx::query_as::<_, FriendWithProfile>(
            "SELECT f.id, f.user_id, f.friend_id, f.status, f.created_at, u.username AS friend_username, u.avatar_url AS friend_avatar_url FROM friends f JOIN users u ON u.id = f.friend_id WHERE f.user_id = $1 AND f.status = 'accepted' ORDER BY u.username"
        )
        .bind(user_id).fetch_all(pool).await
    }

    /// List pending incoming requests.
    pub async fn list_incoming_requests(pool: &PgPool, user_id: Uuid) -> Result<Vec<FriendWithProfile>, sqlx::Error> {
        sqlx::query_as::<_, FriendWithProfile>(
            "SELECT f.id, f.user_id, f.friend_id, f.status, f.created_at, u.username AS friend_username, u.avatar_url AS friend_avatar_url FROM friends f JOIN users u ON u.id = f.user_id WHERE f.friend_id = $1 AND f.status = 'pending' ORDER BY f.created_at DESC"
        )
        .bind(user_id).fetch_all(pool).await
    }

    /// Check friendship status between two users.
    pub async fn status(pool: &PgPool, user_id: Uuid, other_id: Uuid) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT status FROM friends WHERE user_id = $1 AND friend_id = $2"
        )
        .bind(user_id).bind(other_id)
        .fetch_optional(pool).await?;
        Ok(row.map(|r| r.0))
    }
}
