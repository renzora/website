//! Plugins compiled by the engine's CI for one exact engine build. See
//! `migrations/059_prebuilt_plugins.sql` for why the engine stamp, not the
//! engine version, is what a binary is filed under.

use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct PrebuiltPlugin {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub release_id: Option<Uuid>,
    pub plugin_version: String,
    pub platform: String,
    pub engine_stamp: String,
    pub engine_tag: String,
    pub engine_version: Option<String>,
    pub file_name: String,
    pub blob_sha: String,
    pub file_key: String,
    pub byte_size: i64,
    pub sha256: String,
    pub signature: String,
    pub downloads: i64,
    pub created_at: OffsetDateTime,
}

/// One engine build a plugin has binaries for.
#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct EngineBuild {
    /// The engine release or nightly (`r1-alpha8-nightly-22nov26`).
    pub engine_tag: String,
    pub engine_version: Option<String>,
    /// The plugin version built for it.
    pub plugin_version: Option<String>,
    /// How many platforms have a binary for it.
    pub platforms: i64,
    pub built_at: OffsetDateTime,
}

/// What an upload records. Everything but the storage location comes from the
/// statement CI signed.
pub struct NewPrebuilt<'a> {
    pub asset_id: Uuid,
    pub release_id: Option<Uuid>,
    pub plugin_version: &'a str,
    pub platform: &'a str,
    pub engine_stamp: &'a str,
    pub engine_tag: &'a str,
    pub engine_version: Option<&'a str>,
    pub file_name: &'a str,
    pub blob_sha: &'a str,
    pub file_key: &'a str,
    pub byte_size: i64,
    pub sha256: &'a str,
    pub signature: &'a str,
}

const COLS: &str = "id, asset_id, release_id, plugin_version, platform, engine_stamp, engine_tag, \
                    engine_version, file_name, blob_sha, file_key, byte_size, sha256, signature, \
                    downloads, created_at";

impl PrebuiltPlugin {
    /// Record a binary, replacing any for the same plugin, platform and build:
    /// a re-run of a nightly rebuilds the same thing and should not add a row.
    pub async fn upsert(pool: &PgPool, new: &NewPrebuilt<'_>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO prebuilt_plugins
                (asset_id, release_id, plugin_version, platform, engine_stamp, engine_tag,
                 engine_version, file_name, blob_sha, file_key, byte_size, sha256, signature)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
             ON CONFLICT (asset_id, platform, engine_stamp) DO UPDATE SET
                release_id = EXCLUDED.release_id,
                plugin_version = EXCLUDED.plugin_version,
                engine_tag = EXCLUDED.engine_tag,
                engine_version = EXCLUDED.engine_version,
                file_name = EXCLUDED.file_name,
                blob_sha = EXCLUDED.blob_sha,
                file_key = EXCLUDED.file_key,
                byte_size = EXCLUDED.byte_size,
                sha256 = EXCLUDED.sha256,
                signature = EXCLUDED.signature,
                created_at = NOW()
             RETURNING {COLS}"
        ))
        .bind(new.asset_id)
        .bind(new.release_id)
        .bind(new.plugin_version)
        .bind(new.platform)
        .bind(new.engine_stamp)
        .bind(new.engine_tag)
        .bind(new.engine_version)
        .bind(new.file_name)
        .bind(new.blob_sha)
        .bind(new.file_key)
        .bind(new.byte_size)
        .bind(new.sha256)
        .bind(new.signature)
        .fetch_one(pool)
        .await
    }

    /// The binary of `asset_id` for exactly this engine build and platform.
    pub async fn find(
        pool: &PgPool,
        asset_id: Uuid,
        engine_stamp: &str,
        platform: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS} FROM prebuilt_plugins
             WHERE asset_id = $1 AND engine_stamp = $2 AND platform = $3"
        ))
        .bind(asset_id)
        .bind(engine_stamp)
        .bind(platform)
        .fetch_optional(pool)
        .await
    }

    /// The newest binary of `asset_id` on each platform, for the listing page,
    /// which cannot know the visitor's engine build and so shows the latest.
    pub async fn latest_per_platform(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT DISTINCT ON (platform) {COLS} FROM prebuilt_plugins
             WHERE asset_id = $1
             ORDER BY platform, created_at DESC"
        ))
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }

    /// The newest binary of `asset_id` on each platform for one engine build,
    /// named by its tag (`r1-alpha8-nightly-22nov26`). By tag rather than
    /// stamp because a build's stamp differs per platform (each platform's SDK
    /// is its own set of images), while its tag is the one name a person picks.
    pub async fn for_engine_tag(pool: &PgPool, asset_id: Uuid, engine_tag: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT DISTINCT ON (platform) {COLS} FROM prebuilt_plugins
             WHERE asset_id = $1 AND engine_tag = $2
             ORDER BY platform, created_at DESC"
        ))
        .bind(asset_id)
        .bind(engine_tag)
        .fetch_all(pool)
        .await
    }

    /// Every engine build `asset_id` has binaries for, newest first: what the
    /// listing page's build picker offers. One plugin version is rebuilt for
    /// every nightly, so the same version appears under many builds.
    pub async fn engine_builds(pool: &PgPool, asset_id: Uuid, limit: i64) -> Result<Vec<EngineBuild>, sqlx::Error> {
        sqlx::query_as::<_, EngineBuild>(
            "SELECT engine_tag,
                    MAX(engine_version) AS engine_version,
                    MAX(plugin_version) AS plugin_version,
                    COUNT(DISTINCT platform) AS platforms,
                    MAX(created_at) AS built_at
             FROM prebuilt_plugins
             WHERE asset_id = $1 AND engine_tag <> ''
             GROUP BY engine_tag
             ORDER BY MAX(created_at) DESC
             LIMIT $2",
        )
        .bind(asset_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!("SELECT {COLS} FROM prebuilt_plugins WHERE id = $1"))
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn increment_downloads(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE prebuilt_plugins SET downloads = downloads + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
