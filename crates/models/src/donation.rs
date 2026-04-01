use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Donation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub message: String,
    pub anonymous: bool,
    pub created_at: OffsetDateTime,
}

impl Donation {
    pub async fn create(pool: &PgPool, user_id: Uuid, amount: i64, message: &str, anonymous: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO donations (user_id, amount, message, anonymous) VALUES ($1, $2, $3, $4) RETURNING *"
        ).bind(user_id).bind(amount).bind(message).bind(anonymous)
        .fetch_one(pool).await
    }

    pub async fn leaderboard(pool: &PgPool, limit: i64) -> Result<Vec<(Uuid, String, Option<String>, i64, bool)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT d.user_id, u.username, u.avatar_url, SUM(d.amount)::bigint as total, bool_and(d.anonymous) as all_anonymous \
             FROM donations d JOIN users u ON u.id = d.user_id \
             GROUP BY d.user_id, u.username, u.avatar_url ORDER BY total DESC LIMIT $1"
        ).bind(limit).fetch_all(pool).await
    }

    pub async fn total(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount), 0)::bigint FROM donations")
            .fetch_one(pool).await?;
        Ok(row.0)
    }
}
