use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Subcategory {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub approved: bool,
    pub submitted_by: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

impl Subcategory {
    /// List approved subcategories for a category.
    pub async fn list_for_category(pool: &PgPool, category_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Subcategory>(
            "SELECT * FROM subcategories WHERE category_id = $1 AND approved = true ORDER BY sort_order, name",
        )
        .bind(category_id)
        .fetch_all(pool)
        .await
    }

    /// List all approved subcategories (for all categories).
    pub async fn list_all_approved(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Subcategory>(
            "SELECT * FROM subcategories WHERE approved = true ORDER BY category_id, sort_order, name",
        )
        .fetch_all(pool)
        .await
    }

    /// Submit a new subcategory for review.
    pub async fn submit(
        pool: &PgPool,
        category_id: Uuid,
        name: &str,
        submitted_by: Uuid,
    ) -> Result<Self, sqlx::Error> {
        let slug = slugify_sub(name);

        // Return existing if found
        if let Some(existing) = sqlx::query_as::<_, Subcategory>(
            "SELECT * FROM subcategories WHERE category_id = $1 AND slug = $2",
        )
        .bind(category_id)
        .bind(&slug)
        .fetch_optional(pool)
        .await?
        {
            return Ok(existing);
        }

        sqlx::query_as::<_, Subcategory>(
            "INSERT INTO subcategories (category_id, name, slug, approved, submitted_by) VALUES ($1, $2, $3, false, $4) RETURNING *",
        )
        .bind(category_id)
        .bind(name)
        .bind(&slug)
        .bind(submitted_by)
        .fetch_one(pool)
        .await
    }

    /// Approve a pending subcategory (admin only).
    pub async fn approve(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE subcategories SET approved = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    /// Delete a subcategory.
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM subcategories WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    /// List pending subcategories for admin review.
    pub async fn list_pending(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Subcategory>(
            "SELECT * FROM subcategories WHERE approved = false ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }
}

fn slugify_sub(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
