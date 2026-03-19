use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub sort_order: i32,
    pub max_file_size_mb: i32,
    pub allowed_extensions: Vec<String>,
    pub created_at: OffsetDateTime,
}

impl Category {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY sort_order")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        slug: &str,
        description: &str,
        icon: &str,
        max_file_size_mb: i32,
        allowed_extensions: &[String],
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let sort = sqlx::query_as::<_, (i32,)>("SELECT COALESCE(MAX(sort_order), 0)::int + 1 FROM categories")
            .fetch_one(pool)
            .await?;

        sqlx::query_as::<_, Category>(
            "INSERT INTO categories (id, name, slug, description, icon, sort_order, max_file_size_mb, allowed_extensions) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"
        )
        .bind(id).bind(name).bind(slug).bind(description).bind(icon).bind(sort.0).bind(max_file_size_mb).bind(allowed_extensions)
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        icon: Option<&str>,
        max_file_size_mb: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "UPDATE categories SET name=COALESCE($2,name), description=COALESCE($3,description), icon=COALESCE($4,icon), max_file_size_mb=COALESCE($5,max_file_size_mb) WHERE id=$1 RETURNING *"
        )
        .bind(id).bind(name).bind(description).bind(icon).bind(max_file_size_mb)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM categories WHERE id = $1").bind(id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}
