use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Voucher {
    pub id: Uuid,
    pub code: String,
    pub voucher_type: String,
    pub credit_amount: Option<i64>,
    pub discount_percent: Option<i32>,
    pub max_asset_price: Option<i64>,
    pub specific_asset_id: Option<Uuid>,
    pub max_uses: Option<i32>,
    pub max_uses_per_user: i32,
    pub times_used: i32,
    pub active: bool,
    pub expires_at: Option<OffsetDateTime>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct VoucherUse {
    pub id: Uuid,
    pub voucher_id: Uuid,
    pub user_id: Uuid,
    pub credit_amount: Option<i64>,
    pub asset_id: Option<Uuid>,
    pub used_at: OffsetDateTime,
}

impl Voucher {
    pub async fn create(
        pool: &PgPool,
        code: &str,
        voucher_type: &str,
        credit_amount: Option<i64>,
        discount_percent: Option<i32>,
        max_asset_price: Option<i64>,
        specific_asset_id: Option<Uuid>,
        max_uses: Option<i32>,
        max_uses_per_user: i32,
        expires_at: Option<OffsetDateTime>,
        created_by: Uuid,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO vouchers (code, voucher_type, credit_amount, discount_percent, max_asset_price, specific_asset_id, max_uses, max_uses_per_user, expires_at, created_by) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *"
        )
        .bind(code.to_uppercase())
        .bind(voucher_type)
        .bind(credit_amount)
        .bind(discount_percent)
        .bind(max_asset_price)
        .bind(specific_asset_id)
        .bind(max_uses)
        .bind(max_uses_per_user)
        .bind(expires_at)
        .bind(created_by)
        .fetch_one(pool)
        .await
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM vouchers ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_valid(pool: &PgPool, code: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM vouchers WHERE code = $1 AND active = true AND (max_uses IS NULL OR times_used < max_uses) AND (expires_at IS NULL OR expires_at > NOW())"
        )
        .bind(code.to_uppercase())
        .fetch_optional(pool)
        .await
    }

    pub async fn check_user_usage(pool: &PgPool, voucher_id: Uuid, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM voucher_uses WHERE voucher_id = $1 AND user_id = $2")
            .bind(voucher_id).bind(user_id).fetch_one(pool).await?;
        Ok(row.0)
    }

    pub async fn set_active(pool: &PgPool, id: Uuid, active: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE vouchers SET active = $1 WHERE id = $2")
            .bind(active).bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM voucher_uses WHERE voucher_id = $1").bind(id).execute(pool).await?;
        sqlx::query("DELETE FROM vouchers WHERE id = $1").bind(id).execute(pool).await?;
        Ok(())
    }
}
