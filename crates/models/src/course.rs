use serde::Serialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Course {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub cover_image_url: Option<String>,
    pub category: String,
    pub difficulty: String,
    pub price_credits: i64,
    pub published: bool,
    pub chapter_count: i32,
    pub enrolled_count: i32,
    pub rating_sum: i32,
    pub rating_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct CourseWithCreator {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub cover_image_url: Option<String>,
    pub category: String,
    pub difficulty: String,
    pub price_credits: i64,
    pub chapter_count: i32,
    pub enrolled_count: i32,
    pub rating_sum: i32,
    pub rating_count: i32,
    pub creator_name: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Chapter {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub sort_order: i32,
    pub duration_minutes: i32,
    pub video_url: Option<String>,
    pub is_free_preview: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Course {
    pub async fn create(pool: &PgPool, creator_id: Uuid, title: &str, description: &str, category: &str, difficulty: &str, price_credits: i64) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let slug = super::asset::slugify_text(title, id);
        sqlx::query_as("INSERT INTO courses (id,creator_id,title,slug,description,category,difficulty,price_credits) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *")
            .bind(id).bind(creator_id).bind(title).bind(&slug).bind(description).bind(category).bind(difficulty).bind(price_credits)
            .fetch_one(pool).await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM courses WHERE slug=$1").bind(slug).fetch_optional(pool).await
    }

    pub async fn list_published(pool: &PgPool, category: Option<&str>, page: i64) -> Result<(Vec<CourseWithCreator>, i64), sqlx::Error> {
        let per_page: i64 = 12;
        let offset = (page - 1) * per_page;
        let courses = sqlx::query_as::<_, CourseWithCreator>(
            "SELECT c.id,c.title,c.slug,c.description,c.cover_image_url,c.category,c.difficulty,c.price_credits,c.chapter_count,c.enrolled_count,c.rating_sum,c.rating_count,u.username as creator_name,c.created_at FROM courses c JOIN users u ON u.id=c.creator_id WHERE c.published=true AND ($1::text IS NULL OR c.category=$1) ORDER BY c.created_at DESC LIMIT $2 OFFSET $3"
        ).bind(category).bind(per_page).bind(offset).fetch_all(pool).await?;
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM courses WHERE published=true AND ($1::text IS NULL OR category=$1)")
            .bind(category).fetch_one(pool).await?;
        Ok((courses, total.0))
    }

    pub async fn list_by_creator(pool: &PgPool, creator_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM courses WHERE creator_id=$1 ORDER BY created_at DESC").bind(creator_id).fetch_all(pool).await
    }

    pub async fn update(pool: &PgPool, id: Uuid, title: Option<&str>, description: Option<&str>, category: Option<&str>, difficulty: Option<&str>, price_credits: Option<i64>, published: Option<bool>) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE courses SET title=COALESCE($2,title),description=COALESCE($3,description),category=COALESCE($4,category),difficulty=COALESCE($5,difficulty),price_credits=COALESCE($6,price_credits),published=COALESCE($7,published),updated_at=NOW() WHERE id=$1 RETURNING *")
            .bind(id).bind(title).bind(description).bind(category).bind(difficulty).bind(price_credits).bind(published)
            .fetch_one(pool).await
    }
}

impl Chapter {
    pub async fn create(pool: &PgPool, course_id: Uuid, title: &str, content: &str, sort_order: i32, duration_minutes: i32, video_url: Option<&str>, is_free_preview: bool) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let slug = super::asset::slugify_text(title, id);
        let ch = sqlx::query_as("INSERT INTO course_chapters (id,course_id,title,slug,content,sort_order,duration_minutes,video_url,is_free_preview) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *")
            .bind(id).bind(course_id).bind(title).bind(&slug).bind(content).bind(sort_order).bind(duration_minutes).bind(video_url).bind(is_free_preview)
            .fetch_one(pool).await?;
        sqlx::query("UPDATE courses SET chapter_count=chapter_count+1,updated_at=NOW() WHERE id=$1").bind(course_id).execute(pool).await?;
        Ok(ch)
    }

    pub async fn list_for_course(pool: &PgPool, course_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM course_chapters WHERE course_id=$1 ORDER BY sort_order").bind(course_id).fetch_all(pool).await
    }

    pub async fn find_by_slug(pool: &PgPool, course_id: Uuid, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM course_chapters WHERE course_id=$1 AND slug=$2").bind(course_id).bind(slug).fetch_optional(pool).await
    }

    pub async fn update(pool: &PgPool, id: Uuid, title: Option<&str>, content: Option<&str>, sort_order: Option<i32>, duration_minutes: Option<i32>, video_url: Option<&str>, is_free_preview: Option<bool>) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE course_chapters SET title=COALESCE($2,title),content=COALESCE($3,content),sort_order=COALESCE($4,sort_order),duration_minutes=COALESCE($5,duration_minutes),video_url=COALESCE($6,video_url),is_free_preview=COALESCE($7,is_free_preview),updated_at=NOW() WHERE id=$1 RETURNING *")
            .bind(id).bind(title).bind(content).bind(sort_order).bind(duration_minutes).bind(video_url).bind(is_free_preview)
            .fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, course_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM course_chapters WHERE id=$1").bind(id).execute(pool).await?;
        sqlx::query("UPDATE courses SET chapter_count=GREATEST(chapter_count-1,0),updated_at=NOW() WHERE id=$1").bind(course_id).execute(pool).await?;
        Ok(())
    }
}

pub async fn is_enrolled(pool: &PgPool, user_id: Uuid, course_id: Uuid) -> Result<bool, sqlx::Error> {
    let r: Option<(Uuid,)> = sqlx::query_as("SELECT user_id FROM enrollments WHERE user_id=$1 AND course_id=$2")
        .bind(user_id).bind(course_id).fetch_optional(pool).await?;
    Ok(r.is_some())
}

pub async fn enroll(pool: &PgPool, user_id: Uuid, course_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO enrollments (user_id,course_id) VALUES ($1,$2) ON CONFLICT DO NOTHING")
        .bind(user_id).bind(course_id).execute(pool).await?;
    sqlx::query("UPDATE courses SET enrolled_count=enrolled_count+1 WHERE id=$1").bind(course_id).execute(pool).await?;
    Ok(())
}

pub async fn mark_chapter_complete(pool: &PgPool, user_id: Uuid, course_id: Uuid, chapter_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE enrollments SET completed_chapters=array_append(completed_chapters,$3),last_accessed_at=NOW() WHERE user_id=$1 AND course_id=$2 AND NOT ($3=ANY(completed_chapters))")
        .bind(user_id).bind(course_id).bind(chapter_id).execute(pool).await?;
    Ok(())
}
