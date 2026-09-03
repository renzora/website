use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct GiftCard {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Option<Uuid>,
    pub code: String,
    pub amount: i64,
    pub message: String,
    pub status: String,
    pub redeemed_by: Option<Uuid>,
    pub redeemed_at: Option<OffsetDateTime>,
    pub expires_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl GiftCard {
    pub async fn create(pool: &PgPool, sender_id: Uuid, recipient_id: Option<Uuid>, code: &str, amount: i64, message: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO gift_cards (sender_id, recipient_id, code, amount, message, expires_at) VALUES ($1, $2, $3, $4, $5, NOW() + INTERVAL '90 days') RETURNING *"
        ).bind(sender_id).bind(recipient_id).bind(code).bind(amount).bind(message)
        .fetch_one(pool).await
    }

    pub async fn find_by_code(pool: &PgPool, code: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM gift_cards WHERE code = $1 AND status = 'pending' AND (expires_at IS NULL OR expires_at > NOW())")
            .bind(code.to_uppercase()).fetch_optional(pool).await
    }

    pub async fn redeem(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE gift_cards SET status = 'redeemed', redeemed_by = $1, redeemed_at = NOW() WHERE id = $2")
            .bind(user_id).bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn list_sent(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM gift_cards WHERE sender_id = $1 ORDER BY created_at DESC LIMIT 50")
            .bind(user_id).fetch_all(pool).await
    }

    pub async fn list_received(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM gift_cards WHERE (recipient_id = $1 OR redeemed_by = $1) ORDER BY created_at DESC LIMIT 50")
            .bind(user_id).fetch_all(pool).await
    }
}
