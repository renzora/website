use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// One published version of an asset. Files hang off a release, not off the
/// asset, so a new release never invalidates what an earlier buyer downloaded.
#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct AssetRelease {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub version: String,
    pub notes: String,
    pub is_current: bool,
    pub downloads: i64,
    pub created_at: OffsetDateTime,
}

/// A release plus the rolled-up size of its files, for listings.
#[derive(Debug, sqlx::FromRow)]
pub struct AssetReleaseWithStats {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub version: String,
    pub notes: String,
    pub is_current: bool,
    pub downloads: i64,
    pub created_at: OffsetDateTime,
    /// Counts describe the *browsable* set — the entries indexed inside a
    /// stored archive when there are any, otherwise the deliverables. That is
    /// what the file tree shows, so it is what a release listing should say.
    pub file_count: i64,
    pub total_size: i64,
}

const COLS: &str = "id, asset_id, version, notes, is_current, downloads, created_at";

impl AssetRelease {
    /// Newest first, with file counts and total size attached.
    pub async fn list_by_asset(
        pool: &PgPool,
        asset_id: Uuid,
    ) -> Result<Vec<AssetReleaseWithStats>, sqlx::Error> {
        sqlx::query_as::<_, AssetReleaseWithStats>(
            "SELECT r.id, r.asset_id, r.version, r.notes, r.is_current, r.downloads, r.created_at,
                    CASE WHEN COUNT(f.id) FILTER (WHERE f.archived) > 0
                         THEN COUNT(f.id) FILTER (WHERE f.archived)
                         ELSE COUNT(f.id) FILTER (WHERE NOT f.archived)
                    END::bigint AS file_count,
                    CASE WHEN COUNT(f.id) FILTER (WHERE f.archived) > 0
                         THEN COALESCE(SUM(f.file_size) FILTER (WHERE f.archived), 0)
                         ELSE COALESCE(SUM(f.file_size) FILTER (WHERE NOT f.archived), 0)
                    END::bigint AS total_size
             FROM asset_releases r
             LEFT JOIN asset_files f ON f.release_id = r.id
             WHERE r.asset_id = $1
             GROUP BY r.id
             ORDER BY r.created_at DESC",
        )
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_current(pool: &PgPool, asset_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM asset_releases WHERE asset_id = $1 AND is_current"
        ))
        .bind(asset_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!("SELECT {COLS} FROM asset_releases WHERE id = $1"))
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_version(
        pool: &PgPool,
        asset_id: Uuid,
        version: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM asset_releases WHERE asset_id = $1 AND version = $2"
        ))
        .bind(asset_id)
        .bind(version)
        .fetch_optional(pool)
        .await
    }

    /// Resolve a `?release=` parameter: a release id, a version string, or
    /// nothing at all (meaning the current release).
    pub async fn resolve(
        pool: &PgPool,
        asset_id: Uuid,
        selector: Option<&str>,
    ) -> Result<Option<Self>, sqlx::Error> {
        let Some(sel) = selector.map(str::trim).filter(|s| !s.is_empty()) else {
            return Self::find_current(pool, asset_id).await;
        };
        if let Ok(id) = Uuid::parse_str(sel) {
            // An id from another asset must not resolve here.
            if let Some(r) = Self::find_by_id(pool, id).await? {
                return Ok((r.asset_id == asset_id).then_some(r));
            }
            return Ok(None);
        }
        Self::find_by_version(pool, asset_id, sel).await
    }

    /// Create a release and make it the current one, demoting the previous
    /// current release in the same transaction (the partial unique index only
    /// allows one, so the demotion has to land first).
    pub async fn create_current(
        pool: &PgPool,
        asset_id: Uuid,
        version: &str,
        notes: &str,
    ) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("UPDATE asset_releases SET is_current = false WHERE asset_id = $1 AND is_current")
            .bind(asset_id)
            .execute(&mut *tx)
            .await?;

        let release = sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO asset_releases (asset_id, version, notes, is_current)
             VALUES ($1, $2, $3, true)
             RETURNING {COLS}"
        ))
        .bind(asset_id)
        .bind(version)
        .bind(notes)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(release)
    }

    /// Rename a release. Used to keep the current release in step when the
    /// creator edits the asset's version field.
    pub async fn update_version(
        pool: &PgPool,
        id: Uuid,
        version: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "UPDATE asset_releases SET version = $2 WHERE id = $1 RETURNING {COLS}"
        ))
        .bind(id)
        .bind(version)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_notes(
        pool: &PgPool,
        id: Uuid,
        notes: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "UPDATE asset_releases SET notes = $2 WHERE id = $1 RETURNING {COLS}"
        ))
        .bind(id)
        .bind(notes)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM asset_releases WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_downloads(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE asset_releases SET downloads = downloads + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn count_for_asset(pool: &PgPool, asset_id: Uuid) -> Result<i64, sqlx::Error> {
        let (n,): (i64,) =
            sqlx::query_as("SELECT COUNT(*)::bigint FROM asset_releases WHERE asset_id = $1")
                .bind(asset_id)
                .fetch_one(pool)
                .await?;
        Ok(n)
    }
}
