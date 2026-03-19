use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::asset::{self, Asset};
use renzora_models::user::User;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/upload", post(upload_asset))
        .route("/my-assets", get(my_assets))
        .route("/:id/update", put(update_asset))
        .route("/:id/download", get(download_asset))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_assets))
        .route("/detail/:slug", get(get_asset))
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
                let path = format!("{}/assets/{}", state.upload_dir, stored_name);

                tokio::fs::create_dir_all(format!("{}/assets", state.upload_dir))
                    .await
                    .map_err(|e| ApiError::Internal(format!("Failed to create upload dir: {e}")))?;

                tokio::fs::write(&path, &data)
                    .await
                    .map_err(|e| ApiError::Internal(format!("Failed to write file: {e}")))?;

                file_path = Some(format!("{}/assets/{}", state.upload_base_url, stored_name));
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
                let path = format!("{}/thumbnails/{}", state.upload_dir, stored_name);

                tokio::fs::create_dir_all(format!("{}/thumbnails", state.upload_dir))
                    .await
                    .map_err(|e| ApiError::Internal(format!("Failed to create thumbnail dir: {e}")))?;

                tokio::fs::write(&path, &data)
                    .await
                    .map_err(|e| ApiError::Internal(format!("Failed to write thumbnail: {e}")))?;

                thumb_path = Some(format!("{}/thumbnails/{}", state.upload_base_url, stored_name));
            }
            _ => {}
        }
    }

    let meta = metadata.ok_or(ApiError::Validation("Missing metadata field".into()))?;

    // Validate category
    match meta.category.as_str() {
        "plugin" | "theme" | "asset" => {}
        _ => return Err(ApiError::Validation("Category must be 'plugin', 'theme', or 'asset'".into())),
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
