use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Asset {
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

/// Asset joined with creator username for listing.
#[derive(Debug, sqlx::FromRow)]
pub struct AssetWithCreator {
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

impl Asset {
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

        sqlx::query_as::<_, Asset>(
            r#"
            INSERT INTO assets (id, creator_id, name, slug, description, category, price_credits, version, created_at, updated_at)
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
        sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_published(
        pool: &PgPool,
        query: Option<&str>,
        category: Option<&str>,
        sort: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<AssetWithCreator>, i64), sqlx::Error> {
        Self::list_published_filtered(pool, query, category, sort, page, per_page, None, None, None).await
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
    ) -> Result<(Vec<AssetWithCreator>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let order_clause = match sort {
            "newest" => "a.created_at DESC",
            "popular" => "a.downloads DESC",
            "price_asc" => "a.price_credits ASC",
            "price_desc" => "a.price_credits DESC",
            "top_rated" => "CASE WHEN a.rating_count > 0 THEN a.rating_sum::float / a.rating_count ELSE 0 END DESC",
            _ => "a.created_at DESC",
        };

        let free_filter = free_only.unwrap_or(false);
        let min_r = min_rating.unwrap_or(0);
        let max_p = max_price.unwrap_or(-1);

        let assets = sqlx::query_as::<_, AssetWithCreator>(&format!(
            r#"
            SELECT a.id, a.name, a.slug, a.description, a.category, a.price_credits,
                   a.thumbnail_url, a.version, a.downloads, u.username AS creator_name,
                   a.rating_sum, a.rating_count
            FROM assets a
            JOIN users u ON u.id = a.creator_id
            WHERE a.published = true
              AND ($1::text IS NULL OR a.name ILIKE '%' || $1 || '%' OR a.description ILIKE '%' || $1 || '%')
              AND ($2::text IS NULL OR $2 = 'all' OR a.category = $2)
              AND ($5::bool = false OR a.price_credits = 0)
              AND ($6::int = 0 OR (a.rating_count > 0 AND a.rating_sum::float / a.rating_count >= $6::float))
              AND ($7::bigint = -1 OR a.price_credits <= $7)
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
            FROM assets a
            WHERE a.published = true
              AND ($1::text IS NULL OR a.name ILIKE '%' || $1 || '%' OR a.description ILIKE '%' || $1 || '%')
              AND ($2::text IS NULL OR $2 = 'all' OR a.category = $2)
              AND ($3::bool = false OR a.price_credits = 0)
              AND ($4::int = 0 OR (a.rating_count > 0 AND a.rating_sum::float / a.rating_count >= $4::float))
              AND ($5::bigint = -1 OR a.price_credits <= $5)
            "#,
        )
        .bind(query)
        .bind(category)
        .bind(free_filter)
        .bind(min_r)
        .bind(max_p)
        .fetch_one(pool)
        .await?;

        Ok((assets, total.0))
    }

    pub async fn list_by_creator(
        pool: &PgPool,
        creator_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            "SELECT * FROM assets WHERE creator_id = $1 ORDER BY created_at DESC",
        )
        .bind(creator_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_file_url(
        pool: &PgPool,
        id: Uuid,
        file_url: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE assets SET file_url = $1, updated_at = NOW() WHERE id = $2")
            .bind(file_url)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_thumbnail_url(
        pool: &PgPool,
        id: Uuid,
        thumbnail_url: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE assets SET thumbnail_url = $1, updated_at = NOW() WHERE id = $2")
            .bind(thumbnail_url)
            .bind(id)
            .execute(pool)
            .await?;
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
        sqlx::query_as::<_, Asset>(
            r#"
            UPDATE assets SET
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
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(price_credits)
        .bind(version)
        .bind(published)
        .fetch_one(pool)
        .await
    }

    /// List all assets owned/purchased by a user (via user_assets table).
    pub async fn list_purchased_by_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<(Vec<AssetWithCreator>, i64), sqlx::Error> {
        let assets = sqlx::query_as::<_, AssetWithCreator>(
            r#"
            SELECT a.id, a.name, a.slug, a.description, a.category, a.price_credits,
                   a.thumbnail_url, a.version, a.downloads, u.username AS creator_name,
                   a.rating_sum, a.rating_count
            FROM user_assets ua
            JOIN assets a ON a.id = ua.asset_id
            JOIN users u ON u.id = a.creator_id
            WHERE ua.user_id = $1
            ORDER BY ua.purchased_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let total = assets.len() as i64;
        Ok((assets, total))
    }

    pub async fn increment_downloads(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE assets SET downloads = downloads + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// Check if a user owns a specific asset.
pub async fn user_owns_asset(
    pool: &PgPool,
    user_id: Uuid,
    asset_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result: Option<(Uuid,)> = sqlx::query_as(
        "SELECT user_id FROM user_assets WHERE user_id = $1 AND asset_id = $2",
    )
    .bind(user_id)
    .bind(asset_id)
    .fetch_optional(pool)
    .await?;
    Ok(result.is_some())
}

/// Grant ownership of an asset to a user.
pub async fn grant_asset_ownership(
    pool: &PgPool,
    user_id: Uuid,
    asset_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_assets (user_id, asset_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(asset_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub fn slugify_text(name: &str, id: Uuid) -> String {
    slugify(name, id)
}

fn slugify(name: &str, id: Uuid) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    // Trim leading/trailing dashes and collapse multiple dashes
    let trimmed: String = base
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    // Append short UUID suffix for uniqueness
    let short_id = &id.to_string()[..8];
    format!("{trimmed}-{short_id}")
}
