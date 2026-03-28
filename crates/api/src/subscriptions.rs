use axum::{
    extract::{Extension, State},
    routing::{get, post, put},
    Json, Router,
};
use renzora_models::subscription::{AutoTopup, Subscription, SubscriptionPlan};
use renzora_models::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/subscribe", post(subscribe))
        .route("/cancel", post(cancel_subscription))
        .route("/current", get(current_subscription))
        .route("/usage", get(get_usage))
        .route("/extra-seats", put(update_extra_seats))
        .route("/extra-storage", put(update_extra_storage))
        .route("/auto-topup", get(get_auto_topup))
        .route("/auto-topup", put(update_auto_topup))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/plans", get(list_plans))
        .merge(protected)
}

/// List all available plans (public).
async fn list_plans(
    State(state): State<AppState>,
) -> Result<Json<Vec<SubscriptionPlan>>, ApiError> {
    let plans = SubscriptionPlan::list(&state.db).await?;
    Ok(Json(plans))
}

#[derive(Serialize)]
struct CurrentSubResponse {
    plan: SubscriptionPlan,
    subscription: Option<Subscription>,
    monthly_cost: i32,
    credit_balance: i64,
}

async fn current_subscription(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<CurrentSubResponse>, ApiError> {
    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?;
    let plan_id = sub.as_ref().filter(|s| s.is_active()).map(|s| s.plan_id.as_str()).unwrap_or("free");
    let plan = SubscriptionPlan::find(&state.db, plan_id).await?
        .ok_or(ApiError::Internal("Plan not found".into()))?;

    let monthly_cost = match &sub {
        Some(s) if s.is_active() => s.monthly_cost(&state.db).await.unwrap_or(0),
        _ => 0,
    };

    let user = User::find_by_id(&state.db, auth.user_id).await?
        .ok_or(ApiError::Internal("User not found".into()))?;

    Ok(Json(CurrentSubResponse {
        plan,
        subscription: sub,
        monthly_cost,
        credit_balance: user.credit_balance,
    }))
}

#[derive(Deserialize)]
struct SubscribeRequest {
    plan_id: String,
    #[serde(default)]
    extra_seats: i32,
    #[serde(default)]
    extra_storage_gb: i32,
}

/// Subscribe to a plan by paying credits.
async fn subscribe(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let plan = SubscriptionPlan::find(&state.db, &body.plan_id).await?
        .ok_or(ApiError::Validation(format!("Unknown plan: {}", body.plan_id)))?;

    if plan.price_credits == 0 {
        return Err(ApiError::Validation("Use /cancel to downgrade to Free".into()));
    }

    // Validate extra seats
    if body.extra_seats < 0 {
        return Err(ApiError::Validation("Extra seats cannot be negative".into()));
    }
    if body.extra_seats > 0 && plan.max_team_members == 0 {
        return Err(ApiError::Validation("This plan does not support teams".into()));
    }

    // Validate extra storage
    if body.extra_storage_gb < 0 {
        return Err(ApiError::Validation("Extra storage cannot be negative".into()));
    }

    // Calculate total cost
    let base = plan.price_credits;
    let seats_cost = body.extra_seats * plan.extra_seat_credits;
    let storage_cost = body.extra_storage_gb * plan.extra_storage_credits_per_gb;
    let total = base + seats_cost + storage_cost;

    // Check balance
    let user = User::find_by_id(&state.db, auth.user_id).await?
        .ok_or(ApiError::Internal("User not found".into()))?;

    if user.credit_balance < total as i64 {
        return Err(ApiError::Validation(format!(
            "Insufficient credits. Need {} credits, you have {}. Top up your wallet first.",
            total, user.credit_balance
        )));
    }

    // Deduct credits
    sqlx::query("UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2")
        .bind(total as i64)
        .bind(auth.user_id)
        .execute(&state.db)
        .await?;

    // Record transaction
    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, description) VALUES ($1, $2, 'subscription', $3, $4)"
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(-(total as i64))
    .bind(format!("{} plan subscription ({} credits/mo)", plan.name, total))
    .execute(&state.db)
    .await?;

    // Create/update subscription
    let sub = Subscription::subscribe(
        &state.db, auth.user_id, &body.plan_id,
        body.extra_seats, body.extra_storage_gb,
    ).await?;

    // Assign Discord role
    crate::discord::on_subscription_change(&state.db, auth.user_id, &body.plan_id).await;

    Ok(Json(serde_json::json!({
        "message": format!("Subscribed to {} plan", plan.name),
        "credits_charged": total,
        "subscription": sub,
    })))
}

async fn cancel_subscription(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?
        .ok_or(ApiError::Validation("No active subscription".into()))?;

    if sub.plan_id == "free" || !sub.is_active() {
        return Err(ApiError::Validation("No paid subscription to cancel".into()));
    }

    Subscription::cancel(&state.db, auth.user_id).await?;

    // Remove Discord role (will happen at period end, but remove now for immediate feedback)
    crate::discord::on_subscription_end(&state.db, auth.user_id).await;

    Ok(Json(serde_json::json!({
        "message": "Subscription will cancel at end of billing period",
        "period_end": sub.current_period_end.to_string(),
    })))
}

#[derive(Deserialize)]
struct ExtraSeatsRequest {
    extra_seats: i32,
}

async fn update_extra_seats(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<ExtraSeatsRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.extra_seats < 0 {
        return Err(ApiError::Validation("Cannot be negative".into()));
    }

    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?
        .ok_or(ApiError::Validation("No active subscription".into()))?;

    let plan = SubscriptionPlan::find(&state.db, &sub.plan_id).await?
        .ok_or(ApiError::Internal("Plan not found".into()))?;

    if plan.max_team_members == 0 {
        return Err(ApiError::Validation("Your plan does not support teams".into()));
    }

    sqlx::query("UPDATE subscriptions SET extra_seats = $1, updated_at = NOW() WHERE user_id = $2")
        .bind(body.extra_seats).bind(auth.user_id).execute(&state.db).await?;

    Ok(Json(serde_json::json!({
        "extra_seats": body.extra_seats,
        "extra_seat_cost": body.extra_seats * plan.extra_seat_credits,
    })))
}

#[derive(Deserialize)]
struct ExtraStorageRequest {
    extra_storage_gb: i32,
}

async fn update_extra_storage(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<ExtraStorageRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.extra_storage_gb < 0 {
        return Err(ApiError::Validation("Cannot be negative".into()));
    }

    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?
        .ok_or(ApiError::Validation("No active subscription".into()))?;

    let plan = SubscriptionPlan::find(&state.db, &sub.plan_id).await?
        .ok_or(ApiError::Internal("Plan not found".into()))?;

    sqlx::query("UPDATE subscriptions SET extra_storage_gb = $1, updated_at = NOW() WHERE user_id = $2")
        .bind(body.extra_storage_gb).bind(auth.user_id).execute(&state.db).await?;

    Ok(Json(serde_json::json!({
        "extra_storage_gb": body.extra_storage_gb,
        "extra_storage_cost": body.extra_storage_gb * plan.extra_storage_credits_per_gb,
    })))
}

#[derive(Serialize)]
struct UsageResponse {
    daily_requests: i32,
    daily_limit: i32,
    plan: String,
}

async fn get_usage(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UsageResponse>, ApiError> {
    let limit = renzora_models::subscription::daily_api_limit(&state.db, auth.user_id).await?;

    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT request_count FROM api_usage_daily WHERE user_id = $1 AND date = CURRENT_DATE"
    ).bind(auth.user_id).fetch_optional(&state.db).await?;

    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?;
    let plan_id = sub.as_ref().filter(|s| s.is_active()).map(|s| s.plan_id.clone()).unwrap_or("free".into());

    Ok(Json(UsageResponse {
        daily_requests: row.map(|r| r.0).unwrap_or(0),
        daily_limit: limit,
        plan: plan_id,
    }))
}

async fn get_auto_topup(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Option<AutoTopup>>, ApiError> {
    let topup = AutoTopup::find(&state.db, auth.user_id).await?;
    Ok(Json(topup))
}

#[derive(Deserialize)]
struct AutoTopupRequest {
    enabled: bool,
    threshold_credits: Option<i32>,
    topup_amount_credits: Option<i32>,
}

async fn update_auto_topup(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<AutoTopupRequest>,
) -> Result<Json<AutoTopup>, ApiError> {
    let threshold = body.threshold_credits.unwrap_or(100);
    let amount = body.topup_amount_credits.unwrap_or(500);

    if threshold < 0 || amount < 50 {
        return Err(ApiError::Validation("Threshold must be >= 0, amount must be >= 50 credits".into()));
    }

    let topup = AutoTopup::upsert(&state.db, auth.user_id, body.enabled, threshold, amount).await?;
    Ok(Json(topup))
}
