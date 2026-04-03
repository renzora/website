use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// XP required to reach a given level: level * (level - 1) * 50
pub fn xp_for_level(level: i32) -> i64 {
    (level as i64) * ((level - 1) as i64) * 50
}

/// Compute level from total XP
pub fn level_from_xp(xp: i64) -> i32 {
    let mut level = 1;
    while xp_for_level(level + 1) <= xp {
        level += 1;
    }
    level
}

/// XP amounts for various actions
pub const XP_UPLOAD_ASSET: i64 = 25;
pub const XP_FIRST_SALE: i64 = 50;
pub const XP_SALE: i64 = 10;
pub const XP_PURCHASE: i64 = 5;
pub const XP_REVIEW: i64 = 10;
pub const XP_FORUM_POST: i64 = 5;
pub const XP_ARTICLE: i64 = 20;
pub const XP_DAILY_LOGIN: i64 = 5;
pub const XP_REFERRAL: i64 = 30;

/// Seller XP for actions
pub const SELLER_XP_SALE: i64 = 15;
pub const SELLER_XP_REVIEW: i64 = 5;
pub const SELLER_XP_DOWNLOAD: i64 = 1;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct XpEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub reason: String,
    pub source_id: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

impl XpEvent {
    pub async fn list_for_user(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM xp_events WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2")
            .bind(user_id).bind(limit).fetch_all(pool).await
    }
}

/// Award XP to a user. Updates total_xp and recomputes level. Returns new level.
pub async fn award_xp(pool: &PgPool, user_id: Uuid, amount: i64, reason: &str, source_id: Option<Uuid>) -> Result<i32, sqlx::Error> {
    // Insert xp event
    sqlx::query("INSERT INTO xp_events (user_id, amount, reason, source_id) VALUES ($1, $2, $3, $4)")
        .bind(user_id).bind(amount).bind(reason).bind(source_id)
        .execute(pool).await?;

    // Update total_xp
    sqlx::query("UPDATE users SET total_xp = total_xp + $1, updated_at = NOW() WHERE id = $2")
        .bind(amount).bind(user_id)
        .execute(pool).await?;

    // Recompute level
    let row: (i64,) = sqlx::query_as("SELECT total_xp FROM users WHERE id = $1")
        .bind(user_id).fetch_one(pool).await?;
    let new_level = level_from_xp(row.0);

    sqlx::query("UPDATE users SET level = $1 WHERE id = $2")
        .bind(new_level).bind(user_id)
        .execute(pool).await?;

    // Auto-award badges
    let _ = check_and_award_badges(pool, user_id).await;

    Ok(new_level)
}

/// Award seller XP. Updates seller_xp and recomputes seller_level.
pub async fn award_seller_xp(pool: &PgPool, user_id: Uuid, amount: i64, reason: &str, source_id: Option<Uuid>) -> Result<i32, sqlx::Error> {
    // Also log as xp event with "seller_" prefix
    sqlx::query("INSERT INTO xp_events (user_id, amount, reason, source_id) VALUES ($1, $2, $3, $4)")
        .bind(user_id).bind(amount).bind(format!("seller_{}", reason)).bind(source_id)
        .execute(pool).await?;

    sqlx::query("UPDATE users SET seller_xp = seller_xp + $1, updated_at = NOW() WHERE id = $2")
        .bind(amount).bind(user_id)
        .execute(pool).await?;

    // Compute seller level from seller_levels table
    let row: (i64,) = sqlx::query_as("SELECT seller_xp FROM users WHERE id = $1")
        .bind(user_id).fetch_one(pool).await?;

    let new_level: (i32,) = sqlx::query_as(
        "SELECT COALESCE(MAX(level), 0)::int FROM seller_levels WHERE min_seller_xp <= $1"
    ).bind(row.0).fetch_one(pool).await?;

    sqlx::query("UPDATE users SET seller_level = $1 WHERE id = $2")
        .bind(new_level.0).bind(user_id)
        .execute(pool).await?;

    // Auto-award badges
    let _ = check_and_award_badges(pool, user_id).await;

    Ok(new_level.0)
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SellerLevel {
    pub level: i32,
    pub name: String,
    pub min_seller_xp: i64,
    pub search_boost: f32,
    pub badge_color: String,
    pub perks: String,
}

impl SellerLevel {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM seller_levels ORDER BY level").fetch_all(pool).await
    }

    pub async fn find_by_level(pool: &PgPool, level: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM seller_levels WHERE level = $1")
            .bind(level).fetch_optional(pool).await
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SellerTask {
    pub id: Uuid,
    pub seller_level: i32,
    pub description: String,
    pub task_type: String,
    pub target_value: i64,
    pub xp_reward: i64,
    pub sort_order: i32,
}

impl SellerTask {
    pub async fn list_for_level(pool: &PgPool, level: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM seller_tasks WHERE seller_level = $1 ORDER BY sort_order")
            .bind(level).fetch_all(pool).await
    }
}

/// Auto-award badges based on auto_rule / auto_threshold.
/// Call this after any XP change or significant action.
pub async fn check_and_award_badges(pool: &PgPool, user_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
    // Get user stats
    let user = sqlx::query_as::<_, (i64, i32, i32, i32, i32, i64,)>(
        "SELECT total_xp, level, seller_level, follower_count, post_count, credit_balance FROM users WHERE id = $1"
    ).bind(user_id).fetch_one(pool).await?;

    let (total_xp, level, seller_level, follower_count, post_count, _balance) = user;

    // Get purchase count
    let (purchase_count,): (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE user_id = $1 AND type = 'purchase'")
        .bind(user_id).fetch_one(pool).await?;

    // Get upload count
    let (upload_count,): (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM assets WHERE creator_id = $1")
        .bind(user_id).fetch_one(pool).await?;

    // Get sale count
    let (sale_count,): (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM transactions WHERE user_id = $1 AND type = 'earning'")
        .bind(user_id).fetch_one(pool).await?;

    // Get total downloads
    let (total_downloads,): (i64,) = sqlx::query_as("SELECT COALESCE(SUM(downloads),0)::bigint FROM assets WHERE creator_id = $1")
        .bind(user_id).fetch_one(pool).await?;

    // Get article count
    let (article_count,): (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM articles WHERE author_id = $1")
        .bind(user_id).fetch_one(pool).await?;

    // Get all auto-rule badges
    let badges: Vec<(Uuid, String, String, i64)> = sqlx::query_as(
        "SELECT id, slug, auto_rule, auto_threshold FROM badges WHERE auto_rule IS NOT NULL AND auto_rule != ''"
    ).fetch_all(pool).await?;

    let mut awarded = Vec::new();

    for (badge_id, slug, rule, threshold) in &badges {
        let qualifies = match rule.as_str() {
            "level" => level as i64 >= *threshold,
            "total_xp" => total_xp >= *threshold,
            "seller_level" => seller_level as i64 >= *threshold,
            "first_purchase" | "purchase_count" => purchase_count >= *threshold,
            "first_upload" => upload_count >= *threshold,
            "sale_count" => sale_count >= *threshold,
            "total_downloads" => total_downloads >= *threshold,
            "follower_count" => follower_count as i64 >= *threshold,
            "post_count" => post_count as i64 >= *threshold,
            "article_count" => article_count >= *threshold,
            _ => false,
        };

        if qualifies {
            // Try to award (ON CONFLICT DO NOTHING if already has it)
            let result = sqlx::query("INSERT INTO user_badges (user_id, badge_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(user_id).bind(badge_id)
                .execute(pool).await?;
            if result.rows_affected() > 0 {
                awarded.push(slug.clone());
            }
        }
    }

    Ok(awarded)
}
