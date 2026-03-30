use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct PlayerStat {
    pub id: Uuid,
    pub app_id: Uuid,
    pub user_id: Uuid,
    pub stat_key: String,
    pub value_int: i64,
    pub value_float: f64,
    pub updated_at: OffsetDateTime,
}

impl PlayerStat {
    /// Set a stat value (upsert).
    pub async fn set(
        pool: &PgPool,
        app_id: Uuid,
        user_id: Uuid,
        key: &str,
        value_int: i64,
        value_float: f64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO player_stats (app_id, user_id, stat_key, value_int, value_float) VALUES ($1,$2,$3,$4,$5) \
             ON CONFLICT (app_id, user_id, stat_key) DO UPDATE SET value_int = $4, value_float = $5, updated_at = NOW() RETURNING *"
        )
        .bind(app_id).bind(user_id).bind(key).bind(value_int).bind(value_float)
        .fetch_one(pool)
        .await
    }

    /// Increment an integer stat.
    pub async fn increment(
        pool: &PgPool,
        app_id: Uuid,
        user_id: Uuid,
        key: &str,
        delta: i64,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO player_stats (app_id, user_id, stat_key, value_int) VALUES ($1,$2,$3,$4) \
             ON CONFLICT (app_id, user_id, stat_key) DO UPDATE SET value_int = player_stats.value_int + $4, updated_at = NOW() RETURNING *"
        )
        .bind(app_id).bind(user_id).bind(key).bind(delta)
        .fetch_one(pool)
        .await
    }

    /// Get a single stat.
    pub async fn get(pool: &PgPool, app_id: Uuid, user_id: Uuid, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM player_stats WHERE app_id = $1 AND user_id = $2 AND stat_key = $3")
            .bind(app_id).bind(user_id).bind(key).fetch_optional(pool).await
    }

    /// Get all stats for a player in an app.
    pub async fn list_for_player(pool: &PgPool, app_id: Uuid, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM player_stats WHERE app_id = $1 AND user_id = $2 ORDER BY stat_key")
            .bind(app_id).bind(user_id).fetch_all(pool).await
    }
}
