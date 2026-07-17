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

/// Minimum monthly amount for the Supporter subscription.
pub const SUPPORTER_MIN_CREDITS: i64 = 10;

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/subscribe", post(subscribe))
        .route("/cancel", post(cancel_subscription))
        .route("/current", get(current_subscription))
        .route("/usage", get(get_usage))
        .route("/auto-renew", put(update_auto_renew))
        .route("/auto-topup", get(get_auto_topup))
        .route("/auto-topup", put(update_auto_topup))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/plans", get(list_plans))
        .merge(protected)
}

/// List available plans (public) — now just Free and Supporter.
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
    monthly_cost: i64,
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

    let monthly_cost = sub.as_ref().filter(|s| s.is_active()).map(|s| s.monthly_amount).unwrap_or(0);

    let user = User::find_by_id(&state.db, auth.user_id).await?
        .ok_or(ApiError::Internal("User not found".into()))?;

    Ok(Json(CurrentSubResponse {
        plan,
        subscription: sub,
        monthly_cost,
        credit_balance: user.credit_balance,
    }))
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct SubscribeRequest {
    /// Monthly amount in credits, chosen by the supporter (min 10).
    amount: i64,
    #[serde(default = "default_true")]
    auto_renew: bool,
}

/// Become a Supporter: pay-what-you-want, minimum 10 credits/month.
/// Charges the first month immediately and starts a 30-day period.
async fn subscribe(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.amount < SUPPORTER_MIN_CREDITS {
        return Err(ApiError::Validation(format!(
            "Minimum supporter amount is {SUPPORTER_MIN_CREDITS} credits/month"
        )));
    }

    // Deduct, record, and upsert the subscription in one DB transaction so a
    // failure can never charge credits without granting the subscription.
    let mut tx = state.db.begin().await?;

    let result = sqlx::query(
        "UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2 AND credit_balance >= $1",
    )
    .bind(body.amount)
    .bind(auth.user_id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::Validation(
            "Insufficient credits. Top up your wallet first.".into(),
        ));
    }

    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'subscription', $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(auth.user_id)
    .bind(-body.amount)
    .bind(format!("Supporter subscription ({} credits/mo)", body.amount))
    .execute(&mut *tx)
    .await?;

    let sub = Subscription::supporter_subscribe(&mut *tx, auth.user_id, body.amount, body.auto_renew).await?;

    tx.commit().await?;

    // Assign Discord supporter role (best-effort)
    crate::discord::on_subscription_change(&state.db, auth.user_id, "supporter").await;

    Ok(Json(serde_json::json!({
        "message": format!("Thank you for supporting Renzora with {} credits/month!", body.amount),
        "credits_charged": body.amount,
        "subscription": sub,
    })))
}

async fn cancel_subscription(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?
        .ok_or(ApiError::Validation("No active subscription".into()))?;

    if !sub.is_active() {
        return Err(ApiError::Validation("No active subscription to cancel".into()));
    }

    Subscription::cancel(&state.db, auth.user_id).await?;

    // Remove Discord role (will happen at period end, but remove now for immediate feedback)
    crate::discord::on_subscription_end(&state.db, auth.user_id).await;

    Ok(Json(serde_json::json!({
        "message": "Subscription will end at the close of the billing period",
        "period_end": sub.current_period_end.to_string(),
    })))
}

#[derive(Deserialize)]
struct AutoRenewRequest {
    enabled: bool,
}

/// Toggle auto-renewal (auto-deduct credits at the end of each term).
async fn update_auto_renew(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<AutoRenewRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let sub = Subscription::find_by_user(&state.db, auth.user_id).await?
        .ok_or(ApiError::Validation("No active subscription".into()))?;

    if !sub.is_active() {
        return Err(ApiError::Validation("No active subscription".into()));
    }

    // Re-enabling auto-renew also clears a pending cancellation.
    sqlx::query(
        "UPDATE subscriptions SET auto_renew = $1, cancel_at_period_end = (CASE WHEN $1 THEN false ELSE cancel_at_period_end END), updated_at = NOW() WHERE user_id = $2",
    )
    .bind(body.enabled)
    .bind(auth.user_id)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({ "auto_renew": body.enabled })))
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

// ── Renewal processing ──

/// Process all subscriptions whose period has ended. Called periodically from
/// a background task in the server.
///
/// - auto_renew on and enough credits → charge and extend 30 days
/// - auto_renew off, cancelled, or insufficient credits → expire
pub async fn process_due_renewals(state: &AppState) {
    let due = match Subscription::list_period_ended(&state.db).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Renewal sweep: failed to list due subscriptions: {e}");
            return;
        }
    };

    for sub in due {
        let renewable = sub.auto_renew
            && !sub.cancel_at_period_end
            && sub.monthly_amount >= SUPPORTER_MIN_CREDITS;

        if renewable {
            match try_renew(state, &sub).await {
                Ok(true) => {
                    tracing::info!(
                        "Renewed supporter subscription for user {} ({} credits)",
                        sub.user_id, sub.monthly_amount
                    );
                    continue;
                }
                Ok(false) => {
                    // Not enough credits — fall through to expiry
                    let _ = crate::notify::notify(
                        state,
                        sub.user_id,
                        "subscription",
                        "Supporter subscription ended",
                        &format!(
                            "Your balance was too low to renew ({} credits/month). Top up and re-subscribe any time.",
                            sub.monthly_amount
                        ),
                        Some("/subscription"),
                        None,
                    ).await;
                }
                Err(e) => {
                    tracing::error!("Renewal failed for user {}: {e}", sub.user_id);
                    continue; // transient error — retry next sweep
                }
            }
        }

        if let Err(e) = Subscription::expire(&state.db, sub.user_id).await {
            tracing::error!("Failed to expire subscription for user {}: {e}", sub.user_id);
            continue;
        }
        crate::discord::on_subscription_end(&state.db, sub.user_id).await;
    }
}

/// Attempt one renewal charge. Returns Ok(false) when the balance is too low.
async fn try_renew(state: &AppState, sub: &Subscription) -> Result<bool, sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result = sqlx::query(
        "UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2 AND credit_balance >= $1",
    )
    .bind(sub.monthly_amount)
    .bind(sub.user_id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(false);
    }

    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'subscription', $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(sub.user_id)
    .bind(-sub.monthly_amount)
    .bind(format!("Supporter renewal ({} credits/mo)", sub.monthly_amount))
    .execute(&mut *tx)
    .await?;

    Subscription::extend_period(&mut *tx, sub.user_id).await?;

    tx.commit().await?;

    state.ws_broadcast.send_to_user(sub.user_id, "credit_update", serde_json::json!({
        "amount": -sub.monthly_amount,
        "type": "subscription_renewal",
    }));

    Ok(true)
}
