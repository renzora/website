use axum::{
    extract::{Extension, Query, State},
    routing::{get, post},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::asset::Asset;
use renzora_models::user::User;
use sqlx::Row;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stats", get(stats))
        .route("/earnings", get(earnings))
        .route("/onboard-status", get(onboard_status))
        .route("/accept-policy", post(accept_policy))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// Get creator dashboard stats.
async fn stats(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<CreatorStatsResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let assets = Asset::list_by_creator(&state.db, auth.user_id).await?;

    let total_assets = assets.len() as i64;
    let total_downloads: i64 = assets.iter().map(|a| a.downloads).sum();

    // Sum earnings from transactions
    let earnings_row: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount), 0)::bigint FROM transactions WHERE user_id = $1 AND type = 'earning'",
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await?;

    // Top assets by downloads
    let mut sorted = assets;
    sorted.sort_by(|a, b| b.downloads.cmp(&a.downloads));
    let top_assets: Vec<AssetSummary> = sorted
        .into_iter()
        .take(5)
        .map(|a| {
            let rating_avg = if a.rating_count > 0 { a.rating_sum as f64 / a.rating_count as f64 } else { 0.0 };
            AssetSummary {
                id: a.id,
                name: a.name,
                slug: a.slug,
                description: a.description,
                category: a.category,
                price_credits: a.price_credits,
                thumbnail_url: a.thumbnail_url,
                version: a.version,
                downloads: a.downloads,
                views: a.views,
                creator_name: user.username.clone(),
                rating_avg,
                rating_count: a.rating_count,
            }
        })
        .collect();

    Ok(Json(CreatorStatsResponse {
        total_assets,
        total_downloads,
        total_earnings: earnings_row.0,
        credit_balance: user.credit_balance,
        top_assets,
    }))
}

/// Get paginated earnings history.
async fn earnings(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<CreatorEarningsResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page: i64 = 20;
    let offset = (page - 1) * per_page;

    let rows = sqlx::query(
        r#"
        SELECT t.id, t.amount, t.created_at, COALESCE(a.name, 'Unknown') AS asset_name
        FROM transactions t
        LEFT JOIN assets a ON a.id = t.asset_id
        WHERE t.user_id = $1 AND t.type = 'earning'
        ORDER BY t.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(auth.user_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    let entries: Vec<EarningEntry> = rows
        .iter()
        .map(|r| EarningEntry {
            id: r.get("id"),
            amount: r.get("amount"),
            asset_name: r.get("asset_name"),
            created_at: r.get::<time::OffsetDateTime, _>("created_at").to_string(),
        })
        .collect();

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM transactions WHERE user_id = $1 AND type = 'earning'",
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(CreatorEarningsResponse {
        earnings: entries,
        total: total.0,
        page,
    }))
}

/// Get creator onboarding status (policy accepted + Stripe connected).
async fn onboard_status(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(serde_json::json!({
        "policy_accepted": user.creator_policy_accepted_at.is_some(),
        "stripe_connected": user.stripe_connect_id.is_some(),
        "stripe_onboarded": user.stripe_connect_onboarded,
    })))
}

/// Accept the creator/seller policy.
async fn accept_policy(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query(
        "UPDATE users SET creator_policy_accepted_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(auth.user_id)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Policy accepted",
    })))
}
