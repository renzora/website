use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Dispute {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub reason: String,
    pub status: String,
    pub admin_notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub resolved_at: Option<OffsetDateTime>,
}

impl Dispute {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        asset_id: Option<Uuid>,
        transaction_id: Option<Uuid>,
        reason: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Dispute>(
            "INSERT INTO disputes (id, user_id, asset_id, transaction_id, reason) VALUES ($1,$2,$3,$4,$5) RETURNING *"
        )
        .bind(id).bind(user_id).bind(asset_id).bind(transaction_id).bind(reason)
        .fetch_one(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Self>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let disputes = sqlx::query_as::<_, Dispute>(
            "SELECT * FROM disputes WHERE ($1::text IS NULL OR status = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(status).bind(per_page).bind(offset).fetch_all(pool).await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*)::bigint FROM disputes WHERE ($1::text IS NULL OR status = $1)"
        ).bind(status).fetch_one(pool).await?;

        Ok((disputes, total.0))
    }

    pub async fn resolve(pool: &PgPool, id: Uuid, status: &str, admin_notes: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Dispute>(
            "UPDATE disputes SET status=$2, admin_notes=$3, resolved_at=NOW() WHERE id=$1 RETURNING *"
        ).bind(id).bind(status).bind(admin_notes).fetch_one(pool).await
    }
}

pub async fn get_setting(pool: &PgPool, key: &str) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM site_settings WHERE key = $1")
        .bind(key).fetch_optional(pool).await?;
    Ok(row.map(|r| r.0))
}

pub async fn set_setting(pool: &PgPool, key: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO site_settings (key, value, updated_at) VALUES ($1, $2, NOW()) ON CONFLICT (key) DO UPDATE SET value=$2, updated_at=NOW()")
        .bind(key).bind(value).execute(pool).await?;
    Ok(())
}

pub async fn list_settings(pool: &PgPool) -> Result<Vec<(String, String)>, sqlx::Error> {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM site_settings ORDER BY key")
        .fetch_all(pool).await?;
    Ok(rows)
}
