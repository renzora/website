use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct AdminActivity {
    pub id: Uuid,
    pub event_type: String,
    pub actor_id: Option<Uuid>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub summary: String,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
}

impl AdminActivity {
    pub async fn record(pool: &PgPool, event_type: &str, actor_id: Option<Uuid>, target_type: Option<&str>, target_id: Option<Uuid>, summary: &str, metadata: serde_json::Value) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO admin_activity_feed (event_type, actor_id, target_type, target_id, summary, metadata) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(event_type).bind(actor_id).bind(target_type).bind(target_id).bind(summary).bind(metadata)
            .execute(pool).await?;
        Ok(())
    }

    pub async fn list_recent(pool: &PgPool, limit: i64, event_type: Option<&str>) -> Result<Vec<Self>, sqlx::Error> {
        if let Some(et) = event_type {
            sqlx::query_as("SELECT * FROM admin_activity_feed WHERE event_type = $1 ORDER BY created_at DESC LIMIT $2")
                .bind(et).bind(limit).fetch_all(pool).await
        } else {
            sqlx::query_as("SELECT * FROM admin_activity_feed ORDER BY created_at DESC LIMIT $1")
                .bind(limit).fetch_all(pool).await
        }
    }
}
