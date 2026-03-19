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

    if body.amount < 100 {
        return Err(ApiError::Validation("Minimum top-up is 100 credits".into()));
    }

    // 1 credit = $0.01 USD (100 credits = $1.00)
    let price_cents = body.amount;

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

/// Purchase an asset with credits.
async fn purchase_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<PurchaseRequest>,
) -> Result<Json<PurchaseResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, body.asset_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !asset.published {
        return Err(ApiError::NotFound);
    }

    // Can't buy your own asset
    if asset.creator_id == auth.user_id {
        return Err(ApiError::Validation("Cannot purchase your own asset".into()));
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

    // Process paid purchase
    transaction::process_purchase(
        &state.db,
        auth.user_id,
        body.asset_id,
        asset.price_credits,
        asset.creator_id,
    )
    .await
    .map_err(|e| {
        if e.contains("Insufficient") {
            ApiError::Validation(e)
        } else {
            ApiError::Internal(e)
        }
    })?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(PurchaseResponse {
        message: format!("Purchased {} for {} credits", asset.name, asset.price_credits),
        new_balance: user.credit_balance,
    }))
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
