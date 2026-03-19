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
        .route("/:id/update", put(update_asset))
        .route("/:id/download", get(download_asset))
        .route("/:id/comments", post(add_comment))
        .route("/comments/:comment_id", delete(delete_comment))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_assets))
        .route("/categories", get(list_categories))
        .route("/detail/:slug", get(get_asset))
        .route("/:id/comments", get(list_comments))
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

    let (assets, total) = Asset::list_published(
        &state.db,
        params.q.as_deref(),
        params.category.as_deref(),
        sort,
        page,
        per_page,
    )
    .await?;

    let summaries = assets
        .into_iter()
        .map(|a| AssetSummary {
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
) -> Result<Json<AssetDetail>, ApiError> {
    let asset = Asset::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    let creator = User::find_by_id(&state.db, asset.creator_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    Ok(Json(asset_to_detail(&asset, &creator, None)))
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
    Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
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

/// Upload a file to S3 (DigitalOcean Spaces) or fall back to local disk.
async fn upload_to_storage(state: &AppState, key: &str, data: Vec<u8>) -> Result<String, ApiError> {
    if let Some(s3) = &state.s3_client {
        // Upload to S3
        s3.put_object()
            .bucket(&state.s3_bucket)
            .key(key)
            .body(data.into())
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .send()
            .await
            .map_err(|e| ApiError::Internal(format!("S3 upload failed: {e}")))?;

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
        },
        created_at: asset.created_at.to_string(),
        updated_at: asset.updated_at.to_string(),
        owned,
    }
}
