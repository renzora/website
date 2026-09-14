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
    /// Oldest engine this release runs on. `None` means any.
    ///
    /// On the release, not the listing, because a listing outlives the engine
    /// versions it was built for: r1-alpha7 users must keep being offered the
    /// r1-alpha7 release after an r1-alpha8 one exists. There is deliberately no
    /// maximum -- the ceiling is implied by the next release that declares a
    /// higher engine version, so it cannot go stale.
    pub min_engine_version: Option<String>,
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
    /// Carried through so a release list can label each row with the engine it
    /// was built for, which is the whole point of the value being per-release.
    pub min_engine_version: Option<String>,
    /// Counts describe the *browsable* set — the entries indexed inside a
    /// stored archive when there are any, otherwise the deliverables. That is
    /// what the file tree shows, so it is what a release listing should say.
    pub file_count: i64,
    pub total_size: i64,
}

const COLS: &str =
    "id, asset_id, version, notes, is_current, downloads, created_at, min_engine_version";

/// The same columns, qualified.
///
/// Needed by any query that joins `engine_versions`, which has its own `version`
/// column: the bare list above is ambiguous there, and the failure is a runtime
/// error rather than a compile one, because these queries are built as strings.
const COLS_R: &str = "r.id, r.asset_id, r.version, r.notes, r.is_current, r.downloads, \
                      r.created_at, r.min_engine_version";

impl AssetRelease {
    /// Newest first, with file counts and total size attached.
    pub async fn list_by_asset(
        pool: &PgPool,
        asset_id: Uuid,
    ) -> Result<Vec<AssetReleaseWithStats>, sqlx::Error> {
        sqlx::query_as::<_, AssetReleaseWithStats>(
            "SELECT r.id, r.asset_id, r.version, r.notes, r.is_current, r.downloads, r.created_at,
                    r.min_engine_version,
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

    /// The release an engine on `engine_version` should be offered: the newest
    /// one built for an engine no newer than it.
    ///
    /// This replaces "the current release" everywhere a specific engine is
    /// asking. `is_current` is one boolean per listing, so it can only ever
    /// describe the newest release overall, which is precisely the assumption
    /// that cut r1-alpha7 users off from the r1-alpha7 release still sitting in
    /// this table.
    ///
    /// Two details carry the whole behaviour.
    ///
    /// **Ordered by version, never by date.** A fix published to the r1-alpha7
    /// line today is newer in time and older in version than last month's
    /// r1-alpha8 release. `ORDER BY created_at` would hand r1-alpha8 users the
    /// r1-alpha7 code, which is the failure this exists to prevent. `semver_key`
    /// (migration 057) also compares `1.0.10` against `1.0.11` as numbers, which
    /// a string compare gets backwards.
    ///
    /// **Ordered by the engine's `ordinal`, never by its name**, because
    /// `r1-alpha10` sorts below `r1-alpha7` as text.
    ///
    /// `None` is an answer, not an error: a listing whose every release needs a
    /// newer engine has nothing to offer this caller yet, and the caller should
    /// say so rather than fall back to something that will not run.
    ///
    /// A release with no `min_engine_version` is offered to everyone. That is
    /// the honest reading of "nobody has said otherwise", and it is what cargo
    /// and npm do rather than freezing a catalogue on every toolchain release.
    pub async fn resolve_for_engine(
        pool: &PgPool,
        asset_id: Uuid,
        engine_version: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(&format!(
            "SELECT {COLS_R} FROM asset_releases r
             LEFT JOIN engine_versions ev ON ev.version = r.min_engine_version
             WHERE r.asset_id = $1
               AND (r.min_engine_version IS NULL
                    OR ev.ordinal <= (SELECT ordinal FROM engine_versions
                                      WHERE version = $2))
             ORDER BY semver_key(r.version) DESC
             LIMIT 1"
        ))
        .bind(asset_id)
        .bind(engine_version)
        .fetch_optional(pool)
        .await
    }

    /// [`resolve_for_engine`](Self::resolve_for_engine) for many listings at
    /// once, which is what an update check is.
    ///
    /// The editor asks about every installed plugin on startup, so this must not
    /// be a query per plugin: at 200 ids that is 200 round trips for something
    /// that is almost always "no change".
    ///
    /// `max_ordinal` is the caller's engine position, and passing [`i32::MAX`]
    /// means "no ceiling" -- the newest release of each listing regardless of
    /// engine. That is deliberately the same code path rather than a second
    /// query with the filter removed, because an update check needs both answers
    /// (what you can have, and what exists) and they must be produced the same
    /// way or they can disagree about ordering.
    ///
    /// A listing with nothing to offer is absent from the result rather than
    /// present with a null, so a caller iterates what it got instead of
    /// filtering what it did not.
    pub async fn resolve_many_for_engine(
        pool: &PgPool,
        asset_ids: &[Uuid],
        max_ordinal: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }
        sqlx::query_as::<_, Self>(&format!(
            "SELECT DISTINCT ON (r.asset_id) {COLS_R}
             FROM asset_releases r
             LEFT JOIN engine_versions ev ON ev.version = r.min_engine_version
             WHERE r.asset_id = ANY($1)
               AND (r.min_engine_version IS NULL OR ev.ordinal <= $2)
             ORDER BY r.asset_id, semver_key(r.version) DESC"
        ))
        .bind(asset_ids)
        .bind(max_ordinal)
        .fetch_all(pool)
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
    /// `min_engine_version` is `None` for "any engine", and unknown values are
    /// rejected by the foreign key rather than stored: a typo in a manifest
    /// should fail the publish, not create a release nothing can ever resolve.
    ///
    /// `is_current` still marks the newest release overall, and is now only good
    /// for display. Which release a given engine is offered is
    /// [`resolve_for_engine`](Self::resolve_for_engine), because "current"
    /// stopped being one value the day compatibility moved onto the release.
    pub async fn create_current(
        pool: &PgPool,
        asset_id: Uuid,
        version: &str,
        notes: &str,
        min_engine_version: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("UPDATE asset_releases SET is_current = false WHERE asset_id = $1 AND is_current")
            .bind(asset_id)
            .execute(&mut *tx)
            .await?;

        let release = sqlx::query_as::<_, Self>(&format!(
            "INSERT INTO asset_releases (asset_id, version, notes, is_current, min_engine_version)
             VALUES ($1, $2, $3, true, $4)
             RETURNING {COLS}"
        ))
        .bind(asset_id)
        .bind(version)
        .bind(notes)
        .bind(min_engine_version)
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
