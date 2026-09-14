//! Daily request accounting for developer API tokens.

use sqlx::PgPool;
use uuid::Uuid;

/// Requests per day allowed on an API token. Previously read from the caller's
/// subscription plan; with subscription tiers gone every token gets the same
/// allowance.
pub const DAILY_API_LIMIT: i32 = 500;

/// Today's request count for a user, without touching it.
///
/// Separate from [`check_and_increment_usage`] because reading the number to
/// show somebody must not spend one of their requests: the obvious shortcut of
/// reusing the incrementing call would make opening the page the thing that
/// consumes the allowance it is reporting.
///
/// No row means no requests today. The row is only created by the first call.
pub async fn usage_today(db: &PgPool, user_id: Uuid) -> Result<(i32, i32), sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT request_count FROM api_usage_daily WHERE user_id = $1 AND date = CURRENT_DATE",
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    Ok((row.map(|r| r.0).unwrap_or(0), DAILY_API_LIMIT))
}

/// Increment and check daily API usage. Returns (current_count, limit).
pub async fn check_and_increment_usage(db: &PgPool, user_id: Uuid) -> Result<(i32, i32), sqlx::Error> {
    let limit = DAILY_API_LIMIT;

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO api_usage_daily (user_id, date, request_count)
         VALUES ($1, CURRENT_DATE, 1)
         ON CONFLICT (user_id, date) DO UPDATE SET request_count = api_usage_daily.request_count + 1
         RETURNING request_count"
    ).bind(user_id).fetch_one(db).await?;

    Ok((row.0, limit))
}
