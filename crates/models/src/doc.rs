use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Doc {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub sort_order: i32,
    pub published: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Doc {
    pub async fn list_published(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Doc>(
            "SELECT * FROM docs WHERE published = true ORDER BY category, sort_order, title",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_category(
        pool: &PgPool,
        category: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Doc>(
            "SELECT * FROM docs WHERE published = true AND category = $1 ORDER BY sort_order, title",
        )
        .bind(category)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Doc>(
            "SELECT * FROM docs WHERE slug = $1 AND published = true",
        )
        .bind(slug)
        .fetch_optional(pool)
        .await
    }

    pub async fn categories(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT DISTINCT category FROM docs WHERE published = true ORDER BY category",
        )
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    pub async fn search(pool: &PgPool, query: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Doc>(
            r#"
            SELECT * FROM docs
            WHERE published = true
              AND (title ILIKE '%' || $1 || '%' OR content ILIKE '%' || $1 || '%')
            ORDER BY sort_order, title
            "#,
        )
        .bind(query)
        .fetch_all(pool)
        .await
    }
}
