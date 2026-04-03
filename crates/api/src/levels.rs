use axum::{
    extract::{Extension, State},
    routing::get,
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::user::User;
use renzora_models::xp::{self, SellerLevel, SellerTask, XpEvent};

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(my_level))
        .route("/history", get(xp_history))
        .route("/seller-progress", get(seller_progress))
        .route("/seller-levels", get(list_seller_levels))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// Get current user's XP/level info.
async fn my_level(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserLevelResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;

    let current_level_xp = xp::xp_for_level(user.level);
    let next_level_xp = xp::xp_for_level(user.level + 1);
    let range = next_level_xp - current_level_xp;
    let progress = if range > 0 {
        ((user.total_xp - current_level_xp) as f64 / range as f64 * 100.0).min(100.0).max(0.0)
    } else { 100.0 };

    // Seller level info
    let seller_level_info = SellerLevel::find_by_level(&state.db, user.seller_level).await?;
    let next_seller = SellerLevel::find_by_level(&state.db, user.seller_level + 1).await?;
    let seller_name = seller_level_info.as_ref().map(|l| l.name.clone()).unwrap_or_else(|| "New Seller".into());
    let seller_color = seller_level_info.as_ref().map(|l| l.badge_color.clone()).unwrap_or_else(|| "#71717a".into());
    let next_seller_xp = next_seller.as_ref().map(|l| l.min_seller_xp).unwrap_or(0);
    let current_seller_xp = seller_level_info.as_ref().map(|l| l.min_seller_xp).unwrap_or(0);
    let seller_range = next_seller_xp - current_seller_xp;
    let seller_progress = if seller_range > 0 {
        ((user.seller_xp - current_seller_xp) as f64 / seller_range as f64 * 100.0).min(100.0).max(0.0)
    } else { 100.0 };

    Ok(Json(UserLevelResponse {
        total_xp: user.total_xp,
        level: user.level,
        xp_for_current_level: current_level_xp,
        xp_for_next_level: next_level_xp,
        progress_percent: progress,
        seller_level: user.seller_level,
        seller_xp: user.seller_xp,
        seller_level_name: seller_name,
        seller_level_color: seller_color,
        next_seller_level_xp: next_seller_xp,
        seller_progress_percent: seller_progress,
    }))
}

/// XP event history.
async fn xp_history(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<XpEventResponse>>, ApiError> {
    let events = XpEvent::list_for_user(&state.db, auth.user_id, 50).await?;
    Ok(Json(events.iter().map(|e| XpEventResponse {
        amount: e.amount,
        reason: e.reason.clone(),
        created_at: e.created_at.to_string(),
    }).collect()))
}

/// Seller progress with task completion status.
async fn seller_progress(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<SellerProgressResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;

    let current = SellerLevel::find_by_level(&state.db, user.seller_level).await?
        .ok_or(ApiError::Internal("Seller level not found".into()))?;
    let next = SellerLevel::find_by_level(&state.db, user.seller_level + 1).await?;
    let tasks = SellerTask::list_for_level(&state.db, user.seller_level).await?;

    // Compute current values for each task
    let mut task_progress = Vec::new();
    for task in &tasks {
        let current_value = match task.task_type.as_str() {
            "assets_uploaded" => {
                let r: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets WHERE creator_id = $1")
                    .bind(auth.user_id).fetch_one(&state.db).await?;
                r.0
            }
            "total_sales" => {
                let r: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE asset_id IN (SELECT id FROM assets WHERE creator_id = $1) AND type = 'purchase'")
                    .bind(auth.user_id).fetch_one(&state.db).await?;
                r.0
            }
            "total_revenue" => {
                let r: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE user_id = $1 AND type = 'earning'")
                    .bind(auth.user_id).fetch_one(&state.db).await?;
                r.0
            }
            "positive_reviews" => {
                let r: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM reviews WHERE asset_id IN (SELECT id FROM assets WHERE creator_id = $1) AND rating >= 4")
                    .bind(auth.user_id).fetch_one(&state.db).await?;
                r.0
            }
            "top_asset_downloads" => {
                let r: (i64,) = sqlx::query_as("SELECT COALESCE(MAX(downloads),0)::bigint FROM assets WHERE creator_id = $1")
                    .bind(auth.user_id).fetch_one(&state.db).await?;
                r.0
            }
            _ => 0,
        };

        task_progress.push(SellerTaskProgress {
            description: task.description.clone(),
            task_type: task.task_type.clone(),
            target_value: task.target_value,
            current_value,
            completed: current_value >= task.target_value,
            xp_reward: task.xp_reward,
        });
    }

    let next_xp = next.as_ref().map(|l| l.min_seller_xp).unwrap_or(current.min_seller_xp);
    let range = next_xp - current.min_seller_xp;
    let progress = if range > 0 {
        ((user.seller_xp - current.min_seller_xp) as f64 / range as f64 * 100.0).min(100.0).max(0.0)
    } else { 100.0 };

    Ok(Json(SellerProgressResponse {
        current_level: serde_json::json!({ "level": current.level, "name": current.name, "color": current.badge_color, "perks": current.perks }),
        next_level: next.map(|l| serde_json::json!({ "level": l.level, "name": l.name, "color": l.badge_color, "min_xp": l.min_seller_xp })),
        tasks: task_progress,
        seller_xp: user.seller_xp,
        xp_to_next: (next_xp - user.seller_xp).max(0),
        progress_percent: progress,
    }))
}

/// List all seller levels.
async fn list_seller_levels(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let levels = SellerLevel::list_all(&state.db).await?;
    Ok(Json(levels.iter().map(|l| serde_json::json!({
        "level": l.level, "name": l.name, "min_xp": l.min_seller_xp,
        "boost": l.search_boost, "color": l.badge_color, "perks": l.perks
    })).collect()))
}
