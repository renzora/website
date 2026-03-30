use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Leaderboard {
    pub id: Uuid,
    pub app_id: Uuid,
    pub leaderboard_key: String,
    pub name: String,
    pub sort_order: String, // "asc" or "desc"
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct LeaderboardEntry {
    pub id: Uuid,
    pub leaderboard_id: Uuid,
    pub user_id: Uuid,
    pub score: i64,
    pub metadata: serde_json::Value,
    pub submitted_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct LeaderboardEntryWithUser {
    pub id: Uuid,
    pub leaderboard_id: Uuid,
    pub user_id: Uuid,
    pub score: i64,
    pub metadata: serde_json::Value,
    pub submitted_at: OffsetDateTime,
    pub username: String,
    pub avatar_url: Option<String>,
}

impl Leaderboard {
    pub async fn create(
        pool: &PgPool,
        app_id: Uuid,
        key: &str,
        name: &str,
        sort_order: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO leaderboards (app_id, leaderboard_key, name, sort_order) VALUES ($1,$2,$3,$4) RETURNING *"
        )
        .bind(app_id).bind(key).bind(name).bind(sort_order)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_key(pool: &PgPool, app_id: Uuid, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM leaderboards WHERE app_id = $1 AND leaderboard_key = $2")
            .bind(app_id).bind(key).fetch_optional(pool).await
    }

    pub async fn list_by_app(pool: &PgPool, app_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM leaderboards WHERE app_id = $1 ORDER BY created_at")
            .bind(app_id).fetch_all(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, app_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM leaderboards WHERE id = $1 AND app_id = $2")
            .bind(id).bind(app_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}

impl LeaderboardEntry {
    /// Submit or update a score. Uses upsert — keeps the best score based on sort order.
    pub async fn submit(
        pool: &PgPool,
        leaderboard_id: Uuid,
        user_id: Uuid,
        score: i64,
        metadata: serde_json::Value,
        sort_order: &str,
    ) -> Result<Self, sqlx::Error> {
        // Upsert: only replace if the new score is better
        let cmp = if sort_order == "asc" { "<" } else { ">" };
        sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO leaderboard_entries (leaderboard_id, user_id, score, metadata) VALUES ($1,$2,$3,$4) \
             ON CONFLICT (leaderboard_id, user_id) DO UPDATE SET score = $3, metadata = $4, submitted_at = NOW() \
             WHERE leaderboard_entries.score IS NULL OR $3 {cmp} leaderboard_entries.score \
             RETURNING *"
        ))
        .bind(leaderboard_id).bind(user_id).bind(score).bind(metadata)
        .fetch_one(pool)
        .await
    }

    /// Get top scores for a leaderboard.
    pub async fn top(
        pool: &PgPool,
        leaderboard_id: Uuid,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LeaderboardEntryWithUser>, sqlx::Error> {
        let order = if sort_order == "asc" { "ASC" } else { "DESC" };
        sqlx::query_as::<_, LeaderboardEntryWithUser>(&format!(
            "SELECT e.id, e.leaderboard_id, e.user_id, e.score, e.metadata, e.submitted_at, u.username, u.avatar_url \
             FROM leaderboard_entries e JOIN users u ON u.id = e.user_id \
             WHERE e.leaderboard_id = $1 ORDER BY e.score {order} LIMIT $2 OFFSET $3"
        ))
        .bind(leaderboard_id).bind(limit).bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Get a player's entry on a leaderboard.
    pub async fn find_for_user(
        pool: &PgPool,
        leaderboard_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM leaderboard_entries WHERE leaderboard_id = $1 AND user_id = $2")
            .bind(leaderboard_id).bind(user_id).fetch_optional(pool).await
    }
}
