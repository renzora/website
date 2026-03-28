use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub token_hash: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub last_used_at: Option<OffsetDateTime>,
    pub expires_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl ApiToken {
    pub async fn create(
        db: &PgPool,
        user_id: Uuid,
        name: &str,
        token_hash: &str,
        prefix: &str,
        scopes: &[String],
        expires_at: Option<OffsetDateTime>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO api_tokens (user_id, name, token_hash, prefix, scopes, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(user_id)
        .bind(name)
        .bind(token_hash)
        .bind(prefix)
        .bind(scopes)
        .bind(expires_at)
        .fetch_one(db)
        .await
    }

    pub async fn find_by_hash(db: &PgPool, token_hash: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM api_tokens WHERE token_hash = $1")
            .bind(token_hash)
            .fetch_optional(db)
            .await
    }

    pub async fn list_by_user(db: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM api_tokens WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(db)
            .await
    }

    pub async fn delete(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM api_tokens WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn touch_last_used(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE api_tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;
        Ok(())
    }
}
