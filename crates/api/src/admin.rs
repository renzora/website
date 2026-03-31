use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware::Next,
    routing::{delete, get, post, put},
    Json, Router,
};
use renzora_models::category::Category;
use renzora_models::dispute::{self, Dispute};
use renzora_models::forum::ForumCategory;
use renzora_models::user::User;
use renzora_models::asset::Asset;
use renzora_models::article::Article;
use renzora_models::doc::Doc;
use renzora_models::tag::Tag;
use renzora_models::subcategory::Subcategory;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};
use time::format_description::well_known::Rfc3339;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stats", get(admin_stats))
        .route("/users", get(list_users))
        .route("/users/:id/role", put(set_user_role))
        .route("/categories", get(list_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", put(update_category))
        .route("/categories/:id", delete(delete_category))
        .route("/assets", get(list_assets))
        .route("/assets/:id/publish", put(toggle_publish))
        .route("/assets/:id", delete(delete_asset))
        .route("/disputes", get(list_disputes))
        .route("/disputes/:id/resolve", put(resolve_dispute))
        .route("/settings", get(get_settings))
        .route("/settings", put(update_setting))
        .route("/docs", get(list_docs))
        .route("/docs", post(create_doc))
        .route("/docs/:id", put(update_doc))
        .route("/docs/:id", delete(delete_doc))
        .route("/forum-categories", get(list_forum_categories))
        .route("/forum-categories", post(create_forum_category))
        .route("/forum-categories/:id", delete(delete_forum_category))
        .route("/badges", get(list_badges))
        .route("/badges/:user_id/:badge_slug", post(award_badge))
        // Roles
        .route("/roles", get(list_roles))
        .route("/roles", post(create_role))
        .route("/roles/:id/permissions", put(update_role_permissions))
        .route("/roles/:id", delete(delete_role_handler))
        .route("/users/:id/roles", get(get_user_roles_handler))
        .route("/users/:user_id/roles/:role_id", post(assign_role_handler))
        .route("/users/:user_id/roles/:role_id", delete(remove_role_handler))
        // Reviews
        .route("/reviews/flagged", get(list_flagged_reviews))
        .route("/reviews/:id/hide", put(hide_review))
        .route("/reviews/:id/unhide", put(unhide_review))
        .route("/reviews/:id/dismiss", put(dismiss_review_flag))
        .route("/reviews/:id", delete(delete_review))
        // Withdrawals
        .route("/withdrawals", get(list_admin_withdrawals))
        .route("/withdrawals/:id/reject", put(reject_withdrawal))
        // Promo codes
        .route("/promo-codes", get(list_promo_codes))
        .route("/promo-codes", post(create_promo_code))
        .route("/promo-codes/:id/toggle", put(toggle_promo_code))
        .route("/promo-codes/:id", delete(delete_promo_code))
        // Full user edit
        .route("/users/:id/edit", put(edit_user_full))
        // Analytics
        .route("/analytics", get(admin_analytics))
        // Accept withdrawal
        .route("/withdrawals/:id/accept", put(accept_withdrawal))
        // Investigate withdrawal
        .route("/withdrawals/:id/transactions", get(withdrawal_transactions))
        // Badge creation
        .route("/badges/create", post(create_badge))
        .route("/badges/:id", delete(delete_badge))
        // Bans
        .route("/users/:id/ban", post(ban_user_handler))
        .route("/users/:id/unban", post(unban_user_handler))
        .route("/users/:id/bans", get(list_user_bans))
        // Mod notes
        .route("/users/:id/notes", get(get_mod_notes_handler))
        .route("/users/:id/notes", post(add_mod_note_handler))
        .route("/notes/:id", delete(delete_mod_note_handler))
        // User detail
        .route("/users/:id/detail", get(user_detail))
        .route("/users/:id/tokens", get(user_tokens))
        .route("/users/:id/tokens/:token_id", delete(revoke_user_token))
        .route("/users/:id/assets", get(user_assets))
        .route("/users/:id/purchases", get(user_purchases))
        .route("/users/:id/topups", get(user_topups))
        // Articles
        .route("/articles", get(list_admin_articles))
        .route("/articles/:id/publish", put(toggle_article_publish))
        .route("/articles/:id", delete(delete_article))
        // Tags
        .route("/tags/pending", get(list_pending_tags))
        .route("/tags/:id/approve", put(approve_tag))
        .route("/tags/:id", delete(delete_tag))
        // Subcategories
        .route("/subcategories", get(list_admin_subcategories))
        .route("/subcategories/pending", get(list_pending_subcategories))
        .route("/subcategories/:id/approve", put(approve_subcategory))
        .route("/subcategories/:id", delete(delete_subcategory))
        // Games
        .route("/games", get(list_admin_games))
        .route("/games/:id/publish", put(toggle_game_publish))
        .route("/games/:id", delete(delete_game))
        // Courses
        .route("/courses", get(list_admin_courses))
        .route("/courses/:id/publish", put(toggle_course_publish))
        .route("/courses/:id", delete(delete_course))
        // API Tokens (global)
        .route("/tokens", get(list_all_tokens))
        .route("/tokens/:id/suspend", put(suspend_token))
        .route("/tokens/:id", delete(admin_delete_token))
        // Developer Apps
        .route("/apps", get(list_all_apps))
        .route("/apps/:id/approve", put(toggle_app_approval))
        .route("/apps/:id/suspend", put(suspend_app))
        .route("/apps/:id/tokens", get(list_app_tokens_admin))
        .route("/app-tokens/:id/suspend", put(suspend_app_token))
        .route("/apps/:id/grants", get(list_app_grants))
        .route("/apps/:id/grants/:user_id", delete(revoke_app_grant))
        // Teams
        .route("/teams", get(list_all_teams))
        .route("/teams/:id", get(team_detail))
        .route("/teams/:id/members", get(team_members_admin))
        .route("/teams/:id/edit", put(edit_team))
        // Transactions
        .route("/transactions", get(list_all_transactions))
        .route("/users/:id/credit", post(credit_user))
        // Vouchers
        .route("/vouchers", get(list_vouchers))
        .route("/vouchers", post(create_voucher))
        .route("/vouchers/:id/toggle", put(toggle_voucher))
        .route("/vouchers/:id", delete(delete_voucher))
        // Enhanced Analytics
        .route("/analytics/monthly/:year/:month", get(monthly_analytics))
        .route("/analytics/timeseries", get(analytics_timeseries))
        .route("/analytics/business", get(analytics_business))
        .layer(axum::middleware::from_fn(require_admin))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// Middleware: check that the authenticated user has admin role.
async fn require_admin(
    mut req: axum::extract::Request,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    let auth = req.extensions().get::<AuthUser>().cloned().ok_or(StatusCode::UNAUTHORIZED)?;

    // We need the DB pool — grab it from the JwtSecret extension's sibling
    // Actually, we'll check the role in each handler since we need the pool.
    // For now, pass through and let handlers verify.
    Ok(next.run(req).await)
}

// ── Admin Stats ──

#[derive(Serialize)]
struct AdminStats {
    total_users: i64,
    total_assets: i64,
    total_transactions: i64,
    total_credits_circulating: i64,
    open_disputes: i64,
}

async fn admin_stats(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<AdminStats>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let users: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM users").fetch_one(&state.db).await?;
    let assets: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets").fetch_one(&state.db).await?;
    let txns: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions").fetch_one(&state.db).await?;
    let credits: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(credit_balance), 0)::bigint FROM users").fetch_one(&state.db).await?;
    let disputes: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM disputes WHERE status = 'open'").fetch_one(&state.db).await?;

    Ok(Json(AdminStats {
        total_users: users.0,
        total_assets: assets.0,
        total_transactions: txns.0,
        total_credits_circulating: credits.0,
        open_disputes: disputes.0,
    }))
}

// ── Users ──

#[derive(Deserialize)]
struct UserListQuery { page: Option<i64>, q: Option<String> }

#[derive(Serialize)]
struct UserEntry { id: Uuid, username: String, email: String, role: String, credit_balance: i64, created_at: String }

async fn list_users(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<UserListQuery>,
) -> Result<Json<Vec<UserEntry>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * 50;

    let rows = sqlx::query(
        "SELECT id, username, email, role, credit_balance, created_at FROM users WHERE ($1::text IS NULL OR username ILIKE '%' || $1 || '%' OR email ILIKE '%' || $1 || '%') ORDER BY created_at DESC LIMIT 50 OFFSET $2"
    ).bind(params.q.as_deref()).bind(offset).fetch_all(&state.db).await?;

    let users: Vec<UserEntry> = rows.iter().map(|r| UserEntry {
        id: r.get("id"), username: r.get("username"), email: r.get("email"),
        role: r.get("role"), credit_balance: r.get("credit_balance"),
        created_at: r.get::<time::OffsetDateTime, _>("created_at").to_string(),
    }).collect();

    Ok(Json(users))
}

#[derive(Deserialize)]
struct SetRoleBody { role: String }

async fn set_user_role(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<SetRoleBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    match body.role.as_str() {
        "user" | "creator" | "admin" => {}
        _ => return Err(ApiError::Validation("Invalid role".into())),
    }
    sqlx::query("UPDATE users SET role = $1 WHERE id = $2").bind(&body.role).bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Role updated"})))
}

// ── Categories ──

async fn list_categories(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Category>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let cats = Category::list(&state.db).await?;
    Ok(Json(cats))
}

#[derive(Deserialize)]
struct CreateCategoryBody { name: String, slug: String, description: String, icon: String, max_file_size_mb: Option<i32>, allowed_extensions: Option<Vec<String>> }

async fn create_category(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateCategoryBody>,
) -> Result<Json<Category>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let exts = body.allowed_extensions.unwrap_or_else(|| vec!["zip".into()]);
    let cat = Category::create(&state.db, &body.name, &body.slug, &body.description, &body.icon, body.max_file_size_mb.unwrap_or(50), &exts).await?;
    Ok(Json(cat))
}

#[derive(Deserialize)]
struct UpdateCategoryBody { name: Option<String>, description: Option<String>, icon: Option<String>, max_file_size_mb: Option<i32> }

async fn update_category(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCategoryBody>,
) -> Result<Json<Category>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let cat = Category::update(&state.db, id, body.name.as_deref(), body.description.as_deref(), body.icon.as_deref(), body.max_file_size_mb).await?;
    Ok(Json(cat))
}

async fn delete_category(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    Category::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Assets ──

#[derive(Deserialize)]
struct AssetListQuery { page: Option<i64>, q: Option<String>, published: Option<bool> }

async fn list_assets(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<AssetListQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * 50;

    let rows = sqlx::query(
        "SELECT a.id, a.name, a.slug, a.category, a.price_credits, a.downloads, a.published, a.created_at, u.username as creator_name FROM assets a JOIN users u ON u.id = a.creator_id WHERE ($1::text IS NULL OR a.name ILIKE '%' || $1 || '%') AND ($2::bool IS NULL OR a.published = $2) ORDER BY a.created_at DESC LIMIT 50 OFFSET $3"
    ).bind(params.q.as_deref()).bind(params.published).bind(offset).fetch_all(&state.db).await?;

    let assets: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "category": r.get::<String, _>("category"),
        "price_credits": r.get::<i64, _>("price_credits"),
        "downloads": r.get::<i64, _>("downloads"),
        "published": r.get::<bool, _>("published"),
        "creator_name": r.get::<String, _>("creator_name"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").to_string(),
    })).collect();

    Ok(Json(serde_json::json!({"assets": assets})))
}

async fn toggle_publish(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("UPDATE assets SET published = NOT published, updated_at = NOW() WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Toggled"})))
}

async fn delete_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM user_assets WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM transactions WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM assets WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Disputes ──

#[derive(Deserialize)]
struct DisputeQuery { status: Option<String>, page: Option<i64> }

async fn list_disputes(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<DisputeQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let (disputes, total) = Dispute::list_all(&state.db, params.status.as_deref(), params.page.unwrap_or(1), 20).await?;
    let entries: Vec<serde_json::Value> = disputes.iter().map(|d| serde_json::json!({
        "id": d.id, "user_id": d.user_id, "asset_id": d.asset_id, "reason": d.reason,
        "status": d.status, "admin_notes": d.admin_notes, "created_at": d.created_at.to_string(),
    })).collect();
    Ok(Json(serde_json::json!({"disputes": entries, "total": total})))
}

#[derive(Deserialize)]
struct ResolveBody { status: String, admin_notes: String }

async fn resolve_dispute(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ResolveBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    match body.status.as_str() {
        "resolved" | "rejected" | "refunded" => {}
        _ => return Err(ApiError::Validation("Status must be resolved, rejected, or refunded".into())),
    }
    Dispute::resolve(&state.db, id, &body.status, &body.admin_notes).await?;
    Ok(Json(serde_json::json!({"message": "Dispute resolved"})))
}

// ── Settings ──

async fn get_settings(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let settings = dispute::list_settings(&state.db).await?;
    let map: serde_json::Map<String, serde_json::Value> = settings.into_iter().map(|(k, v)| (k, serde_json::Value::String(v))).collect();
    Ok(Json(serde_json::Value::Object(map)))
}

#[derive(Deserialize)]
struct SettingBody { key: String, value: String }

async fn update_setting(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SettingBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    dispute::set_setting(&state.db, &body.key, &body.value).await?;
    Ok(Json(serde_json::json!({"message": "Setting updated"})))
}

// ── Docs management ──

#[derive(Deserialize)]
struct CreateDocBody { slug: String, title: String, content: String, category: String }

async fn list_docs(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let rows = sqlx::query("SELECT id, slug, title, category, published, sort_order, created_at FROM docs ORDER BY category, sort_order")
        .fetch_all(&state.db).await?;
    let docs: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"), "slug": r.get::<String, _>("slug"),
        "title": r.get::<String, _>("title"), "category": r.get::<String, _>("category"),
        "published": r.get::<bool, _>("published"), "sort_order": r.get::<i32, _>("sort_order"),
    })).collect();
    Ok(Json(serde_json::json!({"docs": docs})))
}

async fn create_doc(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateDocBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO docs (id, slug, title, content, category) VALUES ($1,$2,$3,$4,$5)")
        .bind(id).bind(&body.slug).bind(&body.title).bind(&body.content).bind(&body.category)
        .execute(&state.db).await?;
    Ok(Json(serde_json::json!({"id": id, "message": "Doc created"})))
}

#[derive(Deserialize)]
struct UpdateDocBody { title: Option<String>, content: Option<String>, category: Option<String>, published: Option<bool>, sort_order: Option<i32> }

async fn update_doc(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateDocBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("UPDATE docs SET title=COALESCE($2,title), content=COALESCE($3,content), category=COALESCE($4,category), published=COALESCE($5,published), sort_order=COALESCE($6,sort_order), updated_at=NOW() WHERE id=$1")
        .bind(id).bind(body.title).bind(body.content).bind(body.category).bind(body.published).bind(body.sort_order)
        .execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Doc updated"})))
}

async fn delete_doc(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM docs WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Forum categories ──

async fn list_forum_categories(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<ForumCategory>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let cats = ForumCategory::list(&state.db).await?;
    Ok(Json(cats))
}

#[derive(Deserialize)]
struct CreateForumCategoryBody { name: String, slug: String, description: String, icon: String }

async fn create_forum_category(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateForumCategoryBody>,
) -> Result<Json<ForumCategory>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let cat = ForumCategory::create(&state.db, &body.name, &body.slug, &body.description, &body.icon).await?;
    Ok(Json(cat))
}

async fn delete_forum_category(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    ForumCategory::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Badges ──

async fn list_badges(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let rows = sqlx::query("SELECT id, slug, name, description, icon, color FROM badges ORDER BY name")
        .fetch_all(&state.db).await?;
    let badges: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"), "slug": r.get::<String, _>("slug"),
        "name": r.get::<String, _>("name"), "description": r.get::<String, _>("description"),
        "icon": r.get::<String, _>("icon"), "color": r.get::<String, _>("color"),
    })).collect();
    Ok(Json(serde_json::json!({"badges": badges})))
}

async fn award_badge(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((user_id, badge_slug)): Path<(Uuid, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let badge: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM badges WHERE slug=$1").bind(&badge_slug).fetch_optional(&state.db).await?;
    let badge_id = badge.ok_or(ApiError::NotFound)?.0;
    sqlx::query("INSERT INTO user_badges (user_id, badge_id) VALUES ($1,$2) ON CONFLICT DO NOTHING")
        .bind(user_id).bind(badge_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Badge awarded"})))
}

// ── Roles ──

async fn list_roles(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<renzora_models::role::Role>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let roles = renzora_models::role::Role::list(&state.db).await?;
    Ok(Json(roles))
}

#[derive(Deserialize)]
struct CreateRoleBody { name: String, color: String, is_staff: bool, permissions: serde_json::Value }

async fn create_role(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateRoleBody>,
) -> Result<Json<renzora_models::role::Role>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let role = renzora_models::role::Role::create(&state.db, &body.name, &body.color, body.is_staff, body.permissions).await?;
    Ok(Json(role))
}

#[derive(Deserialize)]
struct UpdatePermissionsBody { permissions: serde_json::Value }

async fn update_role_permissions(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePermissionsBody>,
) -> Result<Json<renzora_models::role::Role>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let role = renzora_models::role::Role::update_permissions(&state.db, id, body.permissions).await?;
    Ok(Json(role))
}

async fn delete_role_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::role::Role::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

async fn get_user_roles_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<renzora_models::role::Role>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let roles = renzora_models::role::get_user_roles(&state.db, id).await?;
    Ok(Json(roles))
}

async fn assign_role_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::role::assign_role(&state.db, user_id, role_id, auth.user_id).await?;
    Ok(Json(serde_json::json!({"message": "Role assigned"})))
}

async fn remove_role_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::role::remove_role(&state.db, user_id, role_id).await?;
    Ok(Json(serde_json::json!({"message": "Role removed"})))
}

// ── Bans ──

#[derive(Deserialize)]
struct BanBody { reason: String, ban_type: Option<String>, duration_hours: Option<i64> }

async fn ban_user_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<BanBody>,
) -> Result<Json<renzora_models::role::Ban>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let ban = renzora_models::role::ban_user(&state.db, id, auth.user_id, &body.reason, body.ban_type.as_deref().unwrap_or("full"), body.duration_hours).await?;
    Ok(Json(ban))
}

async fn unban_user_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::role::unban_user(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Unbanned"})))
}

async fn list_user_bans(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<renzora_models::role::Ban>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let bans = renzora_models::role::list_bans(&state.db, id).await?;
    Ok(Json(bans))
}

// ── Mod Notes ──

#[derive(Deserialize)]
struct ModNoteBody { content: String }

async fn get_mod_notes_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<renzora_models::role::ModNoteWithAuthor>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let notes = renzora_models::role::get_mod_notes(&state.db, id).await?;
    Ok(Json(notes))
}

async fn add_mod_note_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ModNoteBody>,
) -> Result<Json<renzora_models::role::ModNote>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let note = renzora_models::role::add_mod_note(&state.db, id, auth.user_id, &body.content).await?;
    Ok(Json(note))
}

async fn delete_mod_note_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::role::delete_mod_note(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Reviews Moderation ──

async fn list_flagged_reviews(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<renzora_models::review::FlaggedReview>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let reviews = renzora_models::review::Review::list_flagged(&state.db).await?;
    Ok(Json(reviews))
}

async fn hide_review(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::review::Review::set_hidden(&state.db, id, true).await?;
    Ok(Json(serde_json::json!({"message": "Review hidden"})))
}

async fn unhide_review(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::review::Review::set_hidden(&state.db, id, false).await?;
    Ok(Json(serde_json::json!({"message": "Review unhidden"})))
}

async fn dismiss_review_flag(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::review::Review::dismiss_flag(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Flag dismissed"})))
}

async fn delete_review(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::review::Review::delete(&state.db, id)
        .await
        .map_err(|e| ApiError::Internal(e))?;
    Ok(Json(serde_json::json!({"message": "Review deleted"})))
}

// ── Withdrawals ──

async fn list_admin_withdrawals(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<DisputeQuery>,
) -> Result<Json<Vec<renzora_models::withdrawal::WithdrawalWithUser>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let withdrawals = renzora_models::withdrawal::Withdrawal::list_all(&state.db, params.status.as_deref()).await?;
    Ok(Json(withdrawals))
}

#[derive(Deserialize)]
struct RejectWithdrawalBody { reason: String }

async fn reject_withdrawal(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<RejectWithdrawalBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::withdrawal::Withdrawal::mark_failed(&state.db, id, &body.reason)
        .await
        .map_err(|e| ApiError::Validation(e))?;
    Ok(Json(serde_json::json!({"message": "Withdrawal rejected and credits refunded"})))
}

// ── Promo Codes ──

async fn list_promo_codes(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<renzora_models::promo_code::PromoCode>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let codes = renzora_models::promo_code::PromoCode::list(&state.db).await?;
    Ok(Json(codes))
}

#[derive(Deserialize)]
struct CreatePromoCodeBody {
    code: String,
    discount_percent: i32,
    max_uses: Option<i32>,
    expires_hours: Option<i64>,
}

async fn create_promo_code(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreatePromoCodeBody>,
) -> Result<Json<renzora_models::promo_code::PromoCode>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    if body.code.trim().is_empty() || body.code.len() > 32 {
        return Err(ApiError::Validation("Code must be 1-32 characters".into()));
    }
    if body.discount_percent < 1 || body.discount_percent > 20 {
        return Err(ApiError::Validation("Discount must be 1-20%".into()));
    }
    let expires_at = body.expires_hours.map(|h| {
        time::OffsetDateTime::now_utc() + time::Duration::hours(h)
    });
    let code = renzora_models::promo_code::PromoCode::create(
        &state.db,
        &body.code,
        body.discount_percent,
        body.max_uses,
        expires_at,
        auth.user_id,
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            ApiError::Validation("A promo code with that name already exists".into())
        } else {
            ApiError::Internal(e.to_string())
        }
    })?;
    Ok(Json(code))
}

async fn toggle_promo_code(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    // Toggle: fetch current, flip
    let current = sqlx::query_as::<_, (bool,)>(
        "SELECT active FROM promo_codes WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    renzora_models::promo_code::PromoCode::set_active(&state.db, id, !current.0).await?;
    Ok(Json(serde_json::json!({"message": "Toggled", "active": !current.0})))
}

async fn delete_promo_code(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::promo_code::PromoCode::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Full user edit ──

#[derive(Deserialize)]
struct EditUserBody {
    username: Option<String>,
    email: Option<String>,
    role: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    website: Option<String>,
    gender: Option<String>,
    profile_color: Option<String>,
    banner_color: Option<String>,
    credit_balance: Option<i64>,
}

async fn edit_user_full(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<EditUserBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query(
        "UPDATE users SET username=COALESCE($2,username), email=COALESCE($3,email), role=COALESCE($4,role), bio=COALESCE($5,bio), location=COALESCE($6,location), website=COALESCE($7,website), gender=COALESCE($8,gender), profile_color=COALESCE($9,profile_color), banner_color=COALESCE($10,banner_color), credit_balance=COALESCE($11,credit_balance), updated_at=NOW() WHERE id=$1"
    )
    .bind(id).bind(&body.username).bind(&body.email).bind(&body.role)
    .bind(&body.bio).bind(&body.location).bind(&body.website).bind(&body.gender)
    .bind(&body.profile_color).bind(&body.banner_color).bind(body.credit_balance)
    .execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "User updated"})))
}

// ── Analytics ──

async fn admin_analytics(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    // Total revenue (all topups)
    let revenue: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='topup'").fetch_one(&state.db).await?;
    // Total purchases
    let purchases: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(ABS(amount)),0)::bigint FROM transactions WHERE type='purchase'").fetch_one(&state.db).await?;
    // Total earnings paid to creators
    let creator_earnings: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='earning'").fetch_one(&state.db).await?;
    // Platform commission = purchases - creator_earnings
    let commission = purchases.0 - creator_earnings.0;
    // Completed withdrawals
    let withdrawn: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount_credits),0)::bigint FROM withdrawals WHERE status='completed'").fetch_one(&state.db).await?;
    // Pending withdrawals
    let pending_withdrawals: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount_credits),0)::bigint FROM withdrawals WHERE status IN ('pending','processing')").fetch_one(&state.db).await?;
    // Total sales count
    let sales_count: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE type='purchase'").fetch_one(&state.db).await?;
    // Referral earnings
    let referral_total: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(referral_amount),0)::bigint FROM referral_earnings").fetch_one(&state.db).await?;
    // Monthly revenue (last 12 months)
    let monthly_rows = sqlx::query(
        "SELECT date_trunc('month', created_at) as month, type, COALESCE(SUM(ABS(amount)),0)::bigint as total FROM transactions WHERE created_at > NOW() - INTERVAL '12 months' GROUP BY month, type ORDER BY month"
    ).fetch_all(&state.db).await?;

    let mut monthly: Vec<serde_json::Value> = Vec::new();
    for row in &monthly_rows {
        monthly.push(serde_json::json!({
            "month": row.get::<time::OffsetDateTime, _>("month").format(&Rfc3339).unwrap_or_default(),
            "type": row.get::<String, _>("type"),
            "total": row.get::<i64, _>("total"),
        }));
    }

    // Top selling assets
    let top_assets = sqlx::query(
        "SELECT a.name, a.slug, COUNT(t.id)::bigint as sales, COALESCE(SUM(ABS(t.amount)),0)::bigint as revenue FROM transactions t JOIN assets a ON a.id=t.asset_id WHERE t.type='purchase' GROUP BY a.id, a.name, a.slug ORDER BY sales DESC LIMIT 10"
    ).fetch_all(&state.db).await?;

    let top: Vec<serde_json::Value> = top_assets.iter().map(|r| serde_json::json!({
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "sales": r.get::<i64, _>("sales"),
        "revenue": r.get::<i64, _>("revenue"),
    })).collect();

    Ok(Json(serde_json::json!({
        "total_revenue": revenue.0,
        "total_purchases": purchases.0,
        "creator_earnings": creator_earnings.0,
        "platform_commission": commission,
        "withdrawn": withdrawn.0,
        "pending_withdrawals": pending_withdrawals.0,
        "sales_count": sales_count.0,
        "referral_total": referral_total.0,
        "monthly": monthly,
        "top_assets": top,
    })))
}

// ── Accept Withdrawal ──

async fn accept_withdrawal(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("UPDATE withdrawals SET status='completed', updated_at=NOW() WHERE id=$1 AND status IN ('pending','processing')")
        .bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Withdrawal accepted"})))
}

// ── Investigate Withdrawal Transactions ──

async fn withdrawal_transactions(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    // Get the withdrawal's user
    let w = sqlx::query("SELECT user_id, amount_credits FROM withdrawals WHERE id=$1")
        .bind(id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;
    let uid: Uuid = w.get("user_id");
    let amount: i64 = w.get("amount_credits");
    // Get recent transactions for this user
    let txns = sqlx::query(
        "SELECT id, type, amount, asset_id, created_at FROM transactions WHERE user_id=$1 ORDER BY created_at DESC LIMIT 50"
    ).bind(uid).fetch_all(&state.db).await?;
    let entries: Vec<serde_json::Value> = txns.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "type": r.get::<String, _>("type"),
        "amount": r.get::<i64, _>("amount"),
        "asset_id": r.get::<Option<Uuid>, _>("asset_id"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
    })).collect();
    Ok(Json(serde_json::json!({
        "withdrawal_amount": amount,
        "transactions": entries,
    })))
}

// ── Badge Creation ──

#[derive(Deserialize)]
struct CreateBadgeBody {
    slug: String,
    name: String,
    description: String,
    icon: String,
    color: String,
    auto_rule: Option<String>,
    auto_threshold: Option<i64>,
}

async fn create_badge(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateBadgeBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
        .bind(id).bind(&body.slug).bind(&body.name).bind(&body.description)
        .bind(&body.icon).bind(&body.color)
        .bind(&body.auto_rule).bind(body.auto_threshold)
        .execute(&state.db).await.map_err(|e| {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                ApiError::Validation("A badge with that slug already exists".into())
            } else {
                ApiError::Internal(e.to_string())
            }
        })?;
    Ok(Json(serde_json::json!({"id": id, "message": "Badge created"})))
}

async fn delete_badge(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM user_badges WHERE badge_id=$1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM badges WHERE id=$1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Badge deleted"})))
}

// ── Helper ──

async fn verify_admin(state: &AppState, user_id: Uuid) -> Result<(), ApiError> {
    // Check legacy role column OR permission-based roles
    let user = User::find_by_id(&state.db, user_id).await?.ok_or(ApiError::NotFound)?;
    if user.role == "admin" { return Ok(()); }
    if renzora_models::role::has_permission(&state.db, user_id, "view_admin").await.unwrap_or(false) {
        return Ok(());
    }
    Err(ApiError::Unauthorized)
}

// ── Articles ──

async fn list_admin_articles(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let q = params.get("q").cloned().unwrap_or_default();
    let rows = sqlx::query_as::<_, Article>(
        r#"
        SELECT * FROM articles
        WHERE ($1 = '' OR title ILIKE '%' || $1 || '%')
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .bind(&q)
    .fetch_all(&state.db)
    .await?;

    // Look up author names
    let mut articles: Vec<serde_json::Value> = Vec::new();
    for a in &rows {
        let author_name = User::find_by_id(&state.db, a.author_id).await
            .ok().flatten()
            .map(|u| u.username)
            .unwrap_or_else(|| "Unknown".to_string());
        articles.push(serde_json::json!({
            "id": a.id,
            "title": a.title,
            "slug": a.slug,
            "author_name": author_name,
            "published": a.published,
            "tags": a.tags,
            "likes": a.likes,
            "views": a.views,
            "created_at": a.created_at.to_string(),
        }));
    }

    Ok(Json(serde_json::json!({ "articles": articles })))
}

async fn toggle_article_publish(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("UPDATE articles SET published = NOT published WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("DELETE FROM article_comments WHERE article_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM article_likes WHERE article_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM articles WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

// ── Tags ──

async fn list_pending_tags(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Tag>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let tags = Tag::list_pending(&state.db).await?;
    Ok(Json(tags))
}

async fn approve_tag(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    Tag::approve(&state.db, id).await?;
    Ok(Json(serde_json::json!({ "message": "Tag approved" })))
}

async fn delete_tag(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    Tag::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({ "message": "Tag deleted" })))
}

// ── Subcategories ──

async fn list_admin_subcategories(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Subcategory>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let subs = Subcategory::list_all_approved(&state.db).await?;
    Ok(Json(subs))
}

async fn list_pending_subcategories(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Subcategory>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let subs = Subcategory::list_pending(&state.db).await?;
    Ok(Json(subs))
}

async fn approve_subcategory(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    Subcategory::approve(&state.db, id).await?;
    Ok(Json(serde_json::json!({ "message": "Subcategory approved" })))
}

async fn delete_subcategory(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    Subcategory::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({ "message": "Subcategory deleted" })))
}

// ── Games ──

#[derive(Serialize)]
struct AdminGameEntry {
    id: Uuid,
    name: String,
    slug: String,
    category: String,
    price_credits: i64,
    downloads: i64,
    published: bool,
    creator_name: String,
    created_at: String,
}

async fn list_admin_games(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page: i64 = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1).max(1);
    let offset = (page - 1) * 50;
    let q = params.get("q");
    let published = params.get("published");

    let mut sql = String::from(
        "SELECT g.id, g.name, g.slug, g.category, g.price_credits, g.downloads, g.published, g.created_at, u.username AS creator_name \
         FROM games g JOIN users u ON u.id = g.creator_id WHERE 1=1"
    );
    if let Some(q) = q {
        if !q.is_empty() {
            sql.push_str(&format!(" AND g.name ILIKE '%{}%'", q.replace('\'', "''")));
        }
    }
    if let Some(p) = published {
        if p == "true" { sql.push_str(" AND g.published = true"); }
        else if p == "false" { sql.push_str(" AND g.published = false"); }
    }
    sql.push_str(&format!(" ORDER BY g.created_at DESC LIMIT 50 OFFSET {}", offset));

    let rows = sqlx::query(&sql).fetch_all(&state.db).await?;
    let games: Vec<AdminGameEntry> = rows.iter().map(|r| AdminGameEntry {
        id: r.get("id"), name: r.get("name"), slug: r.get("slug"),
        category: r.get("category"), price_credits: r.get("price_credits"),
        downloads: r.get("downloads"), published: r.get("published"),
        creator_name: r.get("creator_name"),
        created_at: r.get::<time::OffsetDateTime, _>("created_at").to_string(),
    }).collect();

    Ok(Json(serde_json::json!({ "games": games })))
}

async fn toggle_game_publish(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("UPDATE games SET published = NOT published WHERE id = $1")
        .bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({ "message": "Toggled" })))
}

async fn delete_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM game_media WHERE game_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM user_games WHERE game_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM games WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({ "message": "Game deleted" })))
}

// ── Courses ──

async fn list_admin_courses(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page: i64 = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1).max(1);
    let offset = (page - 1) * 50;
    let q = params.get("q");

    let mut sql = String::from(
        "SELECT c.id, c.title, c.slug, c.category, c.difficulty, c.price_credits, c.published, c.chapter_count, c.enrolled_count, c.created_at, u.username AS creator_name \
         FROM courses c JOIN users u ON u.id = c.creator_id WHERE 1=1"
    );
    if let Some(q) = q {
        if !q.is_empty() {
            sql.push_str(&format!(" AND c.title ILIKE '%{}%'", q.replace('\'', "''")));
        }
    }
    sql.push_str(&format!(" ORDER BY c.created_at DESC LIMIT 50 OFFSET {}", offset));

    let rows = sqlx::query(&sql).fetch_all(&state.db).await?;
    let courses: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "title": r.get::<String, _>("title"),
        "slug": r.get::<String, _>("slug"),
        "category": r.get::<String, _>("category"),
        "difficulty": r.get::<String, _>("difficulty"),
        "price_credits": r.get::<i64, _>("price_credits"),
        "published": r.get::<bool, _>("published"),
        "chapter_count": r.get::<i32, _>("chapter_count"),
        "enrolled_count": r.get::<i32, _>("enrolled_count"),
        "creator_name": r.get::<String, _>("creator_name"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").to_string(),
    })).collect();

    Ok(Json(serde_json::json!({ "courses": courses })))
}

async fn toggle_course_publish(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("UPDATE courses SET published = NOT published WHERE id = $1")
        .bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({ "message": "Toggled" })))
}

async fn delete_course(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM course_chapters WHERE course_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM enrollments WHERE course_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM courses WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({ "message": "Course deleted" })))
}

// ── User Detail ──

async fn user_detail(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let user = User::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let asset_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM assets WHERE creator_id = $1")
        .bind(id).fetch_one(&state.db).await?;

    let purchase_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_assets WHERE user_id = $1")
        .bind(id).fetch_one(&state.db).await?;

    let token_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_tokens WHERE user_id = $1")
        .bind(id).fetch_one(&state.db).await?;

    // Get subscription plan info
    let plan_row = sqlx::query_as::<_, (String, i32)>(
        "SELECT sp.name, sp.daily_api_limit FROM subscription_plans sp JOIN subscriptions s ON s.plan_id = sp.id WHERE s.user_id = $1 AND s.status = 'active'"
    ).bind(id).fetch_optional(&state.db).await.unwrap_or(None);

    // Get today's API usage
    let usage_row = sqlx::query_as::<_, (i32,)>(
        "SELECT request_count FROM api_usage_daily WHERE user_id = $1 AND date = CURRENT_DATE"
    ).bind(id).fetch_optional(&state.db).await.unwrap_or(None);

    Ok(Json(serde_json::json!({
        "id": user.id,
        "username": user.username,
        "email": user.email,
        "role": user.role,
        "credit_balance": user.credit_balance,
        "avatar_url": user.avatar_url,
        "discord_id": user.discord_id,
        "discord_username": user.discord_username,
        "stripe_connect_id": user.stripe_connect_id,
        "stripe_connect_onboarded": user.stripe_connect_onboarded,
        "totp_enabled": user.totp_enabled,
        "referral_code": user.referral_code,
        "referred_by": user.referred_by,
        "created_at": user.created_at.format(&Rfc3339).unwrap_or_default(),
        "updated_at": user.updated_at.format(&Rfc3339).unwrap_or_default(),
        "asset_count": asset_count.0,
        "purchase_count": purchase_count.0,
        "token_count": token_count.0,
        "subscription_plan": plan_row.as_ref().map(|r| &r.0),
        "daily_api_limit": plan_row.as_ref().map(|r| r.1),
        "api_usage_today": usage_row.map(|r| r.0).unwrap_or(0),
    })))
}

async fn user_tokens(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let rows = sqlx::query_as::<_, (Uuid, String, String, Vec<String>, Option<time::OffsetDateTime>, Option<time::OffsetDateTime>, time::OffsetDateTime)>(
        "SELECT id, name, prefix, scopes, last_used_at, expires_at, created_at FROM api_tokens WHERE user_id = $1 ORDER BY created_at DESC"
    ).bind(id).fetch_all(&state.db).await?;

    let tokens: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.0,
        "name": r.1,
        "prefix": r.2,
        "scopes": r.3,
        "last_used_at": r.4.map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "expires_at": r.5.map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "created_at": r.6.format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(tokens)))
}

async fn revoke_user_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((user_id, token_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    sqlx::query("DELETE FROM api_tokens WHERE id = $1 AND user_id = $2")
        .bind(token_id).bind(user_id)
        .execute(&state.db).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

async fn user_assets(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let rows = sqlx::query_as::<_, (Uuid, String, String, i64, i64, bool, time::OffsetDateTime)>(
        "SELECT id, name, slug, price_credits, downloads, published, created_at FROM assets WHERE creator_id = $1 ORDER BY created_at DESC LIMIT 100"
    ).bind(id).fetch_all(&state.db).await?;

    let assets: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.0,
        "name": r.1,
        "slug": r.2,
        "price_credits": r.3,
        "downloads": r.4,
        "published": r.5,
        "created_at": r.6.format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(assets)))
}

async fn user_purchases(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let rows = sqlx::query_as::<_, (Uuid, String, String, i64, time::OffsetDateTime)>(
        "SELECT a.id, a.name, a.slug, a.price_credits, ua.purchased_at FROM user_assets ua JOIN assets a ON a.id = ua.asset_id WHERE ua.user_id = $1 ORDER BY ua.purchased_at DESC LIMIT 100"
    ).bind(id).fetch_all(&state.db).await?;

    let purchases: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.0,
        "name": r.1,
        "slug": r.2,
        "price_credits": r.3,
        "purchased_at": r.4.format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(purchases)))
}

async fn user_topups(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let rows = sqlx::query_as::<_, (Uuid, i64, String, time::OffsetDateTime)>(
        "SELECT id, amount, type, created_at FROM transactions WHERE user_id = $1 AND type = 'topup' ORDER BY created_at DESC LIMIT 100"
    ).bind(id).fetch_all(&state.db).await?;

    let topups: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.0,
        "amount": r.1,
        "type": r.2,
        "created_at": r.3.format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(topups)))
}

// ── API Tokens (global) ──

#[derive(Deserialize)]
struct PaginatedQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    q: Option<String>,
}

async fn list_all_tokens(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<PaginatedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let search = params.q.unwrap_or_default();
    let like = format!("%{}%", search);

    let rows = sqlx::query(
        "SELECT t.id, t.user_id, t.name, t.prefix, t.scopes, t.last_used_at, t.expires_at, t.created_at, t.suspended, u.username, u.email FROM api_tokens t JOIN users u ON u.id = t.user_id WHERE ($1 = '' OR t.name ILIKE $2 OR t.prefix ILIKE $2 OR u.username ILIKE $2) ORDER BY t.created_at DESC LIMIT $3 OFFSET $4"
    ).bind(&search).bind(&like).bind(per_page).bind(offset)
    .fetch_all(&state.db).await?;

    let tokens: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "user_id": r.get::<Uuid, _>("user_id"),
        "name": r.get::<String, _>("name"),
        "prefix": r.get::<String, _>("prefix"),
        "scopes": r.get::<Vec<String>, _>("scopes"),
        "last_used_at": r.get::<Option<time::OffsetDateTime>, _>("last_used_at").map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "expires_at": r.get::<Option<time::OffsetDateTime>, _>("expires_at").map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
        "suspended": r.get::<bool, _>("suspended"),
        "username": r.get::<String, _>("username"),
        "email": r.get::<String, _>("email"),
    })).collect();

    Ok(Json(serde_json::json!(tokens)))
}

#[derive(Deserialize)]
struct SuspendBody {
    suspended: bool,
}

async fn suspend_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<SuspendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::api_token::ApiToken::set_suspended(&state.db, id, body.suspended).await?;
    Ok(Json(serde_json::json!({"ok": true, "suspended": body.suspended})))
}

async fn admin_delete_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM api_tokens WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Developer Apps ──

async fn list_all_apps(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<PaginatedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;
    let search = params.q.unwrap_or_default();
    let like = format!("%{}%", search);

    let rows = sqlx::query(
        "SELECT a.id, a.owner_id, a.name, a.slug, a.description, a.approved, a.suspended, a.created_at, u.username, (SELECT COUNT(*) FROM app_tokens WHERE app_id = a.id) as token_count FROM developer_apps a JOIN users u ON u.id = a.owner_id WHERE ($1 = '' OR a.name ILIKE $2 OR u.username ILIKE $2) ORDER BY a.created_at DESC LIMIT $3 OFFSET $4"
    ).bind(&search).bind(&like).bind(per_page).bind(offset)
    .fetch_all(&state.db).await?;

    let apps: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "owner_id": r.get::<Uuid, _>("owner_id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "description": r.get::<String, _>("description"),
        "approved": r.get::<bool, _>("approved"),
        "suspended": r.get::<bool, _>("suspended"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
        "username": r.get::<String, _>("username"),
        "token_count": r.get::<i64, _>("token_count"),
    })).collect();

    Ok(Json(serde_json::json!(apps)))
}

async fn toggle_app_approval(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let current: (bool,) = sqlx::query_as("SELECT approved FROM developer_apps WHERE id = $1")
        .bind(id).fetch_one(&state.db).await?;
    renzora_models::developer_app::DeveloperApp::set_approved(&state.db, id, !current.0).await?;
    Ok(Json(serde_json::json!({"ok": true, "approved": !current.0})))
}

async fn suspend_app(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<SuspendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::developer_app::DeveloperApp::set_suspended(&state.db, id, body.suspended).await?;
    Ok(Json(serde_json::json!({"ok": true, "suspended": body.suspended})))
}

async fn list_app_tokens_admin(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let rows = sqlx::query(
        "SELECT id, name, prefix, scopes, last_used_at, expires_at, created_at, suspended FROM app_tokens WHERE app_id = $1 ORDER BY created_at DESC"
    ).bind(id).fetch_all(&state.db).await?;

    let tokens: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "prefix": r.get::<String, _>("prefix"),
        "scopes": r.get::<Vec<String>, _>("scopes"),
        "last_used_at": r.get::<Option<time::OffsetDateTime>, _>("last_used_at").map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "expires_at": r.get::<Option<time::OffsetDateTime>, _>("expires_at").map(|t| t.format(&Rfc3339).unwrap_or_default()),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
        "suspended": r.get::<bool, _>("suspended"),
    })).collect();

    Ok(Json(serde_json::json!(tokens)))
}

async fn suspend_app_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<SuspendBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::developer_app::AppToken::set_suspended(&state.db, id, body.suspended).await?;
    Ok(Json(serde_json::json!({"ok": true, "suspended": body.suspended})))
}

async fn list_app_grants(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let rows = sqlx::query(
        "SELECT g.id, g.user_id, g.scopes_granted, g.granted_at, u.username FROM app_user_grants g JOIN users u ON u.id = g.user_id WHERE g.app_id = $1 ORDER BY g.granted_at DESC"
    ).bind(id).fetch_all(&state.db).await?;

    let grants: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "user_id": r.get::<Uuid, _>("user_id"),
        "username": r.get::<String, _>("username"),
        "scopes_granted": r.get::<Vec<String>, _>("scopes_granted"),
        "granted_at": r.get::<time::OffsetDateTime, _>("granted_at").format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(grants)))
}

async fn revoke_app_grant(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((app_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    sqlx::query("DELETE FROM app_user_grants WHERE app_id = $1 AND user_id = $2")
        .bind(app_id).bind(user_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Teams ──

async fn list_all_teams(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<PaginatedQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;
    let search = params.q.unwrap_or_default();
    let like = format!("%{}%", search);

    let rows = sqlx::query(
        "SELECT t.id, t.name, t.slug, t.owner_id, t.description, t.created_at, u.username as owner_name, (SELECT COUNT(*) FROM team_members WHERE team_id = t.id) as member_count FROM teams t JOIN users u ON u.id = t.owner_id WHERE ($1 = '' OR t.name ILIKE $2 OR u.username ILIKE $2) ORDER BY t.created_at DESC LIMIT $3 OFFSET $4"
    ).bind(&search).bind(&like).bind(per_page).bind(offset)
    .fetch_all(&state.db).await?;

    let teams: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "name": r.get::<String, _>("name"),
        "slug": r.get::<String, _>("slug"),
        "owner_id": r.get::<Uuid, _>("owner_id"),
        "owner_name": r.get::<String, _>("owner_name"),
        "description": r.get::<String, _>("description"),
        "member_count": r.get::<i64, _>("member_count"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(teams)))
}

async fn team_detail(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let row = sqlx::query(
        "SELECT t.*, u.username as owner_name, (SELECT COUNT(*) FROM team_members WHERE team_id = t.id) as member_count, (SELECT COUNT(*) FROM team_library WHERE team_id = t.id) as library_count FROM teams t JOIN users u ON u.id = t.owner_id WHERE t.id = $1"
    ).bind(id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>("id"),
        "name": row.get::<String, _>("name"),
        "slug": row.get::<String, _>("slug"),
        "owner_id": row.get::<Uuid, _>("owner_id"),
        "owner_name": row.get::<String, _>("owner_name"),
        "description": row.get::<String, _>("description"),
        "member_count": row.get::<i64, _>("member_count"),
        "library_count": row.get::<i64, _>("library_count"),
        "created_at": row.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
        "updated_at": row.get::<time::OffsetDateTime, _>("updated_at").format(&Rfc3339).unwrap_or_default(),
    })))
}

async fn team_members_admin(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let rows = sqlx::query(
        "SELECT tm.user_id, tm.role, tm.joined_at, u.username, u.email FROM team_members tm JOIN users u ON u.id = tm.user_id WHERE tm.team_id = $1 ORDER BY tm.joined_at"
    ).bind(id).fetch_all(&state.db).await?;

    let members: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "user_id": r.get::<Uuid, _>("user_id"),
        "role": r.get::<String, _>("role"),
        "username": r.get::<String, _>("username"),
        "email": r.get::<String, _>("email"),
        "joined_at": r.get::<time::OffsetDateTime, _>("joined_at").format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(members)))
}

#[derive(Deserialize)]
struct EditTeamBody {
    name: Option<String>,
    description: Option<String>,
}

async fn edit_team(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<EditTeamBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    if let Some(name) = &body.name {
        sqlx::query("UPDATE teams SET name = $1, updated_at = NOW() WHERE id = $2")
            .bind(name).bind(id).execute(&state.db).await?;
    }
    if let Some(desc) = &body.description {
        sqlx::query("UPDATE teams SET description = $1, updated_at = NOW() WHERE id = $2")
            .bind(desc).bind(id).execute(&state.db).await?;
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Transactions ──

#[derive(Deserialize)]
struct TransactionQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    #[serde(rename = "type")]
    tx_type: Option<String>,
    user_id: Option<Uuid>,
    from: Option<String>,
    to: Option<String>,
    q: Option<String>,
}

async fn list_all_transactions(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<TransactionQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;
    let tx_type = params.tx_type.unwrap_or_default();
    let search = params.q.unwrap_or_default();
    let like = format!("%{}%", search);

    let rows = sqlx::query(
        "SELECT t.id, t.user_id, t.type, t.amount, t.asset_id, t.stripe_payment_id, t.reason, t.admin_id, t.created_at, u.username, a.name as asset_name \
         FROM transactions t \
         JOIN users u ON u.id = t.user_id \
         LEFT JOIN assets a ON a.id = t.asset_id \
         WHERE ($1 = '' OR t.type = $1) \
         AND ($2::uuid IS NULL OR t.user_id = $2) \
         AND ($3 = '' OR u.username ILIKE $4) \
         ORDER BY t.created_at DESC LIMIT $5 OFFSET $6"
    )
    .bind(&tx_type)
    .bind(params.user_id)
    .bind(&search)
    .bind(&like)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db).await?;

    let txns: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "user_id": r.get::<Uuid, _>("user_id"),
        "username": r.get::<String, _>("username"),
        "type": r.get::<String, _>("type"),
        "amount": r.get::<i64, _>("amount"),
        "asset_id": r.get::<Option<Uuid>, _>("asset_id"),
        "asset_name": r.get::<Option<String>, _>("asset_name"),
        "stripe_payment_id": r.get::<Option<String>, _>("stripe_payment_id"),
        "reason": r.get::<Option<String>, _>("reason"),
        "admin_id": r.get::<Option<Uuid>, _>("admin_id"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").format(&Rfc3339).unwrap_or_default(),
    })).collect();

    Ok(Json(serde_json::json!(txns)))
}

#[derive(Deserialize)]
struct CreditUserBody {
    amount: i64,
    reason: String,
}

async fn credit_user(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreditUserBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    if body.amount <= 0 {
        return Err(ApiError::Validation("Amount must be positive".into()));
    }
    if body.reason.trim().is_empty() {
        return Err(ApiError::Validation("Reason is required".into()));
    }

    let tx_id = Uuid::new_v4();
    sqlx::query("INSERT INTO transactions (id, user_id, type, amount, reason, admin_id) VALUES ($1, $2, 'admin_credit', $3, $4, $5)")
        .bind(tx_id).bind(id).bind(body.amount).bind(&body.reason).bind(auth.user_id)
        .execute(&state.db).await?;
    sqlx::query("UPDATE users SET credit_balance = credit_balance + $1 WHERE id = $2")
        .bind(body.amount).bind(id).execute(&state.db).await?;

    Ok(Json(serde_json::json!({"ok": true, "transaction_id": tx_id})))
}

// ── Vouchers ──

#[derive(Deserialize)]
struct CreateVoucherBody {
    code: String,
    voucher_type: String,
    credit_amount: Option<i64>,
    discount_percent: Option<i32>,
    max_asset_price: Option<i64>,
    specific_asset_id: Option<Uuid>,
    max_uses: Option<i32>,
    max_uses_per_user: Option<i32>,
    expires_hours: Option<i64>,
}

async fn list_vouchers(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<renzora_models::voucher::Voucher>>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let vouchers = renzora_models::voucher::Voucher::list(&state.db).await?;
    Ok(Json(vouchers))
}

async fn create_voucher(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateVoucherBody>,
) -> Result<Json<renzora_models::voucher::Voucher>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    if body.code.trim().is_empty() || body.code.len() > 32 {
        return Err(ApiError::Validation("Code must be 1-32 characters".into()));
    }
    match body.voucher_type.as_str() {
        "credit" => {
            if body.credit_amount.unwrap_or(0) <= 0 {
                return Err(ApiError::Validation("Credit amount must be positive".into()));
            }
        }
        "asset_discount" => {
            let pct = body.discount_percent.unwrap_or(0);
            if pct < 1 || pct > 100 {
                return Err(ApiError::Validation("Discount must be 1-100%".into()));
            }
        }
        _ => return Err(ApiError::Validation("voucher_type must be 'credit' or 'asset_discount'".into())),
    }
    let expires_at = body.expires_hours.map(|h| time::OffsetDateTime::now_utc() + time::Duration::hours(h));
    let v = renzora_models::voucher::Voucher::create(
        &state.db, &body.code, &body.voucher_type, body.credit_amount, body.discount_percent,
        body.max_asset_price, body.specific_asset_id, body.max_uses,
        body.max_uses_per_user.unwrap_or(1), expires_at, auth.user_id,
    ).await.map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            ApiError::Validation("A voucher with that code already exists".into())
        } else {
            ApiError::Database(e)
        }
    })?;
    Ok(Json(v))
}

async fn toggle_voucher(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let current: (bool,) = sqlx::query_as("SELECT active FROM vouchers WHERE id = $1")
        .bind(id).fetch_one(&state.db).await?;
    renzora_models::voucher::Voucher::set_active(&state.db, id, !current.0).await?;
    Ok(Json(serde_json::json!({"ok": true, "active": !current.0})))
}

async fn delete_voucher(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    renzora_models::voucher::Voucher::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Enhanced Analytics ──

async fn monthly_analytics(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((year, month)): Path<(i32, u32)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    let month_start = format!("{}-{:02}-01", year, month);
    let next_month = if month == 12 { format!("{}-01-01", year + 1) } else { format!("{}-{:02}-01", year, month + 1) };

    // Revenue (topups)
    let revenue: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='topup' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Purchases
    let purchases: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(ABS(amount)),0)::bigint FROM transactions WHERE type='purchase' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Creator earnings
    let earnings: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='earning' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    let commission = purchases.0 - earnings.0;

    // Withdrawals completed
    let withdrawals: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount_credits),0)::bigint FROM withdrawals WHERE status='completed' AND completed_at >= $1::timestamptz AND completed_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Referral payouts
    let referrals: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='referral' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Admin credits
    let admin_credits: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM transactions WHERE type='admin_credit' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Credits in circulation (liability) at end of month
    let liabilities: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(credit_balance),0)::bigint FROM users")
        .fetch_one(&state.db).await?;

    // New registrations
    let registrations: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM users WHERE created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // New assets
    let new_assets: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets WHERE created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    // Sales count
    let sales: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE type='purchase' AND created_at >= $1::timestamptz AND created_at < $2::timestamptz")
        .bind(&month_start).bind(&next_month).fetch_one(&state.db).await?;

    Ok(Json(serde_json::json!({
        "year": year, "month": month,
        "revenue": revenue.0,
        "purchases": purchases.0,
        "creator_earnings": earnings.0,
        "commission": commission,
        "withdrawals": withdrawals.0,
        "referrals": referrals.0,
        "admin_credits": admin_credits.0,
        "credits_in_circulation": liabilities.0,
        "registrations": registrations.0,
        "new_assets": new_assets.0,
        "sales_count": sales.0,
    })))
}

#[derive(Deserialize)]
struct TimeseriesQuery {
    metric: String,
    months: Option<i32>,
}

async fn analytics_timeseries(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(params): Query<TimeseriesQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;
    let months = params.months.unwrap_or(12);

    let query = match params.metric.as_str() {
        "revenue" => "SELECT date_trunc('month', created_at) as month, COALESCE(SUM(amount),0)::bigint as value FROM transactions WHERE type='topup' AND created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "purchases" => "SELECT date_trunc('month', created_at) as month, COALESCE(SUM(ABS(amount)),0)::bigint as value FROM transactions WHERE type='purchase' AND created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "commission" => "SELECT date_trunc('month', t.created_at) as month, (COALESCE(SUM(ABS(CASE WHEN t.type='purchase' THEN t.amount ELSE 0 END)),0) - COALESCE(SUM(CASE WHEN t.type='earning' THEN t.amount ELSE 0 END),0))::bigint as value FROM transactions t WHERE t.type IN ('purchase','earning') AND t.created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "withdrawals" => "SELECT date_trunc('month', completed_at) as month, COALESCE(SUM(amount_credits),0)::bigint as value FROM withdrawals WHERE status='completed' AND completed_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "registrations" => "SELECT date_trunc('month', created_at) as month, COUNT(*)::bigint as value FROM users WHERE created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "assets_added" => "SELECT date_trunc('month', created_at) as month, COUNT(*)::bigint as value FROM assets WHERE created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        "downloads" => "SELECT date_trunc('month', created_at) as month, COUNT(*)::bigint as value FROM launcher_downloads WHERE created_at > NOW() - make_interval(months => $1) GROUP BY month ORDER BY month",
        _ => return Err(ApiError::Validation("Invalid metric".into())),
    };

    let rows = sqlx::query(query).bind(months).fetch_all(&state.db).await?;
    let data: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "month": r.get::<time::OffsetDateTime, _>("month").format(&Rfc3339).unwrap_or_default(),
        "value": r.get::<i64, _>("value"),
    })).collect();

    Ok(Json(serde_json::json!({"metric": params.metric, "data": data})))
}

async fn analytics_business(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, ApiError> {
    verify_admin(&state, auth.user_id).await?;

    // Top creators by earnings
    let top_creators = sqlx::query(
        "SELECT u.id, u.username, COALESCE(SUM(t.amount),0)::bigint as total_earnings FROM transactions t JOIN users u ON u.id = t.user_id WHERE t.type = 'earning' GROUP BY u.id, u.username ORDER BY total_earnings DESC LIMIT 10"
    ).fetch_all(&state.db).await?;

    let creators: Vec<serde_json::Value> = top_creators.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "username": r.get::<String, _>("username"),
        "total_earnings": r.get::<i64, _>("total_earnings"),
    })).collect();

    // Top buyers by spend
    let top_buyers = sqlx::query(
        "SELECT u.id, u.username, COALESCE(SUM(ABS(t.amount)),0)::bigint as total_spent FROM transactions t JOIN users u ON u.id = t.user_id WHERE t.type = 'purchase' GROUP BY u.id, u.username ORDER BY total_spent DESC LIMIT 10"
    ).fetch_all(&state.db).await?;

    let buyers: Vec<serde_json::Value> = top_buyers.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "username": r.get::<String, _>("username"),
        "total_spent": r.get::<i64, _>("total_spent"),
    })).collect();

    // Growth: current month vs last month
    let current_month_users: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM users WHERE created_at >= date_trunc('month', NOW())")
        .fetch_one(&state.db).await?;
    let last_month_users: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM users WHERE created_at >= date_trunc('month', NOW()) - INTERVAL '1 month' AND created_at < date_trunc('month', NOW())")
        .fetch_one(&state.db).await?;

    let current_month_assets: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets WHERE created_at >= date_trunc('month', NOW())")
        .fetch_one(&state.db).await?;
    let last_month_assets: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets WHERE created_at >= date_trunc('month', NOW()) - INTERVAL '1 month' AND created_at < date_trunc('month', NOW())")
        .fetch_one(&state.db).await?;

    // Launcher downloads by platform
    let platform_rows = sqlx::query("SELECT platform, COUNT(*)::bigint as count FROM launcher_downloads GROUP BY platform ORDER BY count DESC")
        .fetch_all(&state.db).await?;
    let platforms: Vec<serde_json::Value> = platform_rows.iter().map(|r| serde_json::json!({
        "platform": r.get::<String, _>("platform"),
        "count": r.get::<i64, _>("count"),
    })).collect();

    Ok(Json(serde_json::json!({
        "top_creators": creators,
        "top_buyers": buyers,
        "growth": {
            "users_this_month": current_month_users.0,
            "users_last_month": last_month_users.0,
            "assets_this_month": current_month_assets.0,
            "assets_last_month": last_month_assets.0,
        },
        "launcher_platforms": platforms,
    })))
}

