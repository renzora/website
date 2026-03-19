use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct ForumCategory {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub sort_order: i32,
    pub thread_count: i32,
    pub post_count: i32,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ForumThread {
    pub id: Uuid,
    pub category_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub pinned: bool,
    pub locked: bool,
    pub post_count: i32,
    pub last_post_at: OffsetDateTime,
    pub last_post_by: Option<Uuid>,
    pub views: i32,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ThreadWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub pinned: bool,
    pub locked: bool,
    pub post_count: i32,
    pub views: i32,
    pub author_name: String,
    pub last_post_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ForumPost {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub is_first_post: bool,
    pub edited: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PostWithAuthor {
    pub id: Uuid,
    pub content: String,
    pub is_first_post: bool,
    pub edited: bool,
    pub author_id: Uuid,
    pub author_name: String,
    pub author_role: String,
    pub author_post_count: i32,
    pub created_at: OffsetDateTime,
}

impl ForumCategory {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM forum_categories ORDER BY sort_order").fetch_all(pool).await
    }
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM forum_categories WHERE slug = $1").bind(slug).fetch_optional(pool).await
    }
    pub async fn create(pool: &PgPool, name: &str, slug: &str, description: &str, icon: &str) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let sort: (i32,) = sqlx::query_as("SELECT COALESCE(MAX(sort_order),0)+1 FROM forum_categories").fetch_one(pool).await?;
        sqlx::query_as("INSERT INTO forum_categories (id,name,slug,description,icon,sort_order) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *")
            .bind(id).bind(name).bind(slug).bind(description).bind(icon).bind(sort.0).fetch_one(pool).await
    }
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM forum_categories WHERE id=$1").bind(id).execute(pool).await?; Ok(())
    }
}

impl ForumThread {
    pub async fn create(pool: &PgPool, category_id: Uuid, author_id: Uuid, title: &str, content: &str) -> Result<(Self, ForumPost), sqlx::Error> {
        let thread_id = Uuid::new_v4();
        let slug = super::asset::slugify_text(title, thread_id);
        let thread = sqlx::query_as::<_, ForumThread>(
            "INSERT INTO forum_threads (id,category_id,author_id,title,slug,last_post_by) VALUES ($1,$2,$3,$4,$5,$3) RETURNING *"
        ).bind(thread_id).bind(category_id).bind(author_id).bind(title).bind(&slug).fetch_one(pool).await?;

        let post_id = Uuid::new_v4();
        let post = sqlx::query_as::<_, ForumPost>(
            "INSERT INTO forum_posts (id,thread_id,author_id,content,is_first_post) VALUES ($1,$2,$3,$4,true) RETURNING *"
        ).bind(post_id).bind(thread_id).bind(author_id).bind(content).fetch_one(pool).await?;

        sqlx::query("UPDATE forum_categories SET thread_count=thread_count+1, post_count=post_count+1 WHERE id=$1").bind(category_id).execute(pool).await?;
        sqlx::query("UPDATE users SET post_count=post_count+1 WHERE id=$1").bind(author_id).execute(pool).await?;

        Ok((thread, post))
    }

    pub async fn list_by_category(pool: &PgPool, category_id: Uuid, page: i64) -> Result<(Vec<ThreadWithAuthor>, i64), sqlx::Error> {
        let per_page: i64 = 25;
        let offset = (page - 1) * per_page;
        let threads = sqlx::query_as::<_, ThreadWithAuthor>(
            "SELECT t.id,t.title,t.slug,t.pinned,t.locked,t.post_count,t.views,u.username as author_name,t.last_post_at,t.created_at FROM forum_threads t JOIN users u ON u.id=t.author_id WHERE t.category_id=$1 ORDER BY t.pinned DESC, t.last_post_at DESC LIMIT $2 OFFSET $3"
        ).bind(category_id).bind(per_page).bind(offset).fetch_all(pool).await?;
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM forum_threads WHERE category_id=$1").bind(category_id).fetch_one(pool).await?;
        Ok((threads, total.0))
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM forum_threads WHERE slug=$1").bind(slug).fetch_optional(pool).await
    }

    pub async fn increment_views(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE forum_threads SET views=views+1 WHERE id=$1").bind(id).execute(pool).await?; Ok(())
    }
}

impl ForumPost {
    pub async fn list_for_thread(pool: &PgPool, thread_id: Uuid, page: i64) -> Result<(Vec<PostWithAuthor>, i64), sqlx::Error> {
        let per_page: i64 = 20;
        let offset = (page - 1) * per_page;
        let posts = sqlx::query_as::<_, PostWithAuthor>(
            "SELECT p.id,p.content,p.is_first_post,p.edited,p.author_id,u.username as author_name,u.role as author_role,u.post_count as author_post_count,p.created_at FROM forum_posts p JOIN users u ON u.id=p.author_id WHERE p.thread_id=$1 ORDER BY p.created_at ASC LIMIT $2 OFFSET $3"
        ).bind(thread_id).bind(per_page).bind(offset).fetch_all(pool).await?;
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM forum_posts WHERE thread_id=$1").bind(thread_id).fetch_one(pool).await?;
        Ok((posts, total.0))
    }

    pub async fn create_reply(pool: &PgPool, thread_id: Uuid, author_id: Uuid, content: &str) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let post = sqlx::query_as::<_, ForumPost>(
            "INSERT INTO forum_posts (id,thread_id,author_id,content) VALUES ($1,$2,$3,$4) RETURNING *"
        ).bind(id).bind(thread_id).bind(author_id).bind(content).fetch_one(pool).await?;

        sqlx::query("UPDATE forum_threads SET post_count=post_count+1, last_post_at=NOW(), last_post_by=$2 WHERE id=$1")
            .bind(thread_id).bind(author_id).execute(pool).await?;
        let thread: ForumThread = sqlx::query_as("SELECT * FROM forum_threads WHERE id=$1").bind(thread_id).fetch_one(pool).await?;
        sqlx::query("UPDATE forum_categories SET post_count=post_count+1 WHERE id=$1").bind(thread.category_id).execute(pool).await?;
        sqlx::query("UPDATE users SET post_count=post_count+1 WHERE id=$1").bind(author_id).execute(pool).await?;

        Ok(post)
    }
}
