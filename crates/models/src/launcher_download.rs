use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct LauncherDownload {
    pub id: Uuid,
    pub platform: String,
    pub version: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

impl LauncherDownload {
    pub async fn record(
        pool: &PgPool,
        platform: &str,
        version: Option<&str>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        user_id: Option<Uuid>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO launcher_downloads (platform, version, ip_address, user_agent, user_id) VALUES ($1,$2,$3,$4,$5) RETURNING *"
        )
        .bind(platform).bind(version).bind(ip_address).bind(user_agent).bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn count_by_month(pool: &PgPool, months: i32) -> Result<Vec<(OffsetDateTime, i64)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT date_trunc('month', created_at) as month, COUNT(*)::bigint as count FROM launcher_downloads WHERE created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month"
        )
        .bind(months)
        .fetch_all(pool)
        .await
    }

    pub async fn count_by_platform(pool: &PgPool) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as("SELECT platform, COUNT(*)::bigint as count FROM launcher_downloads GROUP BY platform ORDER BY count DESC")
            .fetch_all(pool)
            .await
    }
}
