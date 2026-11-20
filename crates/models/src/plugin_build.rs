//! Whether a plugin release compiled against an official engine build. See
//! `migrations/060_plugin_builds.sql`.

use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct PluginBuild {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub release_id: Option<Uuid>,
    pub plugin_version: String,
    pub platform: String,
    pub engine_stamp: String,
    pub engine_tag: String,
    pub ok: bool,
    pub summary: String,
    pub log: String,
    pub created_at: OffsetDateTime,
}

pub struct NewBuild<'a> {
    pub asset_id: Uuid,
    pub release_id: Option<Uuid>,
    pub plugin_version: &'a str,
    pub platform: &'a str,
    pub engine_stamp: &'a str,
    pub engine_tag: &'a str,
    pub ok: bool,
    pub summary: &'a str,
    pub log: &'a str,
}

const COLS: &str = "id, asset_id, release_id, plugin_version, platform, engine_stamp, engine_tag, \
                    ok, summary, log, created_at";

impl PluginBuild {
    /// Record a result, replacing any for the same plugin, platform and build.
    pub async fn upsert(pool: &PgPool, new: &NewBuild<'_>) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO plugin_builds
                (asset_id, release_id, plugin_version, platform, engine_stamp, engine_tag, ok, summary, log)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (asset_id, platform, engine_stamp) DO UPDATE SET
                release_id = EXCLUDED.release_id,
                plugin_version = EXCLUDED.plugin_version,
                engine_tag = EXCLUDED.engine_tag,
                ok = EXCLUDED.ok,
                summary = EXCLUDED.summary,
                log = EXCLUDED.log,
                created_at = NOW()
             RETURNING {COLS}"
        ))
        .bind(new.asset_id)
        .bind(new.release_id)
        .bind(new.plugin_version)
        .bind(new.platform)
        .bind(new.engine_stamp)
        .bind(new.engine_tag)
        .bind(new.ok)
        .bind(new.summary)
        .bind(new.log)
        .fetch_one(pool)
        .await
    }

    /// The newest result for each platform of `asset_id`.
    pub async fn latest_per_platform(pool: &PgPool, asset_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT DISTINCT ON (platform) {COLS} FROM plugin_builds
             WHERE asset_id = $1
             ORDER BY platform, created_at DESC"
        ))
        .bind(asset_id)
        .fetch_all(pool)
        .await
    }
}
