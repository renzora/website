use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct AssetFile {
    pub id: Uuid,
    pub asset_id: Uuid,
    /// The release this file belongs to. Only ever `None` for rows that
    /// predate the release system and somehow escaped the backfill.
    pub release_id: Option<Uuid>,
    pub file_key: String,
    pub preview_key: Option<String>,
    pub original_filename: String,
    /// Path inside the uploaded archive (`docs/install.md`). Equal to the
    /// filename for uploads that weren't a directory tree.
    pub path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub sort_order: i32,
    /// True when this row is an *entry inside* the archive at `file_key`
    /// rather than a stored object of its own. Indexed for browsing; never a
    /// deliverable. See migration 053.
    pub archived: bool,
    /// Cached text of public doc files (markdown, licences), so a README can
    /// be rendered without a round-trip to object storage. `None` means not
    /// cached — the reader fetches and backfills it.
    pub text_content: Option<String>,
    pub created_at: OffsetDateTime,
}

const COLS: &str = "id, asset_id, release_id, file_key, preview_key, original_filename, path, file_size, mime_type, sort_order, archived, text_content, created_at";

impl AssetFile {
    /// Files of the asset's *current* release — what the asset page, previews
    /// and a plain download all mean by "the files".
    ///
    /// Rows with no release are included so an asset can never lose its files
    /// to a half-applied backfill.
    pub async fn list_by_asset(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT f.id, f.asset_id, f.release_id, f.file_key, f.preview_key, f.original_filename,
                    f.path, f.file_size, f.mime_type, f.sort_order, f.archived, f.text_content, f.created_at
             FROM asset_files f
             LEFT JOIN asset_releases r ON r.id = f.release_id
             WHERE f.asset_id = $1 AND (f.release_id IS NULL OR r.is_current)
               AND NOT f.archived
             ORDER BY f.sort_order, f.path"
        ))
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    /// The release's deliverables — what a download hands over.
    pub async fn list_by_release(pool: &PgPool, release_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM asset_files WHERE release_id = $1 AND NOT archived
             ORDER BY sort_order, path"
        ))
        .bind(release_id)
        .fetch_all(pool)
        .await
    }

    /// What the file browser shows for a release: the entries indexed inside a
    /// stored archive if there are any, otherwise the deliverables.
    ///
    /// A plugin has both — the zip it ships as, and the source tree inside it —
    /// and the tree is the useful view.
    pub async fn list_release_tree(
        pool: &PgPool,
        release_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let archived = sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM asset_files WHERE release_id = $1 AND archived
             ORDER BY sort_order, path"
        ))
        .bind(release_id)
        .fetch_all(pool)
        .await?;

        if archived.is_empty() {
            return Self::list_by_release(pool, release_id).await;
        }
        Ok(archived)
    }

    /// One browsable file of a release, addressed by its path.
    pub async fn find_by_path(
        pool: &PgPool,
        release_id: Uuid,
        path: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        // Prefer an indexed archive entry over a deliverable of the same name,
        // matching what `list_release_tree` shows.
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM asset_files WHERE release_id = $1 AND path = $2
             ORDER BY archived DESC LIMIT 1"
        ))
        .bind(release_id)
        .bind(path)
        .fetch_optional(pool)
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert(
        pool: &PgPool,
        asset_id: Uuid,
        release_id: Option<Uuid>,
        file_key: &str,
        preview_key: Option<&str>,
        original_filename: &str,
        path: &str,
        file_size: i64,
        mime_type: &str,
        sort_order: i32,
        archived: bool,
        text_content: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO asset_files (asset_id, release_id, file_key, preview_key, original_filename, path, file_size, mime_type, sort_order, archived, text_content)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING {COLS}"
        ))
        .bind(asset_id)
        .bind(release_id)
        .bind(file_key)
        .bind(preview_key)
        .bind(original_filename)
        .bind(path)
        .bind(file_size)
        .bind(mime_type)
        .bind(sort_order)
        .bind(archived)
        .bind(text_content)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!("SELECT {COLS} FROM asset_files WHERE id = $1"))
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn delete_by_asset(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        // Return rows before deleting so we can clean up storage
        sqlx::query_as::<_, Self>(&format!(
            "DELETE FROM asset_files WHERE asset_id = $1 RETURNING {COLS}"
        ))
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    /// Whether this release's archive has already been indexed.
    pub async fn has_indexed(pool: &PgPool, release_id: Uuid) -> Result<bool, sqlx::Error> {
        let (n,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*)::bigint FROM asset_files WHERE release_id = $1 AND archived",
        )
        .bind(release_id)
        .fetch_one(pool)
        .await?;
        Ok(n > 0)
    }

    /// Insert an entry indexed inside a stored archive.
    ///
    /// Separate from `insert` because indexing can happen lazily on a read, and
    /// two concurrent readers may index the same archive at once — so a repeat
    /// is ignored rather than duplicating the tree.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_indexed(
        pool: &PgPool,
        asset_id: Uuid,
        release_id: Uuid,
        container_key: &str,
        original_filename: &str,
        path: &str,
        file_size: i64,
        mime_type: &str,
        sort_order: i32,
        text_content: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO asset_files
                (asset_id, release_id, file_key, original_filename, path, file_size, mime_type, sort_order, archived, text_content)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, $9)
             ON CONFLICT (release_id, path) WHERE archived DO NOTHING",
        )
        .bind(asset_id)
        .bind(release_id)
        .bind(container_key)
        .bind(original_filename)
        .bind(path)
        .bind(file_size)
        .bind(mime_type)
        .bind(sort_order)
        .bind(text_content)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Fill in `text_content` for a doc file that predates the cache.
    pub async fn cache_text(pool: &PgPool, id: Uuid, text: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_files SET text_content = $2 WHERE id = $1")
            .bind(id)
            .bind(text)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Drop just one release's files, leaving the other releases intact.
    pub async fn delete_by_release(
        pool: &PgPool,
        release_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "DELETE FROM asset_files WHERE release_id = $1 RETURNING {COLS}"
        ))
        .bind(release_id)
        .fetch_all(pool)
        .await
    }
}
