//! Stripe-backed automatic credit top-ups. A credits feature — it rides on the
//! marketplace balance, not on any subscription tier.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AutoTopup {
    pub user_id: Uuid,
    pub enabled: bool,
    pub threshold_credits: i32,
    pub topup_amount_credits: i32,
    pub stripe_payment_method_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub last_topup_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl AutoTopup {
    pub async fn find(db: &PgPool, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM auto_topup WHERE user_id = $1")
            .bind(user_id).fetch_optional(db).await
    }

    pub async fn upsert(
        db: &PgPool,
        user_id: Uuid,
        enabled: bool,
        threshold: i32,
        amount: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO auto_topup (user_id, enabled, threshold_credits, topup_amount_credits)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (user_id) DO UPDATE SET
                enabled = EXCLUDED.enabled,
                threshold_credits = EXCLUDED.threshold_credits,
                topup_amount_credits = EXCLUDED.topup_amount_credits
             RETURNING *"
        ).bind(user_id).bind(enabled).bind(threshold).bind(amount)
        .fetch_one(db).await
    }
}
