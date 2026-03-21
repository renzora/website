use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PromoCode {
    pub id: Uuid,
    pub code: String,
    pub discount_percent: i32,
    pub max_uses: Option<i32>,
    pub times_used: i32,
    pub active: bool,
    pub expires_at: Option<OffsetDateTime>,
    pub created_by: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

impl PromoCode {
    /// Create a new promo code.
    pub async fn create(
        pool: &PgPool,
        code: &str,
        discount_percent: i32,
        max_uses: Option<i32>,
        expires_at: Option<OffsetDateTime>,
        created_by: Uuid,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, PromoCode>(
            r#"
            INSERT INTO promo_codes (code, discount_percent, max_uses, expires_at, created_by)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(code.to_uppercase())
        .bind(discount_percent.clamp(0, 20))
        .bind(max_uses)
        .bind(expires_at)
        .bind(created_by)
        .fetch_one(pool)
        .await
    }

    /// List all promo codes (admin view).
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, PromoCode>(
            "SELECT * FROM promo_codes ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Find an active, valid promo code by its code string.
    pub async fn find_valid(pool: &PgPool, code: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, PromoCode>(
            r#"
            SELECT * FROM promo_codes
            WHERE code = $1
              AND active = true
              AND (max_uses IS NULL OR times_used < max_uses)
              AND (expires_at IS NULL OR expires_at > NOW())
            "#,
        )
        .bind(code.to_uppercase())
        .fetch_optional(pool)
        .await
    }

    /// Increment usage count and record who used it.
    pub async fn record_use(
        pool: &PgPool,
        promo_id: Uuid,
        user_id: Uuid,
        asset_id: Uuid,
        discount_applied: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE promo_codes SET times_used = times_used + 1 WHERE id = $1",
        )
        .bind(promo_id)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO promo_code_uses (promo_code_id, user_id, asset_id, discount_applied) VALUES ($1, $2, $3, $4)",
        )
        .bind(promo_id)
        .bind(user_id)
        .bind(asset_id)
        .bind(discount_applied)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Toggle active status.
    pub async fn set_active(pool: &PgPool, id: Uuid, active: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE promo_codes SET active = $1 WHERE id = $2")
            .bind(active)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Delete a promo code.
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM promo_code_uses WHERE promo_code_id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        sqlx::query("DELETE FROM promo_codes WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
