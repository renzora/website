use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

// ── License Types ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LicenseType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_multiplier: f64,
    pub max_users: i32,
    pub max_projects: i32,
    pub commercial_use: bool,
    pub sort_order: i32,
}

impl LicenseType {
    pub async fn list(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM license_types ORDER BY sort_order")
            .fetch_all(db).await
    }

    pub async fn find(db: &PgPool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM license_types WHERE id = $1")
            .bind(id).fetch_optional(db).await
    }
}

// ── License Grants ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LicenseGrant {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub user_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub license_type: String,
    pub source: String,
    pub credits_paid: i32,
    pub granted_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
}

impl LicenseGrant {
    pub async fn grant(
        db: &PgPool,
        asset_id: Uuid,
        user_id: Option<Uuid>,
        team_id: Option<Uuid>,
        license_type: &str,
        source: &str,
        credits_paid: i32,
        expires_at: Option<OffsetDateTime>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO license_grants (asset_id, user_id, team_id, license_type, source, credits_paid, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(asset_id).bind(user_id).bind(team_id)
        .bind(license_type).bind(source).bind(credits_paid).bind(expires_at)
        .fetch_one(db).await
    }

    pub async fn find_for_user(db: &PgPool, user_id: Uuid, asset_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM license_grants WHERE user_id = $1 AND asset_id = $2 AND (expires_at IS NULL OR expires_at > NOW()) ORDER BY granted_at DESC LIMIT 1"
        ).bind(user_id).bind(asset_id).fetch_optional(db).await
    }

    pub async fn find_for_team(db: &PgPool, team_id: Uuid, asset_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM license_grants WHERE team_id = $1 AND asset_id = $2 AND (expires_at IS NULL OR expires_at > NOW()) ORDER BY granted_at DESC LIMIT 1"
        ).bind(team_id).bind(asset_id).fetch_optional(db).await
    }

    /// Expire all library-sourced grants for a team (when subscription lapses).
    pub async fn expire_library_grants(db: &PgPool, team_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE license_grants SET expires_at = NOW() WHERE team_id = $1 AND source = 'library' AND (expires_at IS NULL OR expires_at > NOW())"
        ).bind(team_id).execute(db).await?;
        Ok(result.rows_affected())
    }
}

// ── Team Library ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamLibraryItem {
    pub id: Uuid,
    pub team_id: Uuid,
    pub asset_id: Uuid,
    pub added_by: Uuid,
    pub license_grant_id: Option<Uuid>,
    pub size_bytes: i64,
    pub added_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamLibraryItemWithAsset {
    pub id: Uuid,
    pub team_id: Uuid,
    pub asset_id: Uuid,
    pub added_by: Uuid,
    pub size_bytes: i64,
    pub added_at: OffsetDateTime,
    pub asset_name: String,
    pub asset_slug: String,
    pub asset_thumbnail_url: Option<String>,
    pub asset_category: String,
    pub added_by_username: String,
}

impl TeamLibraryItem {
    pub async fn add(
        db: &PgPool,
        team_id: Uuid,
        asset_id: Uuid,
        added_by: Uuid,
        license_grant_id: Uuid,
        size_bytes: i64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO team_library (team_id, asset_id, added_by, license_grant_id, size_bytes)
             VALUES ($1, $2, $3, $4, $5) RETURNING *"
        ).bind(team_id).bind(asset_id).bind(added_by).bind(license_grant_id).bind(size_bytes)
        .fetch_one(db).await
    }

    pub async fn list_for_team(db: &PgPool, team_id: Uuid) -> Result<Vec<TeamLibraryItemWithAsset>, sqlx::Error> {
        sqlx::query_as(
            "SELECT tl.id, tl.team_id, tl.asset_id, tl.added_by, tl.size_bytes, tl.added_at,
                    a.name AS asset_name, a.slug AS asset_slug, a.thumbnail_url AS asset_thumbnail_url,
                    a.category AS asset_category, u.username AS added_by_username
             FROM team_library tl
             JOIN assets a ON a.id = tl.asset_id
             JOIN users u ON u.id = tl.added_by
             WHERE tl.team_id = $1 ORDER BY tl.added_at DESC"
        ).bind(team_id).fetch_all(db).await
    }

    pub async fn exists(db: &PgPool, team_id: Uuid, asset_id: Uuid) -> Result<bool, sqlx::Error> {
        let r: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM team_library WHERE team_id = $1 AND asset_id = $2"
        ).bind(team_id).bind(asset_id).fetch_optional(db).await?;
        Ok(r.is_some())
    }

    pub async fn remove(db: &PgPool, team_id: Uuid, asset_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM team_library WHERE team_id = $1 AND asset_id = $2")
            .bind(team_id).bind(asset_id).execute(db).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn total_size(db: &PgPool, team_id: Uuid) -> Result<i64, sqlx::Error> {
        let r: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(size_bytes), 0)::bigint FROM team_library WHERE team_id = $1"
        ).bind(team_id).fetch_one(db).await?;
        Ok(r.0)
    }
}

// ── Library Requests ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LibraryRequest {
    pub id: Uuid,
    pub team_id: Uuid,
    pub asset_id: Uuid,
    pub requested_by: Uuid,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl LibraryRequest {
    pub async fn create(db: &PgPool, team_id: Uuid, asset_id: Uuid, requested_by: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO library_requests (team_id, asset_id, requested_by) VALUES ($1, $2, $3) RETURNING *"
        ).bind(team_id).bind(asset_id).bind(requested_by).fetch_one(db).await
    }

    pub async fn list_pending(db: &PgPool, team_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM library_requests WHERE team_id = $1 AND status = 'pending' ORDER BY created_at DESC")
            .bind(team_id).fetch_all(db).await
    }

    pub async fn approve(db: &PgPool, id: Uuid, reviewed_by: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "UPDATE library_requests SET status = 'approved', reviewed_by = $2, reviewed_at = NOW() WHERE id = $1 RETURNING *"
        ).bind(id).bind(reviewed_by).fetch_one(db).await
    }

    pub async fn deny(db: &PgPool, id: Uuid, reviewed_by: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "UPDATE library_requests SET status = 'denied', reviewed_by = $2, reviewed_at = NOW() WHERE id = $1 RETURNING *"
        ).bind(id).bind(reviewed_by).fetch_one(db).await
    }
}

// ── Team Role Permissions ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamRolePermission {
    pub role: String,
    pub can_browse_library: bool,
    pub can_add_to_library: bool,
    pub can_request_assets: bool,
    pub can_remove_from_library: bool,
    pub can_manage_budget: bool,
    pub can_invite_members: bool,
    pub can_manage_roles: bool,
}

impl TeamRolePermission {
    pub async fn find(db: &PgPool, role: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM team_role_permissions WHERE role = $1")
            .bind(role).fetch_optional(db).await
    }

    pub async fn list(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM team_role_permissions ORDER BY role")
            .fetch_all(db).await
    }
}

// ── Creator Pool ──

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreatorPool {
    pub id: Uuid,
    pub month: time::Date,
    pub total_credits: i64,
    pub total_library_adds: i64,
    pub credits_per_add: f64,
    pub distributed: bool,
    pub distributed_at: Option<OffsetDateTime>,
}

impl CreatorPool {
    /// Get or create pool for the current month.
    pub async fn current(db: &PgPool) -> Result<Self, sqlx::Error> {
        let existing: Option<Self> = sqlx::query_as(
            "SELECT * FROM creator_pool WHERE month = date_trunc('month', CURRENT_DATE)::date"
        ).fetch_optional(db).await?;

        if let Some(pool) = existing {
            return Ok(pool);
        }

        sqlx::query_as(
            "INSERT INTO creator_pool (month) VALUES (date_trunc('month', CURRENT_DATE)::date) RETURNING *"
        ).fetch_one(db).await
    }

    /// Add credits to the current month's pool (called when subscription renews).
    pub async fn contribute(db: &PgPool, user_id: Uuid, credits: i32) -> Result<(), sqlx::Error> {
        let month_start = "date_trunc('month', CURRENT_DATE)::date";

        sqlx::query(&format!(
            "INSERT INTO creator_pool (month, total_credits) VALUES ({month_start}, $1)
             ON CONFLICT (month) DO UPDATE SET total_credits = creator_pool.total_credits + $1"
        )).bind(credits as i64).execute(db).await?;

        sqlx::query(
            "INSERT INTO creator_pool_contributions (pool_month, user_id, credits_contributed)
             VALUES (date_trunc('month', CURRENT_DATE)::date, $1, $2)"
        ).bind(user_id).bind(credits).execute(db).await?;

        Ok(())
    }

    /// Max credits an asset can be weighted at in the pool (prevents price manipulation).
    pub const MAX_POOL_WEIGHT: i64 = 500;

    /// Record a library add and increment the pool counter (weighted by asset price, capped).
    /// Returns false if excluded from pool (self-add or creator not on paid plan).
    pub async fn record_library_add(db: &PgPool, asset_id: Uuid, creator_id: Uuid, team_id: Uuid, added_by: Uuid, asset_price: i64) -> Result<bool, sqlx::Error> {
        // Exclude if creator is a member of this team
        let is_team_member: Option<(Uuid,)> = sqlx::query_as(
            "SELECT user_id FROM team_members WHERE team_id = $1 AND user_id = $2"
        ).bind(team_id).bind(creator_id).fetch_optional(db).await?;

        if is_team_member.is_some() {
            return Ok(false);
        }

        // Exclude if creator not on a paid plan
        let has_paid_sub: Option<(String,)> = sqlx::query_as(
            "SELECT plan_id FROM subscriptions WHERE user_id = $1 AND status = 'active' AND current_period_end > NOW() AND plan_id != 'free'"
        ).bind(creator_id).fetch_optional(db).await?;

        if has_paid_sub.is_none() {
            return Ok(false);
        }

        // Weight is the asset price capped at MAX_POOL_WEIGHT
        let weight = asset_price.min(Self::MAX_POOL_WEIGHT).max(1);

        sqlx::query(
            "INSERT INTO library_add_log (asset_id, creator_id, team_id, added_by) VALUES ($1, $2, $3, $4)"
        ).bind(asset_id).bind(creator_id).bind(team_id).bind(added_by).execute(db).await?;

        // Increment by weight, not by 1
        sqlx::query(
            "UPDATE creator_pool SET total_library_adds = total_library_adds + $1
             WHERE month = date_trunc('month', CURRENT_DATE)::date"
        ).bind(weight).execute(db).await?;

        Ok(true)
    }
}

// ── Library Allowance ──

pub async fn get_monthly_allowance(db: &PgPool, user_id: Uuid, max_assets: i32) -> Result<(i32, i32), sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT assets_added FROM library_allowance WHERE user_id = $1 AND month = date_trunc('month', CURRENT_DATE)::date"
    ).bind(user_id).fetch_optional(db).await?;

    Ok((row.map(|r| r.0).unwrap_or(0), max_assets))
}

pub async fn increment_allowance(db: &PgPool, user_id: Uuid, max_assets: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO library_allowance (user_id, month, assets_added, max_assets)
         VALUES ($1, date_trunc('month', CURRENT_DATE)::date, 1, $2)
         ON CONFLICT (user_id, month) DO UPDATE SET assets_added = library_allowance.assets_added + 1"
    ).bind(user_id).bind(max_assets).execute(db).await?;
    Ok(())
}
