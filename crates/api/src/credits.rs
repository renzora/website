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
        .route("/redeem-voucher", post(redeem_voucher))
        .route("/gift-cards/send", post(send_gift_card))
        .route("/gift-cards/redeem", post(redeem_gift_card))
        .route("/gift-cards/sent", get(list_sent_gifts))
        .route("/gift-cards/received", get(list_received_gifts))
        .route("/donate", post(make_donation))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/webhook", post(stripe_webhook))
        .route("/donate/leaderboard", get(donation_leaderboard))
        .route("/donate/total", get(donation_total))
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

    // Notify the seller about the sale
    let _ = renzora_models::notification::Notification::create(
        &state.db,
        asset.creator_id,
        "sale",
        &format!("{} purchased your asset", user.username),
        &format!("{} was sold for {} credits", asset.name, asset.price_credits),
        Some(&format!("/marketplace/asset/{}", asset.slug)),
    ).await;

    // Send real-time notification to seller
    state.ws_broadcast.send_to_user(asset.creator_id, "notification", serde_json::json!({
        "title": format!("{} purchased your asset", user.username),
        "body": format!("{} was sold for {} credits", asset.name, asset.price_credits),
    }));

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

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| ApiError::Internal(format!("Stripe read error: {e}")))?;

        if !status.is_success() {
            return Err(ApiError::Internal(format!("Stripe Connect error ({}): {}", status, body)));
        }

        let account: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| ApiError::Internal(format!("Stripe parse error: {e}")))?;

        let account_id = account["id"]
            .as_str()
            .ok_or_else(|| ApiError::Internal(format!("Missing account id. Stripe response: {}", body)))?
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

// ── Voucher Redemption ──

#[derive(serde::Deserialize)]
struct RedeemVoucherBody {
    code: String,
}

async fn redeem_voucher(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<RedeemVoucherBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let voucher = renzora_models::voucher::Voucher::find_valid(&state.db, &body.code)
        .await?
        .ok_or(ApiError::Validation("Invalid or expired voucher code".into()))?;

    // Check per-user usage
    let usage = renzora_models::voucher::Voucher::check_user_usage(&state.db, voucher.id, auth.user_id).await?;
    if usage >= voucher.max_uses_per_user as i64 {
        return Err(ApiError::Validation("You have already used this voucher".into()));
    }

    match voucher.voucher_type.as_str() {
        "credit" => {
            let amount = voucher.credit_amount.unwrap_or(0);
            if amount <= 0 {
                return Err(ApiError::Internal("Invalid voucher configuration".into()));
            }

            // Increment usage
            sqlx::query("UPDATE vouchers SET times_used = times_used + 1 WHERE id = $1")
                .bind(voucher.id).execute(&state.db).await?;

            // Record usage
            sqlx::query("INSERT INTO voucher_uses (voucher_id, user_id, credit_amount) VALUES ($1, $2, $3)")
                .bind(voucher.id).bind(auth.user_id).bind(amount).execute(&state.db).await?;

            // Create transaction
            let tx_id = uuid::Uuid::new_v4();
            sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'voucher_credit', $3, $4)")
                .bind(tx_id).bind(auth.user_id).bind(amount).bind(format!("Voucher: {}", voucher.code))
                .execute(&state.db).await?;

            // Add credits
            sqlx::query("UPDATE users SET credit_balance = credit_balance + $1 WHERE id = $2")
                .bind(amount).bind(auth.user_id).execute(&state.db).await?;

            Ok(Json(serde_json::json!({
                "ok": true,
                "type": "credit",
                "amount": amount,
                "message": format!("Credited {} credits to your account", amount),
            })))
        }
        "asset_discount" => {
            // For asset discounts, we store the voucher info but don't apply it here.
            // The actual discount happens during purchase. Just validate and confirm.
            Ok(Json(serde_json::json!({
                "ok": true,
                "type": "asset_discount",
                "discount_percent": voucher.discount_percent,
                "message": format!("Voucher applied! You'll get {}% off your next eligible purchase.", voucher.discount_percent.unwrap_or(0)),
            })))
        }
        _ => Err(ApiError::Validation("Unknown voucher type".into())),
    }
}

// ── Gift Cards ──

#[derive(serde::Deserialize)]
struct SendGiftBody {
    recipient_username: Option<String>,
    amount: i64,
    message: Option<String>,
}

async fn send_gift_card(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SendGiftBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.amount < 10 {
        return Err(ApiError::Validation("Minimum gift is 10 credits".into()));
    }

    // Check balance
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    if user.credit_balance < body.amount {
        return Err(ApiError::Validation("Insufficient credits".into()));
    }

    // Find recipient if specified
    let recipient_id = if let Some(ref username) = body.recipient_username {
        let r = User::find_by_username(&state.db, username).await?.ok_or(ApiError::Validation("User not found".into()))?;
        Some(r.id)
    } else {
        None
    };

    // Generate code
    let code = format!("GIFT-{}", &uuid::Uuid::new_v4().to_string()[..8].to_uppercase());

    // Deduct credits from sender
    sqlx::query("UPDATE users SET credit_balance = credit_balance - $1 WHERE id = $2")
        .bind(body.amount).bind(auth.user_id).execute(&state.db).await?;

    // Record transaction
    sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'gift_sent', $3, $4)")
        .bind(uuid::Uuid::new_v4()).bind(auth.user_id).bind(-body.amount).bind(format!("Gift card: {}", code))
        .execute(&state.db).await?;

    let gift = renzora_models::gift_card::GiftCard::create(&state.db, auth.user_id, recipient_id, &code, body.amount, body.message.as_deref().unwrap_or("")).await?;

    // If direct recipient, auto-redeem and notify
    if let Some(rid) = recipient_id {
        renzora_models::gift_card::GiftCard::redeem(&state.db, gift.id, rid).await?;
        sqlx::query("UPDATE users SET credit_balance = credit_balance + $1 WHERE id = $2")
            .bind(body.amount).bind(rid).execute(&state.db).await?;
        sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'gift_received', $3, $4)")
            .bind(uuid::Uuid::new_v4()).bind(rid).bind(body.amount).bind(format!("Gift from {}", user.username))
            .execute(&state.db).await?;

        // Notify recipient
        renzora_models::notification::Notification::create(&state.db, rid, "gift",
            &format!("{} sent you a gift!", user.username),
            &format!("{} credits", body.amount),
            Some("/wallet"),
        ).await?;
        state.ws_broadcast.send_to_user(rid, "credit_update", serde_json::json!({"amount": body.amount}));
    }

    Ok(Json(serde_json::json!({"code": gift.code, "amount": gift.amount, "auto_redeemed": recipient_id.is_some()})))
}

async fn redeem_gift_card(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<RedeemVoucherBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let gift = renzora_models::gift_card::GiftCard::find_by_code(&state.db, &body.code).await?.ok_or(ApiError::Validation("Invalid or expired gift card".into()))?;

    renzora_models::gift_card::GiftCard::redeem(&state.db, gift.id, auth.user_id).await?;
    sqlx::query("UPDATE users SET credit_balance = credit_balance + $1 WHERE id = $2")
        .bind(gift.amount).bind(auth.user_id).execute(&state.db).await?;
    sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'gift_received', $3, $4)")
        .bind(uuid::Uuid::new_v4()).bind(auth.user_id).bind(gift.amount).bind(format!("Gift card: {}", gift.code))
        .execute(&state.db).await?;

    Ok(Json(serde_json::json!({"ok": true, "amount": gift.amount})))
}

async fn list_sent_gifts(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let gifts = renzora_models::gift_card::GiftCard::list_sent(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!(gifts)))
}

async fn list_received_gifts(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let gifts = renzora_models::gift_card::GiftCard::list_received(&state.db, auth.user_id).await?;
    Ok(Json(serde_json::json!(gifts)))
}

// ── Donations ──

#[derive(serde::Deserialize)]
struct DonateBody {
    amount: i64,
    message: Option<String>,
    anonymous: Option<bool>,
}

async fn make_donation(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<DonateBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.amount < 1 {
        return Err(ApiError::Validation("Minimum donation is 1 credit".into()));
    }

    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    if user.credit_balance < body.amount {
        return Err(ApiError::Validation("Insufficient credits".into()));
    }

    // Deduct credits
    sqlx::query("UPDATE users SET credit_balance = credit_balance - $1 WHERE id = $2")
        .bind(body.amount).bind(auth.user_id).execute(&state.db).await?;
    sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason) VALUES ($1, $2, 'donation', $3, $4)")
        .bind(uuid::Uuid::new_v4()).bind(auth.user_id).bind(-body.amount).bind("Donation to Renzora")
        .execute(&state.db).await?;

    let donation = renzora_models::donation::Donation::create(
        &state.db, auth.user_id, body.amount, body.message.as_deref().unwrap_or(""), body.anonymous.unwrap_or(false)
    ).await?;

    // Check donation badge thresholds
    let total: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount), 0)::bigint FROM donations WHERE user_id = $1")
        .bind(auth.user_id).fetch_one(&state.db).await?;
    // Award badges at thresholds (100, 500, 1000, 5000)
    let badges = [(100, "donor_bronze"), (500, "donor_silver"), (1000, "donor_gold"), (5000, "donor_platinum")];
    for (threshold, badge_slug) in &badges {
        if total.0 >= *threshold {
            let _ = sqlx::query("INSERT INTO user_badges (user_id, badge_id) SELECT $1, id FROM badges WHERE slug = $2 ON CONFLICT DO NOTHING")
                .bind(auth.user_id).bind(badge_slug).execute(&state.db).await;
        }
    }

    Ok(Json(serde_json::json!({"ok": true, "amount": donation.amount, "total_donated": total.0})))
}

async fn donation_leaderboard(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let leaders = renzora_models::donation::Donation::leaderboard(&state.db, 20).await?;
    let items: Vec<serde_json::Value> = leaders.iter().map(|(uid, username, avatar, total, anon)| {
        if *anon {
            serde_json::json!({"username": "Anonymous", "total": total})
        } else {
            serde_json::json!({"user_id": uid, "username": username, "avatar_url": avatar, "total": total})
        }
    }).collect();
    Ok(Json(serde_json::json!(items)))
}

async fn donation_total(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let total = renzora_models::donation::Donation::total(&state.db).await?;
    Ok(Json(serde_json::json!({"total": total})))
}
