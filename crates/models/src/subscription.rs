use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_credits: i32,
    // Limits
    pub daily_api_limit: i32,
    pub storage_mb: i32,
    pub max_team_members: i32,
    pub max_file_size_mb: i32,
    // Add-on pricing
    pub extra_seat_credits: i32,
    pub extra_storage_credits_per_gb: i32,
    // Marketplace
    pub commission_percent: i32,
    pub library_assets_per_month: i32,
    pub search_boost: i32,
    pub asset_spotlights_per_month: i32,
    // Xbox porting
    pub xbox_builds_per_month: i32,
    pub xbox_build_cost_credits: i32,
    pub xbox_submission_cost_credits: i32,
    // Profile
    pub profile_badge: String,
    pub profile_customization: String,
    // Features
    pub features: serde_json::Value,
    pub sort_order: i32,
}

impl SubscriptionPlan {
    pub async fn list(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_plans ORDER BY sort_order")
            .fetch_all(db).await
    }

    pub async fn find(db: &PgPool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_plans WHERE id = $1")
            .bind(id).fetch_optional(db).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: String,
    pub status: String,
    pub extra_seats: i32,
    pub extra_storage_gb: i32,
    pub current_period_start: OffsetDateTime,
    pub current_period_end: OffsetDateTime,
    pub cancel_at_period_end: bool,
    pub auto_renew: bool,
    pub monthly_amount: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Subscription {
    pub async fn find_by_user(db: &PgPool, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscriptions WHERE user_id = $1")
            .bind(user_id).fetch_optional(db).await
    }

    /// Create or update a Supporter subscription. The credit deduction is done
    /// by the caller (inside the same DB transaction as the ledger row).
    pub async fn supporter_subscribe<'e, E>(
        executor: E,
        user_id: Uuid,
        monthly_amount: i64,
        auto_renew: bool,
    ) -> Result<Self, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        let period_end = OffsetDateTime::now_utc() + time::Duration::days(30);

        sqlx::query_as(
            "INSERT INTO subscriptions (user_id, plan_id, monthly_amount, current_period_end, auto_renew)
             VALUES ($1, 'supporter', $2, $3, $4)
             ON CONFLICT (user_id) DO UPDATE SET
                plan_id = 'supporter',
                monthly_amount = EXCLUDED.monthly_amount,
                current_period_start = NOW(),
                current_period_end = EXCLUDED.current_period_end,
                status = 'active',
                auto_renew = EXCLUDED.auto_renew,
                cancel_at_period_end = false,
                updated_at = NOW()
             RETURNING *"
        )
        .bind(user_id).bind(monthly_amount).bind(period_end).bind(auto_renew)
        .fetch_one(executor).await
    }

    /// Extend the current period by 30 days (successful renewal).
    pub async fn extend_period<'e, E>(executor: E, user_id: Uuid) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        sqlx::query(
            "UPDATE subscriptions SET current_period_start = NOW(),
             current_period_end = NOW() + INTERVAL '30 days', updated_at = NOW()
             WHERE user_id = $1"
        ).bind(user_id).execute(executor).await?;
        Ok(())
    }

    pub async fn cancel(db: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE subscriptions SET cancel_at_period_end = true, auto_renew = false, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id).execute(db).await?;
        Ok(())
    }

    pub async fn expire(db: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE subscriptions SET status = 'expired', updated_at = NOW() WHERE user_id = $1")
            .bind(user_id).execute(db).await?;
        Ok(())
    }

    /// List all active subscriptions whose period has ended (renewal or expiry due).
    pub async fn list_period_ended(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM subscriptions WHERE status = 'active' AND current_period_end <= NOW()"
        ).fetch_all(db).await
    }

    pub fn is_active(&self) -> bool {
        self.status == "active" && self.current_period_end > OffsetDateTime::now_utc()
    }
}

/// Get the effective daily API limit for a user based on their subscription.
pub async fn daily_api_limit(db: &PgPool, user_id: Uuid) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT p.daily_api_limit FROM subscriptions s
         JOIN subscription_plans p ON p.id = s.plan_id
         WHERE s.user_id = $1 AND s.status = 'active' AND s.current_period_end > NOW()"
    ).bind(user_id).fetch_optional(db).await?;
    Ok(row.map(|r| r.0).unwrap_or(500))
}

/// Get the effective max team members for a user (plan base + extra seats).
pub async fn max_team_members(db: &PgPool, user_id: Uuid) -> Result<i32, sqlx::Error> {
    let row: Option<(i32, i32)> = sqlx::query_as(
        "SELECT p.max_team_members, s.extra_seats FROM subscriptions s
         JOIN subscription_plans p ON p.id = s.plan_id
         WHERE s.user_id = $1 AND s.status = 'active' AND s.current_period_end > NOW()"
    ).bind(user_id).fetch_optional(db).await?;
    Ok(row.map(|r| r.0 + r.1).unwrap_or(0))
}

/// Get the effective max storage in bytes for a user (plan base + extra).
pub async fn max_storage_bytes(db: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
    let row: Option<(i32, i32)> = sqlx::query_as(
        "SELECT p.storage_mb, s.extra_storage_gb FROM subscriptions s
         JOIN subscription_plans p ON p.id = s.plan_id
         WHERE s.user_id = $1 AND s.status = 'active' AND s.current_period_end > NOW()"
    ).bind(user_id).fetch_optional(db).await?;
    Ok(row.map(|r| (r.0 as i64) * 1024 * 1024 + (r.1 as i64) * 1024 * 1024 * 1024).unwrap_or(0))
}

/// Increment and check daily API usage. Returns (current_count, limit).
pub async fn check_and_increment_usage(db: &PgPool, user_id: Uuid) -> Result<(i32, i32), sqlx::Error> {
    let limit = daily_api_limit(db, user_id).await?;

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO api_usage_daily (user_id, date, request_count)
         VALUES ($1, CURRENT_DATE, 1)
         ON CONFLICT (user_id, date) DO UPDATE SET request_count = api_usage_daily.request_count + 1
         RETURNING request_count"
    ).bind(user_id).fetch_one(db).await?;

    Ok((row.0, limit))
}

// ── Auto Top-Up ──

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
