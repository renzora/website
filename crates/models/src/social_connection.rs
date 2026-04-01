use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SocialConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: String,
    pub platform_id: Option<String>,
    pub platform_username: String,
    pub platform_url: Option<String>,
    pub verified: bool,
    pub connected_at: OffsetDateTime,
}

impl SocialConnection {
    pub async fn upsert(pool: &PgPool, user_id: Uuid, platform: &str, username: &str, url: Option<&str>, platform_id: Option<&str>, verified: bool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO social_connections (user_id, platform, platform_username, platform_url, platform_id, verified) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             ON CONFLICT (user_id, platform) DO UPDATE SET platform_username = $3, platform_url = $4, platform_id = COALESCE($5, social_connections.platform_id), verified = $6 OR social_connections.verified, connected_at = NOW() \
             RETURNING *"
        ).bind(user_id).bind(platform).bind(username).bind(url).bind(platform_id).bind(verified)
        .fetch_one(pool).await
    }

    pub async fn list_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM social_connections WHERE user_id = $1 ORDER BY platform")
            .bind(user_id).fetch_all(pool).await
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid, platform: &str) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM social_connections WHERE user_id = $1 AND platform = $2")
            .bind(user_id).bind(platform).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}
