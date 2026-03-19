use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String,
    pub amount: i64,
    pub asset_id: Option<Uuid>,
    pub stripe_payment_id: Option<String>,
    pub created_at: OffsetDateTime,
}

impl Transaction {
    /// Record a credit top-up.
    pub async fn create_topup(
        pool: &PgPool,
        user_id: Uuid,
        amount: i64,
        stripe_payment_id: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (id, user_id, type, amount, stripe_payment_id)
            VALUES ($1, $2, 'topup', $3, $4)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(amount)
        .bind(stripe_payment_id)
        .fetch_one(pool)
        .await
    }

    /// Record a purchase transaction.
    pub async fn create_purchase(
        pool: &PgPool,
        user_id: Uuid,
        amount: i64,
        asset_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (id, user_id, type, amount, asset_id)
            VALUES ($1, $2, 'purchase', $3, $4)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(-amount) // negative = deduction
        .bind(asset_id)
        .fetch_one(pool)
        .await
    }

    /// Record a creator earning (credit to creator when someone buys their asset).
    pub async fn create_earning(
        pool: &PgPool,
        creator_id: Uuid,
        amount: i64,
        asset_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (id, user_id, type, amount, asset_id)
            VALUES ($1, $2, 'earning', $3, $4)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(creator_id)
        .bind(amount)
        .bind(asset_id)
        .fetch_one(pool)
        .await
    }

    /// Get transaction history for a user, newest first.
    pub async fn list_for_user(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Self>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let transactions = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        Ok((transactions, total.0))
    }
}

/// Atomically process a purchase: deduct buyer credits, credit creator, record transactions, grant ownership.
/// Returns an error if the buyer has insufficient credits.
pub async fn process_purchase(
    pool: &PgPool,
    buyer_id: Uuid,
    asset_id: Uuid,
    price: i64,
    creator_id: Uuid,
) -> Result<(), String> {
    // Platform takes 20% cut, creator gets 80%
    let creator_share = (price * 80) / 100;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Deduct from buyer (with balance check)
    let result = sqlx::query(
        "UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2 AND credit_balance >= $1",
    )
    .bind(price)
    .bind(buyer_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        return Err("Insufficient credits".into());
    }

    // Credit creator
    sqlx::query(
        "UPDATE users SET credit_balance = credit_balance + $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(creator_share)
    .bind(creator_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Record buyer's purchase transaction
    let purchase_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, asset_id) VALUES ($1, $2, 'purchase', $3, $4)",
    )
    .bind(purchase_id)
    .bind(buyer_id)
    .bind(-price)
    .bind(asset_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Record creator's earning transaction
    let earning_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, asset_id) VALUES ($1, $2, 'earning', $3, $4)",
    )
    .bind(earning_id)
    .bind(creator_id)
    .bind(creator_share)
    .bind(asset_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Grant ownership
    sqlx::query(
        "INSERT INTO user_assets (user_id, asset_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(buyer_id)
    .bind(asset_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

/// Add credits to a user's balance (called after Stripe webhook confirms payment).
pub async fn add_credits(
    pool: &PgPool,
    user_id: Uuid,
    amount: i64,
    stripe_payment_id: &str,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE users SET credit_balance = credit_balance + $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(amount)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, stripe_payment_id) VALUES ($1, $2, 'topup', $3, $4)",
    )
    .bind(id)
    .bind(user_id)
    .bind(amount)
    .bind(stripe_payment_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
