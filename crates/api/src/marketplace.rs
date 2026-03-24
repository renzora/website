use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;
use renzora_common::types::*;
use renzora_models::asset::{self, Asset};
use renzora_models::category::Category;
use renzora_models::user::User;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/upload", post(upload_asset))
        .route("/my-assets", get(my_assets))
        .route("/purchased", get(purchased_assets))
        .route("/:id/update", put(update_asset))
        .route("/:id/download", get(download_asset))
        .route("/:id/comments", post(add_comment))
        .route("/comments/:comment_id", delete(delete_comment))
        .route("/:id/reviews", post(submit_review))
        .route("/:id/media", post(upload_media))
        .route("/media/:media_id", delete(delete_media))
        .route("/:id/reviews/flag", post(flag_review))
        .route("/:id/reviews/helpful", post(mark_review_helpful))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_assets))
        .route("/categories", get(list_categories))
        .route("/detail/:slug", get(get_asset))
        .route("/:id/comments", get(list_comments))
        .route("/:id/reviews", get(list_reviews))
        .route("/:id/media", get(list_media))
        .merge(protected)
}

/// Browse/search marketplace assets.
async fn list_assets(
    State(state): State<AppState>,
    Query(params): Query<MarketplaceQuery>,
) -> Result<Json<MarketplaceListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page: i64 = 24;
    let sort = params.sort.as_deref().unwrap_or("newest");

    let (assets, total) = Asset::list_published_filtered(
        &state.db,
        params.q.as_deref(),
        params.category.as_deref(),
        sort,
        page,
        per_page,
        params.free,
        params.min_rating,
        params.max_price,
    )
    .await?;

    let summaries = assets
        .into_iter()
        .map(|a| {
            let rating_avg = if a.rating_count > 0 { a.rating_sum as f64 / a.rating_count as f64 } else { 0.0 };
            AssetSummary {
                id: a.id,
                name: a.name,
                slug: a.slug,
                description: a.description,
                category: a.category,
                price_credits: a.price_credits,
                thumbnail_url: a.thumbnail_url,
                version: a.version,
                downloads: a.downloads,
                creator_name: a.creator_name,
                rating_avg,
                rating_count: a.rating_count,
            }
        })
        .collect();

    Ok(Json(MarketplaceListResponse {
        assets: summaries,
        total,
        page,
        per_page,
    }))
}

/// List all marketplace categories.
async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, ApiError> {
    let cats = Category::list(&state.db).await?;
    Ok(Json(cats))
}

/// Get a single asset by slug.
async fn get_asset(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<AssetDetail>, ApiError> {
    let asset = Asset::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let creator = User::find_by_id(&state.db, asset.creator_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    // Optionally check ownership if user is authenticated
    let owned = if let Some(user_id) = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| crate::jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub)
    {
        if user_id == asset.creator_id {
            Some(true)
        } else {
            Some(asset::user_owns_asset(&state.db, user_id, asset.id).await?)
        }
    } else {
        None
    };

    Ok(Json(asset_to_detail(&asset, &creator, owned)))
}

/// Upload a new asset (multipart: JSON metadata + file + optional thumbnail).
async fn upload_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<AssetDetail>, ApiError> {
    let mut metadata: Option<UploadAssetRequest> = None;
    let mut file_path: Option<String> = None;
    let mut thumb_path: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::Validation(format!("Failed to read multipart field: {e}"))
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "metadata" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read metadata: {e}")))?;
                metadata = Some(
                    serde_json::from_str(&text)
                        .map_err(|e| ApiError::Validation(format!("Invalid metadata JSON: {e}")))?,
                );
            }
            "file" => {
                let filename = field
                    .file_name()
                    .unwrap_or("asset.zip")
                    .to_string();
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read file: {e}")))?;

                let stored_name = format!("{}-{}", Uuid::new_v4(), filename);
                let s3_key = format!("assets/{}", stored_name);
                file_path = Some(upload_to_storage(&state, &s3_key, data.to_vec()).await?);
            }
            "thumbnail" => {
                let filename = field
                    .file_name()
                    .unwrap_or("thumb.png")
                    .to_string();
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read thumbnail: {e}")))?;

                let stored_name = format!("{}-{}", Uuid::new_v4(), filename);
                let s3_key = format!("thumbnails/{}", stored_name);
                thumb_path = Some(upload_to_storage(&state, &s3_key, data.to_vec()).await?);
            }
            _ => {}
        }
    }

    let meta = metadata.ok_or(ApiError::Validation("Missing metadata field".into()))?;

    // Validate category exists in DB
    let cat_exists = renzora_models::category::Category::find_by_slug(&state.db, &meta.category).await?;
    if cat_exists.is_none() {
        return Err(ApiError::Validation(format!("Unknown category: '{}'", meta.category)));
    }

    let asset = Asset::create(
        &state.db,
        auth.user_id,
        &meta.name,
        &meta.description,
        &meta.category,
        meta.price_credits,
        &meta.version,
    )
    .await?;

    if let Some(url) = &file_path {
        Asset::update_file_url(&state.db, asset.id, url).await?;
    }
    if let Some(url) = &thumb_path {
        Asset::update_thumbnail_url(&state.db, asset.id, url).await?;
    }

    // Re-fetch with updated URLs
    let asset = Asset::find_by_id(&state.db, asset.id)
        .await?
        .ok_or(ApiError::Internal("Asset not found after creation".into()))?;

    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    // Upgrade user role to creator if needed
    if creator.role == "user" {
        sqlx::query("UPDATE users SET role = 'creator' WHERE id = $1")
            .bind(auth.user_id)
            .execute(&state.db)
            .await?;
    }

    Ok(Json(asset_to_detail(&asset, &creator, Some(true))))
}

/// Update an asset's metadata (creator only).
async fn update_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAssetRequest>,
) -> Result<Json<AssetDetail>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let updated = Asset::update_metadata(
        &state.db,
        id,
        body.name.as_deref(),
        body.description.as_deref(),
        body.price_credits,
        body.version.as_deref(),
        body.published,
    )
    .await?;

    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    Ok(Json(asset_to_detail(&updated, &creator, Some(true))))
}

/// Download an asset (requires auth + ownership or free).
async fn download_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<DownloadResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Check ownership: free assets are always accessible, paid require ownership
    if asset.price_credits > 0 {
        let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
        if !owns && asset.creator_id != auth.user_id {
            return Err(ApiError::Unauthorized);
        }
    }

    let file_url = asset
        .file_url
        .ok_or(ApiError::Internal("Asset has no file".into()))?;

    // Increment download counter
    Asset::increment_downloads(&state.db, id).await?;

    Ok(Json(DownloadResponse {
        download_url: file_url,
    }))
}

/// List the authenticated user's uploaded assets.
async fn my_assets(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<CreatorAssetsResponse>, ApiError> {
    let assets = Asset::list_by_creator(&state.db, auth.user_id).await?;

    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("User not found".into()))?;

    let details: Vec<AssetDetail> = assets
        .iter()
        .map(|a| asset_to_detail(a, &creator, Some(true)))
        .collect();

    Ok(Json(CreatorAssetsResponse { assets: details }))
}

/// List assets purchased/owned by the authenticated user.
async fn purchased_assets(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<MarketplaceListResponse>, ApiError> {
    let (assets, total) =
        Asset::list_purchased_by_user(&state.db, auth.user_id).await?;

    let summaries = assets
        .into_iter()
        .map(|a| {
            let rating_avg = if a.rating_count > 0 { a.rating_sum as f64 / a.rating_count as f64 } else { 0.0 };
            AssetSummary {
                id: a.id,
                name: a.name,
                slug: a.slug,
                description: a.description,
                category: a.category,
                price_credits: a.price_credits,
                thumbnail_url: a.thumbnail_url,
                version: a.version,
                downloads: a.downloads,
                creator_name: a.creator_name,
                rating_avg,
                rating_count: a.rating_count,
            }
        })
        .collect();

    Ok(Json(MarketplaceListResponse {
        assets: summaries,
        total,
        page: 1,
        per_page: total,
    }))
}

// ── Comments ──

async fn list_comments(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query("SELECT c.id, c.content, c.created_at, u.username as author_name, c.author_id FROM asset_comments c JOIN users u ON u.id=c.author_id WHERE c.asset_id=$1 ORDER BY c.created_at ASC")
        .bind(id).fetch_all(&state.db).await?;
    let comments: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "content": r.get::<String, _>("content"),
        "author_name": r.get::<String, _>("author_name"),
        "author_id": r.get::<Uuid, _>("author_id"),
        "created_at": r.get::<time::OffsetDateTime, _>("created_at").to_string(),
    })).collect();
    Ok(Json(serde_json::json!({"comments": comments})))
}

#[derive(Deserialize)]
struct CommentBody { content: String }

async fn add_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<CommentBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if body.content.is_empty() || body.content.len() > 2000 {
        return Err(ApiError::Validation("Comment must be 1-2000 characters".into()));
    }
    let asset = Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Only asset owners (purchasers) or the creator can comment
    let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
    if !owns && asset.creator_id != auth.user_id {
        return Err(ApiError::Validation("You must own this asset to comment".into()));
    }

    let cid = Uuid::new_v4();
    sqlx::query("INSERT INTO asset_comments (id,asset_id,author_id,content) VALUES ($1,$2,$3,$4)")
        .bind(cid).bind(id).bind(auth.user_id).bind(&body.content)
        .execute(&state.db).await?;
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(serde_json::json!({
        "id": cid, "content": body.content, "author_name": user.username, "author_id": auth.user_id,
    })))
}

async fn delete_comment(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(comment_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get the comment to check permissions
    let row = sqlx::query("SELECT c.author_id, a.creator_id FROM asset_comments c JOIN assets a ON a.id=c.asset_id WHERE c.id=$1")
        .bind(comment_id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;
    let comment_author: Uuid = row.get("author_id");
    let asset_creator: Uuid = row.get("creator_id");
    // Allow deletion by comment author, asset creator, or admin
    let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    if auth.user_id != comment_author && auth.user_id != asset_creator && user.role != "admin" {
        return Err(ApiError::Unauthorized);
    }
    sqlx::query("DELETE FROM asset_comments WHERE id=$1").bind(comment_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

// ── Reviews ──

/// List reviews for an asset.
async fn list_reviews(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    use renzora_models::review::Review;

    let reviews = Review::list_for_asset(&state.db, id).await?;

    // Get asset rating summary
    let asset = Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    let rating_avg = if asset.rating_count > 0 {
        asset.rating_sum as f64 / asset.rating_count as f64
    } else {
        0.0
    };

    let reviews_json: Vec<serde_json::Value> = reviews.iter().map(|r| serde_json::json!({
        "id": r.id,
        "asset_id": r.asset_id,
        "author_id": r.author_id,
        "rating": r.rating,
        "title": r.title,
        "content": r.content,
        "helpful_count": r.helpful_count,
        "flagged": r.flagged,
        "hidden": r.hidden,
        "created_at": r.created_at.to_string(),
        "author_name": r.author_name,
    })).collect();

    Ok(Json(serde_json::json!({
        "reviews": reviews_json,
        "rating_avg": rating_avg,
        "rating_count": asset.rating_count,
    })))
}

/// Submit or update a review (requires ownership).
#[derive(Deserialize)]
struct ReviewBody {
    rating: i32,
    title: Option<String>,
    content: Option<String>,
}

async fn submit_review(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<ReviewBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    use renzora_models::review::Review;
    use renzora_models::asset;

    // Must own the asset to review it
    let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
    let asset = Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    if !owns && asset.creator_id != auth.user_id {
        return Err(ApiError::Validation("You must own this asset to review it".into()));
    }

    // Can't review your own asset
    if asset.creator_id == auth.user_id {
        return Err(ApiError::Validation("You cannot review your own asset".into()));
    }

    if body.rating < 1 || body.rating > 5 {
        return Err(ApiError::Validation("Rating must be 1-5".into()));
    }

    let review = Review::upsert(
        &state.db,
        id,
        auth.user_id,
        body.rating,
        body.title.as_deref().unwrap_or(""),
        body.content.as_deref().unwrap_or(""),
    )
    .await
    .map_err(|e| ApiError::Internal(e))?;

    Ok(Json(serde_json::json!({
        "id": review.id,
        "message": "Review submitted",
    })))
}

/// Flag a review for moderation.
#[derive(Deserialize)]
struct FlagBody { review_id: Uuid, reason: String }

async fn flag_review(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthUser>,
    Path(_id): Path<Uuid>,
    Json(body): Json<FlagBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    use renzora_models::review::Review;

    if body.reason.is_empty() {
        return Err(ApiError::Validation("Reason is required".into()));
    }

    Review::flag(&state.db, body.review_id, &body.reason).await?;
    Ok(Json(serde_json::json!({"message": "Review flagged for moderation"})))
}

/// Mark a review as helpful.
#[derive(Deserialize)]
struct HelpfulBody { review_id: Uuid }

async fn mark_review_helpful(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthUser>,
    Path(_id): Path<Uuid>,
    Json(body): Json<HelpfulBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    use renzora_models::review::Review;
    Review::mark_helpful(&state.db, body.review_id).await?;
    Ok(Json(serde_json::json!({"message": "Marked as helpful"})))
}

/// Upload a file to S3 (DigitalOcean Spaces) or fall back to local disk.
pub async fn upload_to_storage(state: &AppState, key: &str, data: Vec<u8>) -> Result<String, ApiError> {
    if let Some(bucket) = &state.s3_bucket {
        // Upload to S3
        let response = bucket
            .put_object_with_content_type(key, &data, "application/octet-stream")
            .await
            .map_err(|e| ApiError::Internal(format!("S3 upload failed: {e}")))?;

        if response.status_code() != 200 {
            return Err(ApiError::Internal(format!(
                "S3 upload returned status {}",
                response.status_code()
            )));
        }

        Ok(format!("{}/{}", state.s3_public_url, key))
    } else {
        // Fallback to local storage
        let path = format!("{}/{}", state.upload_dir, key);
        if let Some(parent) = std::path::Path::new(&path).parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| ApiError::Internal(format!("Failed to create dir: {e}")))?;
        }
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| ApiError::Internal(format!("Failed to write file: {e}")))?;
        Ok(format!("{}/{}", state.upload_base_url, key))
    }
}

// ── Media gallery ──

async fn list_media(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let rows = sqlx::query("SELECT id, media_type, url, thumbnail_url, sort_order FROM asset_media WHERE asset_id=$1 ORDER BY sort_order, created_at")
        .bind(id).fetch_all(&state.db).await?;
    let media: Vec<serde_json::Value> = rows.iter().map(|r| serde_json::json!({
        "id": r.get::<Uuid, _>("id"),
        "media_type": r.get::<String, _>("media_type"),
        "url": r.get::<String, _>("url"),
        "thumbnail_url": r.get::<Option<String>, _>("thumbnail_url"),
        "sort_order": r.get::<i32, _>("sort_order"),
    })).collect();
    Ok(Json(media))
}

async fn upload_media(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let mut media_type = "image".to_string();
    let mut file_url: Option<String> = None;
    let mut thumb_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::Validation(format!("Failed to read field: {e}"))
    })? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "media_type" => {
                let val = field.text().await.unwrap_or_default();
                if val == "video" || val == "image" { media_type = val; }
            }
            "file" => {
                let filename = field.file_name().unwrap_or("media.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                let stored = format!("{}-{}", Uuid::new_v4(), filename);
                let key = format!("gallery/{}", stored);
                file_url = Some(upload_to_storage(&state, &key, data.to_vec()).await?);
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                let stored = format!("{}-{}", Uuid::new_v4(), filename);
                let key = format!("gallery/thumbs/{}", stored);
                thumb_url = Some(upload_to_storage(&state, &key, data.to_vec()).await?);
            }
            "video_url" => {
                let val = field.text().await.unwrap_or_default();
                if !val.is_empty() { file_url = Some(val); media_type = "video".to_string(); }
            }
            _ => {}
        }
    }

    let url = file_url.ok_or(ApiError::Validation("No file or video URL provided".into()))?;

    let row = sqlx::query("INSERT INTO asset_media (asset_id, media_type, url, thumbnail_url) VALUES ($1,$2,$3,$4) RETURNING id")
        .bind(id).bind(&media_type).bind(&url).bind(thumb_url.as_deref())
        .fetch_one(&state.db).await?;
    let media_id: Uuid = row.get("id");

    Ok(Json(serde_json::json!({
        "id": media_id,
        "media_type": media_type,
        "url": url,
        "thumbnail_url": thumb_url,
    })))
}

async fn delete_media(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(media_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query("SELECT m.asset_id, a.creator_id FROM asset_media m JOIN assets a ON a.id=m.asset_id WHERE m.id=$1")
        .bind(media_id).fetch_optional(&state.db).await?.ok_or(ApiError::NotFound)?;
    let creator_id: Uuid = row.get("creator_id");
    if creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }
    sqlx::query("DELETE FROM asset_media WHERE id=$1").bind(media_id).execute(&state.db).await?;
    Ok(Json(serde_json::json!({"message": "Deleted"})))
}

fn asset_to_detail(
    asset: &Asset,
    creator: &renzora_models::user::User,
    owned: Option<bool>,
) -> AssetDetail {
    AssetDetail {
        id: asset.id,
        name: asset.name.clone(),
        slug: asset.slug.clone(),
        description: asset.description.clone(),
        category: asset.category.clone(),
        price_credits: asset.price_credits,
        file_url: asset.file_url.clone(),
        thumbnail_url: asset.thumbnail_url.clone(),
        version: asset.version.clone(),
        downloads: asset.downloads,
        published: asset.published,
        creator: UserProfile {
            id: creator.id,
            username: creator.username.clone(),
            email: creator.email.clone(),
            role: creator.role.clone(),
            credit_balance: creator.credit_balance,
            discord_username: creator.discord_username.clone(),
            discord_avatar: creator.discord_avatar.clone(),
            totp_enabled: creator.totp_enabled,
        },
        created_at: asset.created_at.to_string(),
        updated_at: asset.updated_at.to_string(),
        owned,
    }
}
