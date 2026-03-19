use axum::{extract::{Extension, Path, Query, State}, routing::{get, post, put, delete}, Json, Router};
use renzora_models::course::{self, Course, Chapter};
use renzora_models::user::User;
use serde::Deserialize;
use uuid::Uuid;
use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/create", post(create_course))
        .route("/my-courses", get(my_courses))
        .route("/:slug/update", put(update_course))
        .route("/:slug/chapters", post(add_chapter))
        .route("/:slug/chapters/:chapter_slug", put(update_chapter))
        .route("/:slug/chapters/:chapter_id/delete", delete(delete_chapter))
        .route("/:slug/enroll", post(enroll_course))
        .route("/:slug/chapters/:chapter_id/complete", post(complete_chapter))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_courses))
        .route("/:slug", get(get_course))
        .route("/:slug/chapters/:chapter_slug/view", get(view_chapter))
        .merge(protected)
}

#[derive(Deserialize)]
struct ListQuery { category: Option<String>, page: Option<i64> }

async fn list_courses(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let (courses, total) = Course::list_published(&state.db, params.category.as_deref(), params.page.unwrap_or(1)).await?;
    let items: Vec<serde_json::Value> = courses.iter().map(|c| {
        let rating = if c.rating_count > 0 { c.rating_sum as f64 / c.rating_count as f64 } else { 0.0 };
        serde_json::json!({
            "id": c.id, "title": c.title, "slug": c.slug, "description": c.description,
            "cover_image_url": c.cover_image_url, "category": c.category, "difficulty": c.difficulty,
            "price_credits": c.price_credits, "chapter_count": c.chapter_count,
            "enrolled_count": c.enrolled_count, "rating": rating, "rating_count": c.rating_count,
            "creator_name": c.creator_name, "created_at": c.created_at.to_string(),
        })
    }).collect();
    Ok(Json(serde_json::json!({"courses": items, "total": total})))
}

async fn get_course(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let course = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    let creator = User::find_by_id(&state.db, course.creator_id).await?.ok_or(ApiError::NotFound)?;
    let chapters = Chapter::list_for_course(&state.db, course.id).await?;
    let rating = if course.rating_count > 0 { course.rating_sum as f64 / course.rating_count as f64 } else { 0.0 };

    let chapter_list: Vec<serde_json::Value> = chapters.iter().map(|ch| serde_json::json!({
        "id": ch.id, "title": ch.title, "slug": ch.slug, "sort_order": ch.sort_order,
        "duration_minutes": ch.duration_minutes, "is_free_preview": ch.is_free_preview,
    })).collect();

    Ok(Json(serde_json::json!({
        "id": course.id, "title": course.title, "slug": course.slug, "description": course.description,
        "cover_image_url": course.cover_image_url, "category": course.category, "difficulty": course.difficulty,
        "price_credits": course.price_credits, "published": course.published,
        "chapter_count": course.chapter_count, "enrolled_count": course.enrolled_count,
        "rating": rating, "rating_count": course.rating_count,
        "creator": {"id": creator.id, "username": creator.username, "role": creator.role},
        "chapters": chapter_list,
        "created_at": course.created_at.to_string(),
    })))
}

async fn view_chapter(
    State(state): State<AppState>,
    Path((course_slug, chapter_slug)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let course = Course::find_by_slug(&state.db, &course_slug).await?.ok_or(ApiError::NotFound)?;
    let chapter = Chapter::find_by_slug(&state.db, course.id, &chapter_slug).await?.ok_or(ApiError::NotFound)?;

    // Only return full content if free preview or course is free
    if !chapter.is_free_preview && course.price_credits > 0 {
        return Ok(Json(serde_json::json!({
            "id": chapter.id, "title": chapter.title, "slug": chapter.slug,
            "duration_minutes": chapter.duration_minutes, "is_free_preview": false,
            "locked": true, "content": "", "video_url": null,
        })));
    }

    Ok(Json(serde_json::json!({
        "id": chapter.id, "title": chapter.title, "slug": chapter.slug,
        "content": chapter.content, "video_url": chapter.video_url,
        "duration_minutes": chapter.duration_minutes, "is_free_preview": chapter.is_free_preview,
        "locked": false,
    })))
}

#[derive(Deserialize)]
struct CreateCourseBody { title: String, description: String, category: String, difficulty: String, price_credits: i64 }

async fn create_course(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateCourseBody>,
) -> Result<Json<Course>, ApiError> {
    if body.title.is_empty() { return Err(ApiError::Validation("Title required".into())); }
    let course = Course::create(&state.db, auth.user_id, &body.title, &body.description, &body.category, &body.difficulty, body.price_credits).await?;
    Ok(Json(course))
}

async fn my_courses(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Course>>, ApiError> {
    let courses = Course::list_by_creator(&state.db, auth.user_id).await?;
    Ok(Json(courses))
}

#[derive(Deserialize)]
struct UpdateCourseBody { title: Option<String>, description: Option<String>, category: Option<String>, difficulty: Option<String>, price_credits: Option<i64>, published: Option<bool> }

async fn update_course(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(slug): Path<String>,
    Json(body): Json<UpdateCourseBody>,
) -> Result<Json<Course>, ApiError> {
    let course = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if course.creator_id != auth.user_id { return Err(ApiError::Unauthorized); }
    let updated = Course::update(&state.db, course.id, body.title.as_deref(), body.description.as_deref(), body.category.as_deref(), body.difficulty.as_deref(), body.price_credits, body.published).await?;
    Ok(Json(updated))
}

#[derive(Deserialize)]
struct AddChapterBody { title: String, content: String, sort_order: Option<i32>, duration_minutes: Option<i32>, video_url: Option<String>, is_free_preview: Option<bool> }

async fn add_chapter(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(slug): Path<String>,
    Json(body): Json<AddChapterBody>,
) -> Result<Json<Chapter>, ApiError> {
    let course = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if course.creator_id != auth.user_id { return Err(ApiError::Unauthorized); }
    let ch = Chapter::create(&state.db, course.id, &body.title, &body.content, body.sort_order.unwrap_or(course.chapter_count + 1), body.duration_minutes.unwrap_or(0), body.video_url.as_deref(), body.is_free_preview.unwrap_or(false)).await?;
    Ok(Json(ch))
}

#[derive(Deserialize)]
struct UpdateChapterBody { title: Option<String>, content: Option<String>, sort_order: Option<i32>, duration_minutes: Option<i32>, video_url: Option<String>, is_free_preview: Option<bool> }

async fn update_chapter(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((slug, chapter_slug)): Path<(String, String)>,
    Json(body): Json<UpdateChapterBody>,
) -> Result<Json<Chapter>, ApiError> {
    let course = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if course.creator_id != auth.user_id { return Err(ApiError::Unauthorized); }
    let ch = Chapter::find_by_slug(&state.db, course.id, &chapter_slug).await?.ok_or(ApiError::NotFound)?;
    let updated = Chapter::update(&state.db, ch.id, body.title.as_deref(), body.content.as_deref(), body.sort_order, body.duration_minutes, body.video_url.as_deref(), body.is_free_preview).await?;
    Ok(Json(updated))
}

async fn delete_chapter(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((slug, chapter_id)): Path<(String, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let course = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if course.creator_id != auth.user_id { return Err(ApiError::Unauthorized); }
    Chapter::delete(&state.db, chapter_id, course.id).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

async fn enroll_course(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let course_val = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    if !course_val.published { return Err(ApiError::NotFound); }
    if course::is_enrolled(&state.db, auth.user_id, course_val.id).await? {
        return Err(ApiError::Validation("Already enrolled".into()));
    }
    // Handle payment for paid courses
    if course_val.price_credits > 0 {
        renzora_models::transaction::process_purchase(&state.db, auth.user_id, course_val.id, course_val.price_credits, course_val.creator_id).await
            .map_err(|e| if e.contains("Insufficient") { ApiError::Validation(e) } else { ApiError::Internal(e) })?;
    }
    course::enroll(&state.db, auth.user_id, course_val.id).await?;
    Ok(Json(serde_json::json!({"message": "Enrolled"})))
}

async fn complete_chapter(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((slug, chapter_id)): Path<(String, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let course_val = Course::find_by_slug(&state.db, &slug).await?.ok_or(ApiError::NotFound)?;
    course::mark_chapter_complete(&state.db, auth.user_id, course_val.id, chapter_id).await?;
    Ok(Json(serde_json::json!({"message": "Completed"})))
}
