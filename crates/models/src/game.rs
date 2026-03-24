use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub published: bool,
    pub rating_sum: i64,
    pub rating_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameWithCreator {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub creator_name: String,
    pub rating_sum: i64,
    pub rating_count: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameCategory {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub sort_order: i32,
    pub max_file_size_mb: i32,
    pub allowed_extensions: Vec<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameMedia {
    pub id: Uuid,
    pub game_id: Uuid,
    pub media_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub sort_order: i32,
    pub created_at: OffsetDateTime,
}

impl Game {
    pub async fn create(
        pool: &PgPool,
        creator_id: Uuid,
        name: &str,
        description: &str,
        category: &str,
        price_credits: i64,
        version: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let slug = slugify(name, id);
        let now = OffsetDateTime::now_utc();

        sqlx::query_as::<_, Game>(
            r#"
            INSERT INTO games (id, creator_id, name, slug, description, category, price_credits, version, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(creator_id)
        .bind(name)
        .bind(&slug)
        .bind(description)
        .bind(category)
        .bind(price_credits)
        .bind(version)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Game>("SELECT * FROM games WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_published_filtered(
        pool: &PgPool,
        query: Option<&str>,
        category: Option<&str>,
        sort: &str,
        page: i64,
        per_page: i64,
        free_only: Option<bool>,
        min_rating: Option<i32>,
        max_price: Option<i64>,
    ) -> Result<(Vec<GameWithCreator>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let order_clause = match sort {
            "newest" => "g.created_at DESC",
            "popular" => "g.downloads DESC",
            "price_asc" => "g.price_credits ASC",
            "price_desc" => "g.price_credits DESC",
            "top_rated" => "CASE WHEN g.rating_count > 0 THEN g.rating_sum::float / g.rating_count ELSE 0 END DESC",
            _ => "g.created_at DESC",
        };

        let free_filter = free_only.unwrap_or(false);
        let min_r = min_rating.unwrap_or(0);
        let max_p = max_price.unwrap_or(-1);

        let games = sqlx::query_as::<_, GameWithCreator>(&format!(
            r#"
            SELECT g.id, g.name, g.slug, g.description, g.category, g.price_credits,
                   g.thumbnail_url, g.version, g.downloads, u.username AS creator_name,
                   g.rating_sum, g.rating_count
            FROM games g
            JOIN users u ON u.id = g.creator_id
            WHERE g.published = true
              AND ($1::text IS NULL OR g.name ILIKE '%' || $1 || '%' OR g.description ILIKE '%' || $1 || '%')
              AND ($2::text IS NULL OR $2 = 'all' OR g.category = $2)
              AND ($5::bool = false OR g.price_credits = 0)
              AND ($6::int = 0 OR (g.rating_count > 0 AND g.rating_sum::float / g.rating_count >= $6::float))
              AND ($7::bigint = -1 OR g.price_credits <= $7)
            ORDER BY {order_clause}
            LIMIT $3 OFFSET $4
            "#,
        ))
        .bind(query)
        .bind(category)
        .bind(per_page)
        .bind(offset)
        .bind(free_filter)
        .bind(min_r)
        .bind(max_p)
        .fetch_all(pool)
        .await?;

        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)::bigint
            FROM games g
            WHERE g.published = true
              AND ($1::text IS NULL OR g.name ILIKE '%' || $1 || '%' OR g.description ILIKE '%' || $1 || '%')
              AND ($2::text IS NULL OR $2 = 'all' OR g.category = $2)
              AND ($3::bool = false OR g.price_credits = 0)
              AND ($4::int = 0 OR (g.rating_count > 0 AND g.rating_sum::float / g.rating_count >= $4::float))
              AND ($5::bigint = -1 OR g.price_credits <= $5)
            "#,
        )
        .bind(query)
        .bind(category)
        .bind(free_filter)
        .bind(min_r)
        .bind(max_p)
        .fetch_one(pool)
        .await?;

        Ok((games, total.0))
    }

    pub async fn list_by_creator(pool: &PgPool, creator_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Game>(
            "SELECT * FROM games WHERE creator_id = $1 ORDER BY created_at DESC",
        )
        .bind(creator_id)
        .fetch_all(pool)
        .await
    }

    pub async fn list_purchased_by_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<(Vec<GameWithCreator>, i64), sqlx::Error> {
        let games = sqlx::query_as::<_, GameWithCreator>(
            r#"
            SELECT g.id, g.name, g.slug, g.description, g.category, g.price_credits,
                   g.thumbnail_url, g.version, g.downloads, u.username AS creator_name,
                   g.rating_sum, g.rating_count
            FROM user_games ug
            JOIN games g ON g.id = ug.game_id
            JOIN users u ON u.id = g.creator_id
            WHERE ug.user_id = $1
            ORDER BY ug.purchased_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let total = games.len() as i64;
        Ok((games, total))
    }

    pub async fn update_file_url(pool: &PgPool, id: Uuid, file_url: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE games SET file_url = $1, updated_at = NOW() WHERE id = $2")
            .bind(file_url).bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn update_thumbnail_url(pool: &PgPool, id: Uuid, thumbnail_url: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE games SET thumbnail_url = $1, updated_at = NOW() WHERE id = $2")
            .bind(thumbnail_url).bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn update_metadata(
        pool: &PgPool,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        price_credits: Option<i64>,
        version: Option<&str>,
        published: Option<bool>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Game>(
            r#"
            UPDATE games SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                price_credits = COALESCE($4, price_credits),
                version = COALESCE($5, version),
                published = COALESCE($6, published),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id).bind(name).bind(description).bind(price_credits).bind(version).bind(published)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_downloads(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE games SET downloads = downloads + 1 WHERE id = $1")
            .bind(id).execute(pool).await?;
        Ok(())
    }
}

impl GameCategory {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, GameCategory>(
            "SELECT * FROM game_categories ORDER BY sort_order ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, GameCategory>(
            "SELECT * FROM game_categories WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(pool)
        .await
    }
}

impl GameMedia {
    pub async fn create(
        pool: &PgPool,
        game_id: Uuid,
        media_type: &str,
        url: &str,
        thumbnail_url: Option<&str>,
        sort_order: i32,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, GameMedia>(
            r#"
            INSERT INTO game_media (id, game_id, media_type, url, thumbnail_url, sort_order)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4()).bind(game_id).bind(media_type).bind(url).bind(thumbnail_url).bind(sort_order)
        .fetch_one(pool)
        .await
    }

    pub async fn list_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, GameMedia>(
            "SELECT * FROM game_media WHERE game_id = $1 ORDER BY sort_order ASC",
        )
        .bind(game_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM game_media WHERE id = $1")
            .bind(id).execute(pool).await?;
        Ok(())
    }
}

pub async fn user_owns_game(pool: &PgPool, user_id: Uuid, game_id: Uuid) -> Result<bool, sqlx::Error> {
    let result: Option<(Uuid,)> = sqlx::query_as(
        "SELECT user_id FROM user_games WHERE user_id = $1 AND game_id = $2",
    )
    .bind(user_id).bind(game_id)
    .fetch_optional(pool)
    .await?;
    Ok(result.is_some())
}

pub async fn grant_game_ownership(pool: &PgPool, user_id: Uuid, game_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_games (user_id, game_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id).bind(game_id)
    .execute(pool)
    .await?;
    Ok(())
}

fn slugify(name: &str, id: Uuid) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed: String = base
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let short_id = &id.to_string()[..8];
    format!("{trimmed}-{short_id}")
}
