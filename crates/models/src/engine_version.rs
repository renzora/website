//! The catalogue of engine releases a plugin can declare itself built for.
//!
//! Kept as a table rather than parsed out of the version string, because
//! `r1-alpha10` sorts below `r1-alpha7` as text and every question asked here is
//! an ordering question. `ordinal` is the sort key; the string is a label.

use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct EngineVersion {
    pub version: String,
    pub ordinal: i32,
    /// `None` for a version that exists but has not shipped yet, which is how a
    /// version can be published against before its release date.
    pub released_at: Option<OffsetDateTime>,
}

impl EngineVersion {
    /// Newest first, which is the order a dropdown wants and the order the CLI
    /// prints when it rejects an unknown version.
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT version, ordinal, released_at FROM engine_versions ORDER BY ordinal DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Where a version sits in the ordering, or `None` if it is not one we know.
    pub async fn ordinal_of(pool: &PgPool, version: &str) -> Result<Option<i32>, sqlx::Error> {
        let row: Option<(i32,)> =
            sqlx::query_as("SELECT ordinal FROM engine_versions WHERE version = $1")
                .bind(version)
                .fetch_optional(pool)
                .await?;
        Ok(row.map(|(o,)| o))
    }

    /// Cut a reported build string down to the release it belongs to.
    ///
    /// The engine reports more than the bare version in two cases that both have
    /// to resolve to the same row: a nightly is `r1-alpha8-nightly-06sep26`, and
    /// an untagged local build is `r1-alpha8 (dev)`. Both are r1-alpha8 as far as
    /// plugin compatibility goes, and neither will ever be in this table.
    ///
    /// Anything left unrecognised is returned trimmed and looked up as-is, so a
    /// genuine typo still misses rather than being coerced into a near match.
    pub fn normalize(reported: &str) -> &str {
        let s = reported.trim();
        let end = s
            .find("-nightly")
            .or_else(|| s.find(char::is_whitespace))
            .unwrap_or(s.len());
        s[..end].trim()
    }
}

#[cfg(test)]
mod tests {
    use super::EngineVersion as E;

    #[test]
    fn normalize_strips_what_the_engine_actually_reports() {
        assert_eq!(E::normalize("r1-alpha8"), "r1-alpha8");
        assert_eq!(E::normalize("r1-alpha8-nightly-06sep26"), "r1-alpha8");
        assert_eq!(E::normalize("r1-alpha8 (dev)"), "r1-alpha8");
        assert_eq!(E::normalize("  r1-alpha7  "), "r1-alpha7");
        // A typo stays a typo: it must miss the catalogue, not be repaired.
        assert_eq!(E::normalize("r1-alhpa8"), "r1-alhpa8");
    }
}
