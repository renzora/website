use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub media_urls: Vec<String>,
    pub visibility: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PostWithAuthor {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub body: String,
    pub media_urls: Vec<String>,
    pub visibility: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub is_liked: bool,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PostComment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub parent_id: Option<Uuid>,
    pub like_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct CommentWithAuthor {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub avatar_url: Option<String>,
    pub body: String,
    pub parent_id: Option<Uuid>,
    pub like_count: i32,
    pub is_liked: bool,
    pub created_at: OffsetDateTime,
}

impl Post {
    pub async fn create(pool: &PgPool, user_id: Uuid, body: &str, media_urls: &[String], visibility: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO posts (user_id, body, media_urls, visibility) VALUES ($1, $2, $3, $4) RETURNING *"
        ).bind(user_id).bind(body).bind(media_urls).bind(visibility)
        .fetch_one(pool).await
    }

    /// Feed: posts from users the viewer follows + own posts
    pub async fn feed(pool: &PgPool, viewer_id: Uuid, limit: i64, before_id: Option<Uuid>) -> Result<Vec<PostWithAuthor>, sqlx::Error> {
        let before_clause = if before_id.is_some() {
            "AND p.created_at < (SELECT created_at FROM posts WHERE id = $3)"
        } else {
            "AND ($3::uuid IS NULL OR true)"
        };

        let query = format!(
            "SELECT p.id, p.user_id, u.username, u.avatar_url, u.role, p.body, p.media_urls, p.visibility, p.like_count, p.comment_count, \
             EXISTS(SELECT 1 FROM post_likes pl WHERE pl.post_id = p.id AND pl.user_id = $1) as is_liked, \
             p.created_at \
             FROM posts p JOIN users u ON u.id = p.user_id \
             WHERE (p.user_id = $1 OR p.user_id IN (SELECT following_id FROM follows WHERE follower_id = $1)) \
             AND (p.visibility = 'public' OR p.user_id = $1 \
                  OR (p.visibility = 'followers' AND p.user_id IN (SELECT following_id FROM follows WHERE follower_id = $1)) \
                  OR (p.visibility = 'friends' AND p.user_id IN (SELECT friend_id FROM friends WHERE user_id = $1 AND status = 'accepted'))) \
             {} ORDER BY p.created_at DESC LIMIT $2",
            before_clause
        );

        sqlx::query_as::<_, PostWithAuthor>(&query)
            .bind(viewer_id).bind(limit).bind(before_id)
            .fetch_all(pool).await
    }

    /// Posts by a specific user (for profile page)
    pub async fn list_by_user(pool: &PgPool, user_id: Uuid, viewer_id: Option<Uuid>, limit: i64) -> Result<Vec<PostWithAuthor>, sqlx::Error> {
        let vid = viewer_id.unwrap_or(Uuid::nil());
        sqlx::query_as::<_, PostWithAuthor>(
            "SELECT p.id, p.user_id, u.username, u.avatar_url, u.role, p.body, p.media_urls, p.visibility, p.like_count, p.comment_count, \
             EXISTS(SELECT 1 FROM post_likes pl WHERE pl.post_id = p.id AND pl.user_id = $3) as is_liked, \
             p.created_at \
             FROM posts p JOIN users u ON u.id = p.user_id \
             WHERE p.user_id = $1 AND (p.visibility = 'public' OR p.user_id = $3) \
             ORDER BY p.created_at DESC LIMIT $2"
        ).bind(user_id).bind(limit).bind(vid)
        .fetch_all(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM posts WHERE id = $1")
            .bind(id).fetch_optional(pool).await
    }

    pub async fn toggle_like(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let existing = sqlx::query_as::<_, (Uuid,)>(
            "SELECT id FROM post_likes WHERE post_id = $1 AND user_id = $2"
        ).bind(post_id).bind(user_id).fetch_optional(pool).await?;

        if existing.is_some() {
            sqlx::query("DELETE FROM post_likes WHERE post_id = $1 AND user_id = $2")
                .bind(post_id).bind(user_id).execute(pool).await?;
            sqlx::query("UPDATE posts SET like_count = like_count - 1 WHERE id = $1")
                .bind(post_id).execute(pool).await?;
            Ok(false)
        } else {
            sqlx::query("INSERT INTO post_likes (post_id, user_id) VALUES ($1, $2)")
                .bind(post_id).bind(user_id).execute(pool).await?;
            sqlx::query("UPDATE posts SET like_count = like_count + 1 WHERE id = $1")
                .bind(post_id).execute(pool).await?;
            Ok(true)
        }
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM posts WHERE id = $1 AND user_id = $2")
            .bind(id).bind(user_id).execute(pool).await?;
        Ok(r.rows_affected() > 0)
    }
}

impl PostComment {
    pub async fn create(pool: &PgPool, post_id: Uuid, user_id: Uuid, body: &str, parent_id: Option<Uuid>) -> Result<Self, sqlx::Error> {
        sqlx::query("UPDATE posts SET comment_count = comment_count + 1 WHERE id = $1")
            .bind(post_id).execute(pool).await?;
        sqlx::query_as::<_, Self>(
            "INSERT INTO post_comments (post_id, user_id, body, parent_id) VALUES ($1, $2, $3, $4) RETURNING *"
        ).bind(post_id).bind(user_id).bind(body).bind(parent_id)
        .fetch_one(pool).await
    }

    pub async fn list_for_post(pool: &PgPool, post_id: Uuid, viewer_id: Option<Uuid>, limit: i64, offset: i64) -> Result<Vec<CommentWithAuthor>, sqlx::Error> {
        let vid = viewer_id.unwrap_or(Uuid::nil());
        sqlx::query_as::<_, CommentWithAuthor>(
            "SELECT c.id, c.post_id, c.user_id, u.username, u.avatar_url, c.body, c.parent_id, c.like_count, \
             EXISTS(SELECT 1 FROM comment_likes cl WHERE cl.comment_id = c.id AND cl.user_id = $3) as is_liked, \
             c.created_at \
             FROM post_comments c JOIN users u ON u.id = c.user_id \
             WHERE c.post_id = $1 ORDER BY c.created_at ASC LIMIT $2 OFFSET $4"
        ).bind(post_id).bind(limit).bind(vid).bind(offset)
        .fetch_all(pool).await
    }

    pub async fn toggle_like(pool: &PgPool, comment_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let existing = sqlx::query_as::<_, (Uuid,)>(
            "SELECT id FROM comment_likes WHERE comment_id = $1 AND user_id = $2"
        ).bind(comment_id).bind(user_id).fetch_optional(pool).await?;

        if existing.is_some() {
            sqlx::query("DELETE FROM comment_likes WHERE comment_id = $1 AND user_id = $2")
                .bind(comment_id).bind(user_id).execute(pool).await?;
            sqlx::query("UPDATE post_comments SET like_count = like_count - 1 WHERE id = $1")
                .bind(comment_id).execute(pool).await?;
            Ok(false)
        } else {
            sqlx::query("INSERT INTO comment_likes (comment_id, user_id) VALUES ($1, $2)")
                .bind(comment_id).bind(user_id).execute(pool).await?;
            sqlx::query("UPDATE post_comments SET like_count = like_count + 1 WHERE id = $1")
                .bind(comment_id).execute(pool).await?;
            Ok(true)
        }
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let comment = sqlx::query_as::<_, (Uuid,)>("SELECT post_id FROM post_comments WHERE id = $1 AND user_id = $2")
            .bind(id).bind(user_id).fetch_optional(pool).await?;
        if let Some((post_id,)) = comment {
            sqlx::query("DELETE FROM post_comments WHERE id = $1").bind(id).execute(pool).await?;
            sqlx::query("UPDATE posts SET comment_count = comment_count - 1 WHERE id = $1 AND comment_count > 0")
                .bind(post_id).execute(pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
