use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct AvatarPart {
    pub id: Uuid,
    pub slot: String,
    pub name: String,
    pub slug: String,
    pub part_data: serde_json::Value,
    pub price_credits: i64,
    pub is_default: bool,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

impl AvatarPart {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM avatar_parts ORDER BY slot, sort_order")
            .fetch_all(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM avatar_parts WHERE id = $1")
            .bind(id).fetch_optional(pool).await
    }

    pub async fn find_defaults(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM avatar_parts WHERE is_default = true ORDER BY slot, sort_order")
            .fetch_all(pool).await
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserAvatarPart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub part_id: Uuid,
    pub purchased_at: OffsetDateTime,
}

impl UserAvatarPart {
    pub async fn owns_part(pool: &PgPool, user_id: Uuid, part_id: Uuid) -> Result<bool, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM user_avatar_parts WHERE user_id = $1 AND part_id = $2")
            .bind(user_id).bind(part_id).fetch_one(pool).await?;
        Ok(row.0 > 0)
    }

    pub async fn grant(pool: &PgPool, user_id: Uuid, part_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO user_avatar_parts (user_id, part_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(user_id).bind(part_id).execute(pool).await?;
        Ok(())
    }

    pub async fn list_owned_ids(pool: &PgPool, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let rows: Vec<(Uuid,)> = sqlx::query_as("SELECT part_id FROM user_avatar_parts WHERE user_id = $1")
            .bind(user_id).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserAvatar {
    pub user_id: Uuid,
    pub skin_color: String,
    pub eye_color: String,
    pub hair_color: String,
    pub equipped_parts: serde_json::Value,
    pub updated_at: OffsetDateTime,
}

impl UserAvatar {
    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM user_avatars WHERE user_id = $1")
            .bind(user_id).fetch_optional(pool).await
    }

    pub async fn upsert(
        pool: &PgPool,
        user_id: Uuid,
        skin_color: &str,
        eye_color: &str,
        hair_color: &str,
        equipped_parts: serde_json::Value,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"INSERT INTO user_avatars (user_id, skin_color, eye_color, hair_color, equipped_parts, updated_at)
               VALUES ($1, $2, $3, $4, $5, NOW())
               ON CONFLICT (user_id) DO UPDATE SET skin_color = $2, eye_color = $3, hair_color = $4, equipped_parts = $5, updated_at = NOW()
               RETURNING *"#,
        )
        .bind(user_id).bind(skin_color).bind(eye_color).bind(hair_color).bind(equipped_parts)
        .fetch_one(pool).await
    }
}
