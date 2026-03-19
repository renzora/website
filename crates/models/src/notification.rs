use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub body: String,
    pub link: Option<String>,
    pub read: bool,
    pub created_at: OffsetDateTime,
}

impl Notification {
    pub async fn create(pool: &PgPool, user_id: Uuid, ntype: &str, title: &str, body: &str, link: Option<&str>) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as("INSERT INTO notifications (id,user_id,type,title,body,link) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *")
            .bind(id).bind(user_id).bind(ntype).bind(title).bind(body).bind(link).fetch_one(pool).await
    }

    pub async fn list_for_user(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM notifications WHERE user_id=$1 ORDER BY created_at DESC LIMIT $2")
            .bind(user_id).bind(limit).fetch_all(pool).await
    }

    pub async fn unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let r: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM notifications WHERE user_id=$1 AND read=false")
            .bind(user_id).fetch_one(pool).await?;
        Ok(r.0)
    }

    pub async fn mark_read(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read=true WHERE id=$1 AND user_id=$2").bind(id).bind(user_id).execute(pool).await?; Ok(())
    }

    pub async fn mark_all_read(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE notifications SET read=true WHERE user_id=$1 AND read=false").bind(user_id).execute(pool).await?; Ok(())
    }
}
