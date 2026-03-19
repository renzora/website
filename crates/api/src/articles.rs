use axum::{
    extract::{Extension, Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::article::{Article, ArticleComment};
use renzora_models::user::User;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/create", post(create_article))
        .route("/my-articles", get(my_articles))
        .route("/{id}/update", put(update_article))
        .route("/{id}/like", post(toggle_like))
        .route("/{id}/comment", post(add_comment))
        .route("/comment/{id}", delete(delete_comment))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_articles))
        .route("/detail/{slug}", get(get_article))
        .merge(protected)
}

/// List published community articles.
async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleListQuery>,
) -> Result<Json<ArticleListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page: i64 = 20;
    let sort = params.sort.as_deref().unwrap_or("newest");

    let (articles, total) = Article::list_published(
        &state.db,
        params.tag.as_deref(),
        sort,
        page,
        per_page,
    )
    .await?;

    let summaries: Vec<ArticleSummary> = articles
        .into_iter()
        .map(|a| ArticleSummary {
            id: a.id,
            title: a.title,
            slug: a.slug,
            summary: a.summary,
            tags: a.tags,
            cover_image_url: a.cover_image_url,
            likes: a.likes,
            views: a.views,
            author_name: a.author_name,
            created_at: a.created_at.to_string(),
        })
        .collect();

    Ok(Json(ArticleListResponse {
        articles: summaries,
        total,
        page,
    }))
}

/// Get a single article by slug (increments view count).
async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ArticleDetailResponse>, ApiError> {
    let article = Article::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !article.published {
        return Err(ApiError::NotFound);
    }

    // Increment views
    Article::increment_views(&state.db, article.id).await?;

    let author = User::find_by_id(&state.db, article.author_id)
        .await?
        .ok_or(ApiError::Internal("Author not found".into()))?;

    let comments = ArticleComment::list_for_article(&state.db, article.id).await?;
    let comment_responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|c| CommentResponse {
            id: c.id,
            content: c.content,
            author_name: c.author_name,
            created_at: c.created_at.to_string(),
        })
        .collect();

    Ok(Json(ArticleDetailResponse {
        id: article.id,
        title: article.title,
        slug: article.slug,
        summary: article.summary,
        content: article.content,
        tags: article.tags,
        cover_image_url: article.cover_image_url,
        likes: article.likes,
        views: article.views + 1,
        author: UserProfile {
            id: author.id,
            username: author.username,
            email: author.email,
            role: author.role,
            credit_balance: author.credit_balance,
        },
        created_at: article.created_at.to_string(),
        updated_at: article.updated_at.to_string(),
        user_has_liked: None,
        comments: comment_responses,
    }))
}

/// Create a new article.
async fn create_article(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateArticleRequest>,
) -> Result<Json<ArticleSummary>, ApiError> {
    if body.title.is_empty() || body.title.len() > 255 {
        return Err(ApiError::Validation("Title must be 1-255 characters".into()));
    }
    if body.content.is_empty() {
        return Err(ApiError::Validation("Content cannot be empty".into()));
    }

    let article = Article::create(
        &state.db,
        auth.user_id,
        &body.title,
        &body.summary,
        &body.content,
        &body.tags,
    )
    .await?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(ArticleSummary {
        id: article.id,
        title: article.title,
        slug: article.slug,
        summary: article.summary,
        tags: article.tags,
        cover_image_url: article.cover_image_url,
        likes: 0,
        views: 0,
        author_name: user.username,
        created_at: article.created_at.to_string(),
    }))
}

/// Update an article (author only).
async fn update_article(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateArticleRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    let article = Article::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if article.author_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    Article::update(
        &state.db,
        id,
        body.title.as_deref(),
        body.summary.as_deref(),
        body.content.as_deref(),
        body.tags.as_deref(),
        body.published,
    )
    .await?;

    Ok(Json(MessageResponse {
        message: "Article updated".into(),
    }))
}

/// Toggle like on an article.
async fn toggle_like(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<LikeResponse>, ApiError> {
    let article = Article::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let liked = Article::toggle_like(&state.db, auth.user_id, id).await?;

    let new_likes = if liked {
        article.likes + 1
    } else {
        article.likes - 1
    };

    Ok(Json(LikeResponse {
        liked,
        total_likes: new_likes,
    }))
}

/// Add a comment to an article.
async fn add_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    if body.content.is_empty() || body.content.len() > 2000 {
        return Err(ApiError::Validation(
            "Comment must be 1-2000 characters".into(),
        ));
    }

    // Verify article exists
    Article::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let comment = ArticleComment::create(&state.db, id, auth.user_id, &body.content).await?;

    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(CommentResponse {
        id: comment.id,
        content: comment.content,
        author_name: user.username,
        created_at: comment.created_at.to_string(),
    }))
}

/// Delete a comment (author only).
async fn delete_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, ApiError> {
    let deleted = ArticleComment::delete(&state.db, id, auth.user_id).await?;

    if !deleted {
        return Err(ApiError::NotFound);
    }

    Ok(Json(MessageResponse {
        message: "Comment deleted".into(),
    }))
}

/// List the authenticated user's articles.
async fn my_articles(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<ArticleSummary>>, ApiError> {
    let articles = Article::list_by_author(&state.db, auth.user_id).await?;
    let user = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let summaries: Vec<ArticleSummary> = articles
        .into_iter()
        .map(|a| ArticleSummary {
            id: a.id,
            title: a.title,
            slug: a.slug,
            summary: a.summary,
            tags: a.tags,
            cover_image_url: a.cover_image_url,
            likes: a.likes,
            views: a.views,
            author_name: user.username.clone(),
            created_at: a.created_at.to_string(),
        })
        .collect();

    Ok(Json(summaries))
}
