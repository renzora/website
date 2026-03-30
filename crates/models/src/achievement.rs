use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// Achievement definition, created by the app developer.
#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct AppAchievement {
    pub id: Uuid,
    pub app_id: Uuid,
    pub achievement_key: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub points: i32,
    pub hidden: bool,
    pub created_at: OffsetDateTime,
}

/// A player's unlocked achievement.
#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct PlayerAchievement {
    pub id: Uuid,
    pub app_id: Uuid,
    pub user_id: Uuid,
    pub achievement_id: Uuid,
    pub unlocked_at: OffsetDateTime,
    pub metadata: serde_json::Value,
}

impl AppAchievement {
    pub async fn create(
        pool: &PgPool,
        app_id: Uuid,
        key: &str,
        name: &str,
        description: &str,
        icon_url: Option<&str>,
        points: i32,
        hidden: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO app_achievements (app_id, achievement_key, name, description, icon_url, points, hidden) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *"
        )
        .bind(app_id).bind(key).bind(name).bind(description).bind(icon_url).bind(points).bind(hidden)
        .fetch_one(pool)
        .await
    }

    pub async fn list_by_app(pool: &PgPool, app_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_achievements WHERE app_id = $1 ORDER BY created_at")
            .bind(app_id).fetch_all(pool).await
    }

    pub async fn find_by_key(pool: &PgPool, app_id: Uuid, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM app_achievements WHERE app_id = $1 AND achievement_key = $2")
            .bind(app_id).bind(key).fetch_optional(pool).await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        app_id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        icon_url: Option<&str>,
        points: Option<i32>,
        hidden: Option<bool>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "UPDATE app_achievements SET name=COALESCE($3,name), description=COALESCE($4,description), icon_url=COALESCE($5,icon_url), points=COALESCE($6,points), hidden=COALESCE($7,hidden) WHERE id=$1 AND app_id=$2 RETURNING *"
        )
        .bind(id).bind(app_id).bind(name).bind(description).bind(icon_url).bind(points).bind(hidden)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, app_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM app_achievements WHERE id = $1 AND app_id = $2")
            .bind(id).bind(app_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}

impl PlayerAchievement {
    /// Unlock an achievement for a player. Returns None if already unlocked.
    pub async fn unlock(
        pool: &PgPool,
        app_id: Uuid,
        user_id: Uuid,
        achievement_id: Uuid,
        metadata: serde_json::Value,
    ) -> Result<Option<Self>, sqlx::Error> {
        // Use ON CONFLICT to avoid duplicates
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO player_achievements (app_id, user_id, achievement_id, metadata) VALUES ($1,$2,$3,$4) ON CONFLICT (user_id, achievement_id) DO NOTHING RETURNING *"
        )
        .bind(app_id).bind(user_id).bind(achievement_id).bind(metadata)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    /// List all achievements unlocked by a player for an app.
    pub async fn list_for_player(pool: &PgPool, app_id: Uuid, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM player_achievements WHERE app_id = $1 AND user_id = $2 ORDER BY unlocked_at")
            .bind(app_id).bind(user_id).fetch_all(pool).await
    }
}
