//! Daily view/download rollups per asset, and the series the marketplace graphs
//! read back out of them.
//!
//! The running totals on `assets` stay the source of truth for "how many ever";
//! this module answers "how many, when". Both are written in the same place so
//! they cannot drift: see [`bump_view`] and [`bump_download`], called from the
//! same handlers that move `assets.views` / `assets.downloads`.

use sqlx::PgPool;
use uuid::Uuid;

/// How a series is bucketed. The graph offers all three; the storage is daily
/// either way, and weeks and months are rolled up on read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bucket {
    Day,
    Week,
    Month,
}

impl Bucket {
    /// Parse the `range` query parameter. Anything unrecognised is `Day`, which
    /// is the view the graph opens on.
    pub fn from_param(s: &str) -> Self {
        match s {
            "weeks" | "week" => Self::Week,
            "months" | "month" => Self::Month,
            _ => Self::Day,
        }
    }

    /// The `date_trunc` unit. A fixed string per variant, never user input.
    fn unit(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
        }
    }

    /// One step of the series, as a Postgres interval.
    fn step(self) -> &'static str {
        match self {
            Self::Day => "1 day",
            Self::Week => "1 week",
            Self::Month => "1 month",
        }
    }

    /// How many buckets a request for this range returns. Thirty days reads as a
    /// month of detail; twelve weeks and twelve months each read as a season and
    /// a year without crowding the axis.
    pub fn count(self) -> i32 {
        match self {
            Self::Day => 30,
            Self::Week => 12,
            Self::Month => 12,
        }
    }
}

/// One point on the graph.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct StatsPoint {
    /// Start of the bucket, ISO `YYYY-MM-DD`. Serialized as a string rather than
    /// a date so the browser gets it back exactly as Postgres truncated it, with
    /// no timezone re-interpretation on the way through.
    pub bucket: String,
    pub views: i64,
    pub downloads: i64,
}

/// Record one view against today's row.
///
/// Call only when the view actually counted: `Asset::record_view` applies a
/// 24 hour per-IP cooldown, and a series that counted every refresh would
/// disagree with the total sitting next to it.
pub async fn bump_view(pool: &PgPool, asset_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO asset_stats_daily (asset_id, day, views, downloads)
        VALUES ($1, CURRENT_DATE, 1, 0)
        ON CONFLICT (asset_id, day) DO UPDATE SET views = asset_stats_daily.views + 1
        "#,
    )
    .bind(asset_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Record one download against today's row.
pub async fn bump_download(pool: &PgPool, asset_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO asset_stats_daily (asset_id, day, views, downloads)
        VALUES ($1, CURRENT_DATE, 0, 1)
        ON CONFLICT (asset_id, day) DO UPDATE SET downloads = asset_stats_daily.downloads + 1
        "#,
    )
    .bind(asset_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// The series for one asset, newest bucket last.
pub async fn series_for_asset(
    pool: &PgPool,
    asset_id: Uuid,
    bucket: Bucket,
) -> Result<Vec<StatsPoint>, sqlx::Error> {
    sqlx::query_as::<_, StatsPoint>(&series_sql("s.asset_id = $1"))
        .bind(asset_id)
        .bind(bucket.unit())
        .bind(bucket.step())
        .bind(bucket.count())
        .fetch_all(pool)
        .await
}

/// The series for every published or unpublished asset a creator owns, summed.
pub async fn series_for_creator(
    pool: &PgPool,
    creator_id: Uuid,
    bucket: Bucket,
) -> Result<Vec<StatsPoint>, sqlx::Error> {
    sqlx::query_as::<_, StatsPoint>(&series_sql(
        "s.asset_id IN (SELECT id FROM assets WHERE creator_id = $1)",
    ))
    .bind(creator_id)
    .bind(bucket.unit())
    .bind(bucket.step())
    .bind(bucket.count())
    .fetch_all(pool)
    .await
}

/// The shared query, parameterised only by which assets to sum over.
///
/// Built from `generate_series` with a LEFT JOIN rather than from the rollup
/// alone, so a bucket nothing happened in comes back as a zero instead of being
/// missing. A graph that silently drops quiet days draws a line between the two
/// days either side of them and shows a trend that never happened.
///
/// `scope` is one of two literals from this module, never user input; every
/// value in the query is bound.
fn series_sql(scope: &str) -> String {
    format!(
        r#"
        SELECT to_char(g.bucket, 'YYYY-MM-DD')       AS bucket,
               COALESCE(SUM(s.views), 0)::bigint     AS views,
               COALESCE(SUM(s.downloads), 0)::bigint AS downloads
        FROM generate_series(
                 date_trunc($2, NOW()) - (($4::int - 1) * $3::interval),
                 date_trunc($2, NOW()),
                 $3::interval
             ) AS g(bucket)
        LEFT JOIN asset_stats_daily s
               ON {scope}
              AND date_trunc($2, s.day::timestamptz) = g.bucket
        GROUP BY g.bucket
        ORDER BY g.bucket
        "#
    )
}
