use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub is_staff: bool,
    pub permissions: serde_json::Value,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Ban {
    pub id: Uuid,
    pub user_id: Uuid,
    pub banned_by: Uuid,
    pub reason: String,
    pub r#type: String,
    pub expires_at: Option<OffsetDateTime>,
    pub active: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ModNote {
    pub id: Uuid,
    pub target_user_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ModNoteWithAuthor {
    pub id: Uuid,
    pub content: String,
    pub author_name: String,
    pub created_at: OffsetDateTime,
}

impl Role {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM roles ORDER BY sort_order").fetch_all(pool).await
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM roles WHERE name=$1").bind(name).fetch_optional(pool).await
    }

    pub async fn create(pool: &PgPool, name: &str, color: &str, is_staff: bool, permissions: serde_json::Value) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let sort: (i32,) = sqlx::query_as("SELECT COALESCE(MAX(sort_order),0)+1 FROM roles").fetch_one(pool).await?;
        sqlx::query_as("INSERT INTO roles (id,name,color,is_staff,permissions,sort_order) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *")
            .bind(id).bind(name).bind(color).bind(is_staff).bind(permissions).bind(sort.0)
            .fetch_one(pool).await
    }

    pub async fn update_permissions(pool: &PgPool, id: Uuid, permissions: serde_json::Value) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE roles SET permissions=$2 WHERE id=$1 RETURNING *")
            .bind(id).bind(permissions).fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_roles WHERE role_id=$1").bind(id).execute(pool).await?;
        sqlx::query("DELETE FROM roles WHERE id=$1").bind(id).execute(pool).await?;
        Ok(())
    }
}

/// Get all roles for a user.
pub async fn get_user_roles(pool: &PgPool, user_id: Uuid) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as("SELECT r.* FROM roles r JOIN user_roles ur ON ur.role_id=r.id WHERE ur.user_id=$1 ORDER BY r.sort_order")
        .bind(user_id).fetch_all(pool).await
}

/// Check if a user has a specific permission.
pub async fn has_permission(pool: &PgPool, user_id: Uuid, permission: &str) -> Result<bool, sqlx::Error> {
    let roles = get_user_roles(pool, user_id).await?;
    // Also check the legacy role column
    let user_role: Option<(String,)> = sqlx::query_as("SELECT role FROM users WHERE id=$1").bind(user_id).fetch_optional(pool).await?;
    if let Some((role,)) = user_role {
        if role == "admin" { return Ok(true); }
    }
    for role in &roles {
        if let Some(perms) = role.permissions.as_object() {
            if perms.get(permission).and_then(|v| v.as_bool()) == Some(true) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Assign a role to a user.
pub async fn assign_role(pool: &PgPool, user_id: Uuid, role_id: Uuid, granted_by: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user_roles (user_id,role_id,granted_by) VALUES ($1,$2,$3) ON CONFLICT DO NOTHING")
        .bind(user_id).bind(role_id).bind(granted_by).execute(pool).await?;
    Ok(())
}

/// Remove a role from a user.
pub async fn remove_role(pool: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM user_roles WHERE user_id=$1 AND role_id=$2")
        .bind(user_id).bind(role_id).execute(pool).await?;
    Ok(())
}

// ── Bans ──

/// Check if a user is currently banned.
pub async fn is_banned(pool: &PgPool, user_id: Uuid) -> Result<Option<Ban>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM bans WHERE user_id=$1 AND active=true AND (expires_at IS NULL OR expires_at > NOW()) ORDER BY created_at DESC LIMIT 1")
        .bind(user_id).fetch_optional(pool).await
}

/// Ban a user.
pub async fn ban_user(pool: &PgPool, user_id: Uuid, banned_by: Uuid, reason: &str, ban_type: &str, duration_hours: Option<i64>) -> Result<Ban, sqlx::Error> {
    let id = Uuid::new_v4();
    let expires = duration_hours.map(|h| OffsetDateTime::now_utc() + time::Duration::hours(h));
    sqlx::query_as("INSERT INTO bans (id,user_id,banned_by,reason,type,expires_at) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *")
        .bind(id).bind(user_id).bind(banned_by).bind(reason).bind(ban_type).bind(expires)
        .fetch_one(pool).await
}

/// Unban a user.
pub async fn unban_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE bans SET active=false WHERE user_id=$1 AND active=true")
        .bind(user_id).execute(pool).await?;
    Ok(())
}

/// List bans for a user.
pub async fn list_bans(pool: &PgPool, user_id: Uuid) -> Result<Vec<Ban>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM bans WHERE user_id=$1 ORDER BY created_at DESC")
        .bind(user_id).fetch_all(pool).await
}

// ── Mod Notes ──

pub async fn add_mod_note(pool: &PgPool, target_user_id: Uuid, author_id: Uuid, content: &str) -> Result<ModNote, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query_as("INSERT INTO mod_notes (id,target_user_id,author_id,content) VALUES ($1,$2,$3,$4) RETURNING *")
        .bind(id).bind(target_user_id).bind(author_id).bind(content).fetch_one(pool).await
}

pub async fn get_mod_notes(pool: &PgPool, target_user_id: Uuid) -> Result<Vec<ModNoteWithAuthor>, sqlx::Error> {
    sqlx::query_as("SELECT mn.id, mn.content, u.username as author_name, mn.created_at FROM mod_notes mn JOIN users u ON u.id=mn.author_id WHERE mn.target_user_id=$1 ORDER BY mn.created_at DESC")
        .bind(target_user_id).fetch_all(pool).await
}

pub async fn delete_mod_note(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM mod_notes WHERE id=$1").bind(id).execute(pool).await?;
    Ok(())
}
