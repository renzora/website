use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub approved: bool,
    pub submitted_by: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

impl Tag {
    /// Search approved tags by prefix (for autocomplete).
    pub async fn search(pool: &PgPool, query: &str, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE approved = true AND name ILIKE $1 || '%' ORDER BY name LIMIT $2",
        )
        .bind(query)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    /// List all approved tags.
    pub async fn list_approved(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE approved = true ORDER BY name")
            .fetch_all(pool)
            .await
    }

    /// Find a tag by slug.
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    /// Submit a new tag for review. Returns the tag (approved=false).
    /// If it already exists, returns the existing one.
    pub async fn submit(
        pool: &PgPool,
        name: &str,
        submitted_by: Uuid,
    ) -> Result<Self, sqlx::Error> {
        let slug = slugify_tag(name);

        // Return existing if found
        if let Some(existing) = Self::find_by_slug(pool, &slug).await? {
            return Ok(existing);
        }

        sqlx::query_as::<_, Tag>(
            "INSERT INTO tags (name, slug, approved, submitted_by) VALUES ($1, $2, false, $3) RETURNING *",
        )
        .bind(name)
        .bind(&slug)
        .bind(submitted_by)
        .fetch_one(pool)
        .await
    }

    /// Approve a pending tag (admin only).
    pub async fn approve(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE tags SET approved = true WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    /// Delete a tag.
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }

    /// List pending (unapproved) tags for admin review.
    pub async fn list_pending(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE approved = false ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }
}

fn slugify_tag(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
