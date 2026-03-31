use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct DeveloperApp {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub website_url: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret_hash: String,
    pub icon_url: Option<String>,
    pub approved: bool,
    pub suspended: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct AppToken {
    pub id: Uuid,
    pub app_id: Uuid,
    pub name: String,
    pub token_hash: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub last_used_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub suspended: bool,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct AppUserGrant {
    pub id: Uuid,
    pub app_id: Uuid,
    pub user_id: Uuid,
    pub scopes_granted: Vec<String>,
    pub granted_at: OffsetDateTime,
}

/// Valid scopes that can be requested by developer apps.
pub const VALID_SCOPES: &[&str] = &[
    "profile:read",        // Read username, avatar
    "friends:read",        // Read friend list
    "friends:write",       // Send/accept friend requests
    "achievements:read",   // Read player achievements
    "achievements:write",  // Unlock achievements
    "stats:read",          // Read player stats
    "stats:write",         // Update player stats
    "leaderboards:read",   // Read leaderboard scores
    "leaderboards:write",  // Submit leaderboard scores
    "inventory:read",      // Read purchased assets/games
];

impl DeveloperApp {
    pub async fn create(
        pool: &PgPool,
        owner_id: Uuid,
        name: &str,
        description: &str,
        website_url: &str,
        redirect_uri: &str,
        client_id: &str,
        client_secret_hash: &str,
    ) -> Result<Self, sqlx::Error> {
        let slug = slugify_app(name);
        sqlx::query_as::<_, Self>(
            "INSERT INTO developer_apps (owner_id, name, slug, description, website_url, redirect_uri, client_id, client_secret_hash) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"
        )
        .bind(owner_id).bind(name).bind(&slug).bind(description)
        .bind(website_url).bind(redirect_uri).bind(client_id).bind(client_secret_hash)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM developer_apps WHERE id = $1")
            .bind(id).fetch_optional(pool).await
    }

    pub async fn find_by_client_id(pool: &PgPool, client_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM developer_apps WHERE client_id = $1")
            .bind(client_id).fetch_optional(pool).await
    }

    pub async fn list_by_owner(pool: &PgPool, owner_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM developer_apps WHERE owner_id = $1 ORDER BY created_at DESC")
            .bind(owner_id).fetch_all(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM developer_apps WHERE id = $1 AND owner_id = $2")
            .bind(id).bind(owner_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn set_suspended(pool: &PgPool, id: Uuid, suspended: bool) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE developer_apps SET suspended = $1 WHERE id = $2")
            .bind(suspended).bind(id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn set_approved(pool: &PgPool, id: Uuid, approved: bool) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE developer_apps SET approved = $1 WHERE id = $2")
            .bind(approved).bind(id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}

impl AppToken {
    pub async fn create(
        pool: &PgPool,
        app_id: Uuid,
        name: &str,
        token_hash: &str,
        prefix: &str,
        scopes: &[String],
        expires_at: Option<OffsetDateTime>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO app_tokens (app_id, name, token_hash, prefix, scopes, expires_at) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"
        )
        .bind(app_id).bind(name).bind(token_hash).bind(prefix).bind(scopes).bind(expires_at)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_hash(pool: &PgPool, token_hash: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_tokens WHERE token_hash = $1")
            .bind(token_hash).fetch_optional(pool).await
    }

    pub async fn list_by_app(pool: &PgPool, app_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_tokens WHERE app_id = $1 ORDER BY created_at DESC")
            .bind(app_id).fetch_all(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, app_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM app_tokens WHERE id = $1 AND app_id = $2")
            .bind(id).bind(app_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn touch_last_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE app_tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn set_suspended(pool: &PgPool, id: Uuid, suspended: bool) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE app_tokens SET suspended = $1 WHERE id = $2")
            .bind(suspended).bind(id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}

impl AppUserGrant {
    pub async fn grant(
        pool: &PgPool,
        app_id: Uuid,
        user_id: Uuid,
        scopes: &[String],
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO app_user_grants (app_id, user_id, scopes_granted) VALUES ($1,$2,$3) ON CONFLICT (app_id, user_id) DO UPDATE SET scopes_granted = $3, granted_at = NOW() RETURNING *"
        )
        .bind(app_id).bind(user_id).bind(scopes)
        .fetch_one(pool)
        .await
    }

    pub async fn find(pool: &PgPool, app_id: Uuid, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_user_grants WHERE app_id = $1 AND user_id = $2")
            .bind(app_id).bind(user_id).fetch_optional(pool).await
    }

    pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_user_grants WHERE user_id = $1 ORDER BY granted_at DESC")
            .bind(user_id).fetch_all(pool).await
    }

    pub async fn revoke(pool: &PgPool, app_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM app_user_grants WHERE app_id = $1 AND user_id = $2")
            .bind(app_id).bind(user_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }

    /// Check if user has granted a specific scope to an app.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes_granted.iter().any(|s| s == scope)
    }
}

fn slugify_app(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
