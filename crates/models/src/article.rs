use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
    pub cover_image_url: Option<String>,
    pub published: bool,
    pub likes: i32,
    pub views: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ArticleWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub cover_image_url: Option<String>,
    pub likes: i32,
    pub views: i32,
    pub author_name: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ArticleComment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommentWithAuthor {
    pub id: Uuid,
    pub content: String,
    pub author_name: String,
    pub created_at: OffsetDateTime,
}

impl Article {
    pub async fn create(
        pool: &PgPool,
        author_id: Uuid,
        title: &str,
        summary: &str,
        content: &str,
        tags: &[String],
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let slug = super::asset::slugify_text(title, id);
        let now = OffsetDateTime::now_utc();

        sqlx::query_as::<_, Article>(
            r#"
            INSERT INTO articles (id, author_id, title, slug, summary, content, tags, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(author_id)
        .bind(title)
        .bind(&slug)
        .bind(summary)
        .bind(content)
        .bind(tags)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_published(
        pool: &PgPool,
        tag: Option<&str>,
        sort: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<ArticleWithAuthor>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let order = match sort {
            "popular" => "a.likes DESC",
            "views" => "a.views DESC",
            _ => "a.created_at DESC",
        };

        let articles = sqlx::query_as::<_, ArticleWithAuthor>(&format!(
            r#"
            SELECT a.id, a.title, a.slug, a.summary, a.tags, a.cover_image_url,
                   a.likes, a.views, u.username AS author_name, a.created_at
            FROM articles a
            JOIN users u ON u.id = a.author_id
            WHERE a.published = true
              AND ($1::text IS NULL OR $1 = ANY(a.tags))
            ORDER BY {order}
            LIMIT $2 OFFSET $3
            "#,
        ))
        .bind(tag)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let total: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM articles a
            WHERE a.published = true
              AND ($1::text IS NULL OR $1 = ANY(a.tags))
            "#,
        )
        .bind(tag)
        .fetch_one(pool)
        .await?;

        Ok((articles, total.0))
    }

    pub async fn list_by_author(
        pool: &PgPool,
        author_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Article>(
            "SELECT * FROM articles WHERE author_id = $1 ORDER BY created_at DESC",
        )
        .bind(author_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        title: Option<&str>,
        summary: Option<&str>,
        content: Option<&str>,
        tags: Option<&[String]>,
        published: Option<bool>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Article>(
            r#"
            UPDATE articles SET
                title = COALESCE($2, title),
                summary = COALESCE($3, summary),
                content = COALESCE($4, content),
                tags = COALESCE($5, tags),
                published = COALESCE($6, published),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(summary)
        .bind(content)
        .bind(tags)
        .bind(published)
        .fetch_one(pool)
        .await
    }

    pub async fn increment_views(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE articles SET views = views + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn toggle_like(
        pool: &PgPool,
        user_id: Uuid,
        article_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        // Try to insert; if already exists, delete it
        let existing: Option<(Uuid,)> = sqlx::query_as(
            "SELECT user_id FROM article_likes WHERE user_id = $1 AND article_id = $2",
        )
        .bind(user_id)
        .bind(article_id)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            sqlx::query("DELETE FROM article_likes WHERE user_id = $1 AND article_id = $2")
                .bind(user_id)
                .bind(article_id)
                .execute(pool)
                .await?;
            sqlx::query("UPDATE articles SET likes = likes - 1 WHERE id = $1")
                .bind(article_id)
                .execute(pool)
                .await?;
            Ok(false) // unliked
        } else {
            sqlx::query("INSERT INTO article_likes (user_id, article_id) VALUES ($1, $2)")
                .bind(user_id)
                .bind(article_id)
                .execute(pool)
                .await?;
            sqlx::query("UPDATE articles SET likes = likes + 1 WHERE id = $1")
                .bind(article_id)
                .execute(pool)
                .await?;
            Ok(true) // liked
        }
    }

    pub async fn has_liked(
        pool: &PgPool,
        user_id: Uuid,
        article_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "SELECT user_id FROM article_likes WHERE user_id = $1 AND article_id = $2",
        )
        .bind(user_id)
        .bind(article_id)
        .fetch_optional(pool)
        .await?;
        Ok(row.is_some())
    }
}

impl ArticleComment {
    pub async fn create(
        pool: &PgPool,
        article_id: Uuid,
        author_id: Uuid,
        content: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, ArticleComment>(
            r#"
            INSERT INTO article_comments (id, article_id, author_id, content)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(article_id)
        .bind(author_id)
        .bind(content)
        .fetch_one(pool)
        .await
    }

    pub async fn list_for_article(
        pool: &PgPool,
        article_id: Uuid,
    ) -> Result<Vec<CommentWithAuthor>, sqlx::Error> {
        sqlx::query_as::<_, CommentWithAuthor>(
            r#"
            SELECT c.id, c.content, u.username AS author_name, c.created_at
            FROM article_comments c
            JOIN users u ON u.id = c.author_id
            WHERE c.article_id = $1
            ORDER BY c.created_at ASC
            "#,
        )
        .bind(article_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, author_id: Uuid) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("DELETE FROM article_comments WHERE id = $1 AND author_id = $2")
                .bind(id)
                .bind(author_id)
                .execute(pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }
}
