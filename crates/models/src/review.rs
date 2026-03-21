use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Review {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub author_id: Uuid,
    pub rating: i32,
    pub title: String,
    pub content: String,
    pub helpful_count: i32,
    pub flagged: bool,
    pub flag_reason: Option<String>,
    pub hidden: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ReviewWithAuthor {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub author_id: Uuid,
    pub rating: i32,
    pub title: String,
    pub content: String,
    pub helpful_count: i32,
    pub flagged: bool,
    pub hidden: bool,
    pub created_at: OffsetDateTime,
    pub author_name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct FlaggedReview {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub author_id: Uuid,
    pub rating: i32,
    pub title: String,
    pub content: String,
    pub flag_reason: Option<String>,
    pub hidden: bool,
    pub created_at: OffsetDateTime,
    pub author_name: String,
    pub asset_name: String,
}

impl Review {
    /// Submit or update a review. Updates asset rating summary atomically.
    pub async fn upsert(
        pool: &PgPool,
        asset_id: Uuid,
        author_id: Uuid,
        rating: i32,
        title: &str,
        content: &str,
    ) -> Result<Self, String> {
        let rating = rating.clamp(1, 5);
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Check if review already exists
        let existing: Option<(Uuid, i32)> = sqlx::query_as(
            "SELECT id, rating FROM asset_reviews WHERE asset_id = $1 AND author_id = $2",
        )
        .bind(asset_id)
        .bind(author_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let review = if let Some((existing_id, old_rating)) = existing {
            // Update existing review
            let r = sqlx::query_as::<_, Review>(
                "UPDATE asset_reviews SET rating = $1, title = $2, content = $3, updated_at = NOW(), flagged = false, flag_reason = NULL WHERE id = $4 RETURNING *",
            )
            .bind(rating)
            .bind(title)
            .bind(content)
            .bind(existing_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            // Adjust asset rating: remove old, add new
            sqlx::query(
                "UPDATE assets SET rating_sum = rating_sum - $1 + $2, updated_at = NOW() WHERE id = $3",
            )
            .bind(old_rating as i64)
            .bind(rating as i64)
            .bind(asset_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            r
        } else {
            // Insert new review
            let r = sqlx::query_as::<_, Review>(
                "INSERT INTO asset_reviews (asset_id, author_id, rating, title, content) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            )
            .bind(asset_id)
            .bind(author_id)
            .bind(rating)
            .bind(title)
            .bind(content)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            // Increment asset rating
            sqlx::query(
                "UPDATE assets SET rating_sum = rating_sum + $1, rating_count = rating_count + 1, updated_at = NOW() WHERE id = $2",
            )
            .bind(rating as i64)
            .bind(asset_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            r
        };

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(review)
    }

    /// List visible reviews for an asset.
    pub async fn list_for_asset(
        pool: &PgPool,
        asset_id: Uuid,
    ) -> Result<Vec<ReviewWithAuthor>, sqlx::Error> {
        sqlx::query_as::<_, ReviewWithAuthor>(
            r#"
            SELECT r.id, r.asset_id, r.author_id, r.rating, r.title, r.content,
                   r.helpful_count, r.flagged, r.hidden, r.created_at, u.username AS author_name
            FROM asset_reviews r
            JOIN users u ON u.id = r.author_id
            WHERE r.asset_id = $1 AND r.hidden = false
            ORDER BY r.helpful_count DESC, r.created_at DESC
            "#,
        )
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    /// Flag a review for moderation.
    pub async fn flag(pool: &PgPool, id: Uuid, reason: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_reviews SET flagged = true, flag_reason = $1 WHERE id = $2")
            .bind(reason)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Mark a review as helpful (increment counter).
    pub async fn mark_helpful(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_reviews SET helpful_count = helpful_count + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Admin: hide a review.
    pub async fn set_hidden(pool: &PgPool, id: Uuid, hidden: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_reviews SET hidden = $1, flagged = false WHERE id = $2")
            .bind(hidden)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Admin: dismiss flag (unflag without hiding).
    pub async fn dismiss_flag(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_reviews SET flagged = false, flag_reason = NULL WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Admin: list flagged reviews.
    pub async fn list_flagged(pool: &PgPool) -> Result<Vec<FlaggedReview>, sqlx::Error> {
        sqlx::query_as::<_, FlaggedReview>(
            r#"
            SELECT r.id, r.asset_id, r.author_id, r.rating, r.title, r.content,
                   r.flag_reason, r.hidden, r.created_at,
                   u.username AS author_name, a.name AS asset_name
            FROM asset_reviews r
            JOIN users u ON u.id = r.author_id
            JOIN assets a ON a.id = r.asset_id
            WHERE r.flagged = true
            ORDER BY r.created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Delete a review and update asset rating summary.
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
        let review = sqlx::query_as::<_, Review>("SELECT * FROM asset_reviews WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Review not found")?;

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query("DELETE FROM asset_reviews WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(
            "UPDATE assets SET rating_sum = GREATEST(rating_sum - $1, 0), rating_count = GREATEST(rating_count - 1, 0), updated_at = NOW() WHERE id = $2",
        )
        .bind(review.rating as i64)
        .bind(review.asset_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
