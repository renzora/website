use crate::error::ApiError;
use crate::AppState;
use axum::{extract::State, routing::post, Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/join", post(join_waitlist))
}

#[derive(serde::Deserialize)]
struct JoinBody {
    email: String,
    #[serde(default)]
    source: Option<String>,
}

/// Public: add an email to the waiting list. Idempotent on the email.
async fn join_waitlist(
    State(state): State<AppState>,
    Json(body): Json<JoinBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let email = body.email.trim().to_lowercase();
    if !email.contains('@') || email.len() > 254 {
        return Err(ApiError::Validation("Enter a valid email".into()));
    }
    let source: String = body
        .source
        .as_deref()
        .unwrap_or("game")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .take(32)
        .collect();

    sqlx::query("INSERT INTO waitlist (id, email, source) VALUES ($1, $2, $3) ON CONFLICT (email) DO NOTHING")
        .bind(uuid::Uuid::new_v4())
        .bind(&email)
        .bind(&source)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "ok": true })))
}
