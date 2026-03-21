use axum::{
    body::Bytes,
    extract::{Extension, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use hmac::{Hmac, Mac};
use renzora_common::types::*;
use renzora_models::asset::{self, Asset};
use renzora_models::transaction;
use renzora_models::user::User;
use sha2::Sha256;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/balance", get(get_balance))
        .route("/topup", post(create_topup))
        .route("/history", get(get_history))
        .route("/purchase", post(purchase_asset))
        .route("/referral-stats", get(referral_stats))
        .route("/connect/onboard", post(start_connect_onboarding))
        .route("/connect/status", get(connect_status))
        .route("/withdraw", post(request_withdrawal))
        .route("/withdrawals", get(list_withdrawals))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/webhook", post(stripe_webhook))
        .merge(protected)
}

/// Get the current user's credit balance.
async fn get_balance(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<BalanceResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(BalanceResponse {
        credit_balance: user.credit_balance,
    }))
}

/// Create a Stripe Checkout session for topping up credits.
async fn create_topup(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<TopUpRequest>,
) -> Result<Json<TopUpResponse>, ApiError> {
    let stripe_secret = state.stripe_secret_key.as_ref().ok_or_else(|| {
        ApiError::Internal("Stripe not configured".into())
    })?;

    if body.amount < 50 {
        return Err(ApiError::Validation("Minimum top-up is 50 credits".into()));
    }

    // 1 credit = $0.10 USD (10 credits = $1.00)
    let price_cents = body.amount * 10;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .header("Authorization", format!("Bearer {stripe_secret}"))
        .form(&[
            ("mode", "payment"),
            ("success_url", &format!("{}/wallet?success=true", state.site_url)),
            ("cancel_url", &format!("{}/wallet?cancelled=true", state.site_url)),
            ("line_items[0][price_data][currency]", "usd"),
            (
                "line_items[0][price_data][product_data][name]",
                &format!("{} Renzora Credits", body.amount),
            ),
            (
                "line_items[0][price_data][unit_amount]",
                &price_cents.to_string(),
            ),
            ("line_items[0][quantity]", "1"),
            ("metadata[user_id]", &auth.user_id.to_string()),
            ("metadata[credits]", &body.amount.to_string()),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Stripe request failed: {e}")))?;

    if !response.status().is_success() {
        let err_text = response.text().await.unwrap_or_default();
        return Err(ApiError::Internal(format!("Stripe error: {err_text}")));
    }

    let session: serde_json::Value = response
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Stripe response: {e}")))?;

    let checkout_url = session["url"]
        .as_str()
        .ok_or_else(|| ApiError::Internal("Missing checkout URL in Stripe response".into()))?
        .to_string();

    Ok(Json(TopUpResponse { checkout_url }))
}

/// Stripe webhook handler — processes completed checkout sessions.
async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, ApiError> {
    let webhook_secret = state.stripe_webhook_secret.as_ref().ok_or_else(|| {
        ApiError::Internal("Stripe webhook secret not configured".into())
    })?;

    // Verify webhook signature
    let sig_header = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Validation("Missing Stripe signature".into()))?;

    verify_stripe_signature(&body, sig_header, webhook_secret)
        .map_err(|e| ApiError::Validation(format!("Invalid webhook signature: {e}")))?;

    let event: serde_json::Value =
        serde_json::from_slice(&body).map_err(|e| ApiError::Validation(e.to_string()))?;

    let event_type = event["type"].as_str().unwrap_or("");

    if event_type == "checkout.session.completed" {
        let session = &event["data"]["object"];
        let payment_status = session["payment_status"].as_str().unwrap_or("");

        if payment_status == "paid" {
            let user_id_str = session["metadata"]["user_id"]
                .as_str()
                .ok_or_else(|| ApiError::Internal("Missing user_id in metadata".into()))?;
            let credits_str = session["metadata"]["credits"]
                .as_str()
                .ok_or_else(|| ApiError::Internal("Missing credits in metadata".into()))?;
            let stripe_session_id = session["id"]
                .as_str()
                .ok_or_else(|| ApiError::Internal("Missing session id".into()))?;

            let user_id: Uuid = user_id_str
                .parse()
                .map_err(|_| ApiError::Internal("Invalid user_id".into()))?;
            let credits: i64 = credits_str
                .parse()
                .map_err(|_| ApiError::Internal("Invalid credits amount".into()))?;

            transaction::add_credits(&state.db, user_id, credits, stripe_session_id)
                .await
                .map_err(|e| ApiError::Internal(format!("Failed to add credits: {e}")))?;

            tracing::info!("Added {credits} credits to user {user_id}");

            // Live credit update
            state.ws_broadcast.send_to_user(user_id, "credit_update", serde_json::json!({
                "amount": credits,
                "type": "topup",
            }));
        }
    }

    Ok(StatusCode::OK)
}

/// Get transaction history for the current user.
async fn get_history(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<TransactionHistoryResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page: i64 = 20;

    let (transactions, total) =
        renzora_models::transaction::Transaction::list_for_user(&state.db, auth.user_id, page, per_page)
            .await?;

    let entries: Vec<TransactionEntry> = transactions
        .into_iter()
        .map(|t| TransactionEntry {
            id: t.id,
            r#type: t.r#type,
            amount: t.amount,
            asset_id: t.asset_id,
            created_at: t.created_at.to_string(),
        })
        .collect();

    Ok(Json(TransactionHistoryResponse {
        transactions: entries,
        total,
        page,
    }))
}

/// Purchase an asset with credits, optionally applying a promo code.
async fn purchase_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<PurchaseRequest>,
) -> Result<Json<PurchaseResponse>, ApiError> {
    use renzora_models::promo_code::PromoCode;

    let asset = Asset::find_by_id(&state.db, body.asset_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !asset.published {
        return Err(ApiError::NotFound);
    }

    // Creator can always access their own asset
    if asset.creator_id == auth.user_id {
        asset::grant_asset_ownership(&state.db, auth.user_id, body.asset_id).await?;
        let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
        return Ok(Json(PurchaseResponse {
            message: "This is your asset".into(),
            new_balance: user.credit_balance,
        }));
    }

    // Check if already owned
    if asset::user_owns_asset(&state.db, auth.user_id, body.asset_id).await? {
        return Err(ApiError::Validation("You already own this asset".into()));
    }

    // Free assets — just grant ownership
    if asset.price_credits == 0 {
        asset::grant_asset_ownership(&state.db, auth.user_id, body.asset_id).await?;
        let user = User::find_by_id(&state.db, auth.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;
        return Ok(Json(PurchaseResponse {
            message: "Asset added to your library".into(),
            new_balance: user.credit_balance,
        }));
    }

    // Validate promo code if provided
    let mut promo_discount: i32 = 0;
    let mut promo_id: Option<Uuid> = None;
    if let Some(ref code) = body.promo_code {
        let code = code.trim();
        if !code.is_empty() {
            let promo = PromoCode::find_valid(&state.db, code)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?
                .ok_or_else(|| ApiError::Validation("Invalid or expired promo code".into()))?;
            promo_discount = promo.discount_percent;
            promo_id = Some(promo.id);
        }
    }

    // Process paid purchase with promo discount
    transaction::process_purchase_with_promo(
        &state.db,
        auth.user_id,
        body.asset_id,
        asset.price_credits,
        asset.creator_id,
        promo_discount,
    )
    .await
    .map_err(|e| {
        if e.contains("Insufficient") {
            ApiError::Validation(e)
        } else {
            ApiError::Internal(e)
        }
    })?;

    // Record promo code use
    if let Some(pid) = promo_id {
        let _ = PromoCode::record_use(
            &state.db,
            pid,
            auth.user_id,
            body.asset_id,
            promo_discount,
        )
        .await;
    }

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let msg = if promo_discount > 0 {
        format!(
            "Purchased {} for {} credits (promo: {}% off platform fee)",
            asset.name, asset.price_credits, promo_discount
        )
    } else {
        format!("Purchased {} for {} credits", asset.name, asset.price_credits)
    };

    Ok(Json(PurchaseResponse {
        message: msg,
        new_balance: user.credit_balance,
    }))
}

// ── Stripe Connect & Withdrawals ──

/// Start Stripe Connect onboarding — returns a URL to redirect the user to.
async fn start_connect_onboarding(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let stripe_secret = state.stripe_secret_key.as_ref().ok_or_else(|| {
        ApiError::Internal("Stripe not configured".into())
    })?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Create or reuse Connect account
    let account_id = if let Some(ref id) = user.stripe_connect_id {
        id.clone()
    } else {
        // Create Express account
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.stripe.com/v1/accounts")
            .header("Authorization", format!("Bearer {stripe_secret}"))
            .form(&[
                ("type", "express"),
                ("email", &user.email),
                ("metadata[user_id]", &auth.user_id.to_string()),
            ])
            .send()
            .await
            .map_err(|e| ApiError::Internal(format!("Stripe error: {e}")))?;

        let account: serde_json::Value = res
            .json()
            .await
            .map_err(|e| ApiError::Internal(format!("Stripe parse error: {e}")))?;

        let account_id = account["id"]
            .as_str()
            .ok_or_else(|| ApiError::Internal("Missing account id".into()))?
            .to_string();

        sqlx::query("UPDATE users SET stripe_connect_id = $1, updated_at = NOW() WHERE id = $2")
            .bind(&account_id)
            .bind(auth.user_id)
            .execute(&state.db)
            .await?;

        account_id
    };

    // Create account link for onboarding
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.stripe.com/v1/account_links")
        .header("Authorization", format!("Bearer {stripe_secret}"))
        .form(&[
            ("account", account_id.as_str()),
            ("refresh_url", &format!("{}/settings", state.site_url)),
            ("return_url", &format!("{}/settings?connect=success", state.site_url)),
            ("type", "account_onboarding"),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Stripe error: {e}")))?;

    let link: serde_json::Value = res
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Stripe parse error: {e}")))?;

    let url = link["url"]
        .as_str()
        .ok_or_else(|| ApiError::Internal("Missing onboarding URL".into()))?;

    Ok(Json(serde_json::json!({ "url": url })))
}

/// Check Stripe Connect onboarding status.
async fn connect_status(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let stripe_secret = state.stripe_secret_key.as_ref().ok_or_else(|| {
        ApiError::Internal("Stripe not configured".into())
    })?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let Some(ref account_id) = user.stripe_connect_id else {
        return Ok(Json(serde_json::json!({
            "connected": false,
            "onboarded": false,
        })));
    };

    // Check account status with Stripe
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("https://api.stripe.com/v1/accounts/{account_id}"))
        .header("Authorization", format!("Bearer {stripe_secret}"))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Stripe error: {e}")))?;

    let account: serde_json::Value = res
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Stripe parse error: {e}")))?;

    let charges_enabled = account["charges_enabled"].as_bool().unwrap_or(false);
    let payouts_enabled = account["payouts_enabled"].as_bool().unwrap_or(false);
    let onboarded = charges_enabled && payouts_enabled;

    // Update DB if status changed
    if onboarded != user.stripe_connect_onboarded {
        sqlx::query("UPDATE users SET stripe_connect_onboarded = $1, updated_at = NOW() WHERE id = $2")
            .bind(onboarded)
            .bind(auth.user_id)
            .execute(&state.db)
            .await?;
    }

    Ok(Json(serde_json::json!({
        "connected": true,
        "onboarded": onboarded,
        "charges_enabled": charges_enabled,
        "payouts_enabled": payouts_enabled,
    })))
}

/// Request a credit withdrawal.
async fn request_withdrawal(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    use renzora_models::withdrawal::Withdrawal;

    let stripe_secret = state.stripe_secret_key.as_ref().ok_or_else(|| {
        ApiError::Internal("Stripe not configured".into())
    })?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Verify Connect is set up
    let account_id = user
        .stripe_connect_id
        .as_deref()
        .ok_or_else(|| ApiError::Validation("Please connect your bank account first".into()))?;

    if !user.stripe_connect_onboarded {
        return Err(ApiError::Validation("Please complete Stripe onboarding first".into()));
    }

    // Check no pending withdrawal exists
    let pending: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM withdrawals WHERE user_id = $1 AND status IN ('pending', 'processing')",
    )
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?;

    if pending.is_some() {
        return Err(ApiError::Validation("You already have a pending withdrawal".into()));
    }

    // Create withdrawal (deducts credits atomically)
    let withdrawal = Withdrawal::create(&state.db, auth.user_id, body.amount)
        .await
        .map_err(|e| {
            if e.contains("Insufficient") || e.contains("Minimum") {
                ApiError::Validation(e)
            } else {
                ApiError::Internal(e)
            }
        })?;

    // Create Stripe Transfer
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.stripe.com/v1/transfers")
        .header("Authorization", format!("Bearer {stripe_secret}"))
        .form(&[
            ("amount", &withdrawal.amount_usd_cents.to_string()),
            ("currency", &"usd".to_string()),
            ("destination", &account_id.to_string()),
            ("metadata[withdrawal_id]", &withdrawal.id.to_string()),
            ("metadata[user_id]", &auth.user_id.to_string()),
        ])
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            let transfer: serde_json::Value = response
                .json()
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            let transfer_id = transfer["id"].as_str().unwrap_or("");
            Withdrawal::mark_completed(&state.db, withdrawal.id, transfer_id).await?;

            let new_balance = User::find_by_id(&state.db, auth.user_id)
                .await?
                .map(|u| u.credit_balance)
                .unwrap_or(0);

            Ok(Json(serde_json::json!({
                "message": format!("Withdrawal of {} credits (${:.2}) initiated", withdrawal.amount_credits, withdrawal.amount_usd_cents as f64 / 100.0),
                "new_balance": new_balance,
                "withdrawal_id": withdrawal.id,
            })))
        }
        Ok(response) => {
            let err_text = response.text().await.unwrap_or_default();
            // Refund credits on failure
            let _ = Withdrawal::mark_failed(&state.db, withdrawal.id, &err_text).await;
            Err(ApiError::Internal(format!("Transfer failed: {err_text}")))
        }
        Err(e) => {
            let _ = Withdrawal::mark_failed(&state.db, withdrawal.id, &e.to_string()).await;
            Err(ApiError::Internal(format!("Transfer request failed: {e}")))
        }
    }
}

#[derive(serde::Deserialize)]
struct WithdrawRequest {
    amount: i64,
}

/// List the user's withdrawal history.
async fn list_withdrawals(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<renzora_models::withdrawal::Withdrawal>>, ApiError> {
    let withdrawals =
        renzora_models::withdrawal::Withdrawal::list_for_user(&state.db, auth.user_id).await?;
    Ok(Json(withdrawals))
}

/// Get referral stats for the current user.
async fn referral_stats(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let referral_code = user.referral_code.unwrap_or_default();

    // Count referred users
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM users WHERE referred_by = $1",
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await?;

    // Total referral earnings
    let earned: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(referral_amount), 0)::bigint FROM referral_earnings WHERE referrer_id = $1",
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "referral_code": referral_code,
        "referral_count": count.0,
        "total_earned": earned.0,
    })))
}

/// Verify Stripe webhook signature (v1 scheme).
fn verify_stripe_signature(payload: &[u8], sig_header: &str, secret: &str) -> Result<(), String> {
    let mut timestamp = None;
    let mut signatures = Vec::new();

    for part in sig_header.split(',') {
        let mut kv = part.splitn(2, '=');
        match (kv.next(), kv.next()) {
            (Some("t"), Some(t)) => timestamp = Some(t.to_string()),
            (Some("v1"), Some(v)) => signatures.push(v.to_string()),
            _ => {}
        }
    }

    let ts = timestamp.ok_or("Missing timestamp")?;
    if signatures.is_empty() {
        return Err("Missing v1 signature".into());
    }

    let signed_payload = format!("{ts}.{}", std::str::from_utf8(payload).map_err(|e| e.to_string())?);

    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|e| e.to_string())?;
    mac.update(signed_payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    if signatures.iter().any(|s| s == &expected) {
        Ok(())
    } else {
        Err("Signature mismatch".into())
    }
}
