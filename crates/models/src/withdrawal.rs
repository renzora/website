use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Withdrawal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount_credits: i64,
    pub amount_usd_cents: i64,
    pub status: String,
    pub stripe_transfer_id: Option<String>,
    pub failure_reason: Option<String>,
    pub created_at: OffsetDateTime,
    pub completed_at: Option<OffsetDateTime>,
}

impl Withdrawal {
    /// Create a pending withdrawal request and deduct credits atomically.
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        amount_credits: i64,
    ) -> Result<Self, String> {
        if amount_credits < 500 {
            return Err("Minimum withdrawal is 500 credits ($50)".into());
        }

        let amount_usd_cents = amount_credits * 10; // 1 credit = $0.10 = 10 cents
        let id = Uuid::new_v4();

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Deduct credits from user (with balance check)
        let result = sqlx::query(
            "UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2 AND credit_balance >= $1",
        )
        .bind(amount_credits)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err("Insufficient credits".into());
        }

        // Record withdrawal transaction
        let tx_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO transactions (id, user_id, type, amount) VALUES ($1, $2, 'withdrawal', $3)",
        )
        .bind(tx_id)
        .bind(user_id)
        .bind(-amount_credits)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Create withdrawal record
        let withdrawal = sqlx::query_as::<_, Withdrawal>(
            "INSERT INTO withdrawals (id, user_id, amount_credits, amount_usd_cents) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .bind(amount_credits)
        .bind(amount_usd_cents)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(withdrawal)
    }

    /// List withdrawals for a user.
    pub async fn list_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Withdrawal>(
            "SELECT * FROM withdrawals WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// List all withdrawals (admin).
    pub async fn list_all(pool: &PgPool, status: Option<&str>) -> Result<Vec<WithdrawalWithUser>, sqlx::Error> {
        sqlx::query_as::<_, WithdrawalWithUser>(
            r#"
            SELECT w.id, w.user_id, w.amount_credits, w.amount_usd_cents, w.status,
                   w.stripe_transfer_id, w.failure_reason, w.created_at, w.completed_at,
                   u.username
            FROM withdrawals w
            JOIN users u ON u.id = w.user_id
            WHERE ($1::text IS NULL OR w.status = $1)
            ORDER BY w.created_at DESC
            LIMIT 100
            "#,
        )
        .bind(status)
        .fetch_all(pool)
        .await
    }

    /// Mark a withdrawal as completed with a Stripe transfer ID.
    pub async fn mark_completed(
        pool: &PgPool,
        id: Uuid,
        stripe_transfer_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE withdrawals SET status = 'completed', stripe_transfer_id = $1, completed_at = NOW() WHERE id = $2",
        )
        .bind(stripe_transfer_id)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Mark a withdrawal as failed and refund credits.
    pub async fn mark_failed(
        pool: &PgPool,
        id: Uuid,
        reason: &str,
    ) -> Result<(), String> {
        let withdrawal = sqlx::query_as::<_, Withdrawal>(
            "SELECT * FROM withdrawals WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Withdrawal not found")?;

        if withdrawal.status != "pending" && withdrawal.status != "processing" {
            return Err("Withdrawal is not in a refundable state".into());
        }

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Refund credits
        sqlx::query(
            "UPDATE users SET credit_balance = credit_balance + $1, updated_at = NOW() WHERE id = $2",
        )
        .bind(withdrawal.amount_credits)
        .bind(withdrawal.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Update withdrawal status
        sqlx::query(
            "UPDATE withdrawals SET status = 'failed', failure_reason = $1 WHERE id = $2",
        )
        .bind(reason)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Record refund transaction
        let tx_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO transactions (id, user_id, type, amount) VALUES ($1, $2, 'withdrawal_refund', $3)",
        )
        .bind(tx_id)
        .bind(withdrawal.user_id)
        .bind(withdrawal.amount_credits)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct WithdrawalWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount_credits: i64,
    pub amount_usd_cents: i64,
    pub status: String,
    pub stripe_transfer_id: Option<String>,
    pub failure_reason: Option<String>,
    pub created_at: OffsetDateTime,
    pub completed_at: Option<OffsetDateTime>,
    pub username: String,
}
