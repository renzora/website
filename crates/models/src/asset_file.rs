use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct AssetFile {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub file_key: String,
    pub preview_key: Option<String>,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

impl AssetFile {
    pub async fn list_by_asset(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, asset_id, file_key, preview_key, original_filename, file_size, mime_type, sort_order, created_at
             FROM asset_files WHERE asset_id = $1 ORDER BY sort_order, created_at",
        )
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    pub async fn insert(
        pool: &PgPool,
        asset_id: Uuid,
        file_key: &str,
        preview_key: Option<&str>,
        original_filename: &str,
        file_size: i64,
        mime_type: &str,
        sort_order: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO asset_files (asset_id, file_key, preview_key, original_filename, file_size, mime_type, sort_order)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, asset_id, file_key, preview_key, original_filename, file_size, mime_type, sort_order, created_at",
        )
        .bind(asset_id)
        .bind(file_key)
        .bind(preview_key)
        .bind(original_filename)
        .bind(file_size)
        .bind(mime_type)
        .bind(sort_order)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, asset_id, file_key, preview_key, original_filename, file_size, mime_type, sort_order, created_at
             FROM asset_files WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_by_asset(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        // Return rows before deleting so we can clean up storage
        let rows = sqlx::query_as::<_, Self>(
            "DELETE FROM asset_files WHERE asset_id = $1
             RETURNING id, asset_id, file_key, preview_key, original_filename, file_size, mime_type, sort_order, created_at",
        )
        .bind(asset_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}
