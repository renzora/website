use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;
use renzora_common::types::*;
use renzora_models::asset::{self, Asset};
use renzora_models::asset_file::AssetFile;
use renzora_models::category::Category;
use renzora_models::subcategory::Subcategory;
use renzora_models::tag::Tag;
use renzora_models::user::User;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, preview, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/upload", post(upload_asset))
        .route("/my-assets", get(my_assets))
        .route("/purchased", get(purchased_assets))
        .route("/:id/update", put(update_asset))
        .route("/:id/files", put(update_asset_files))
        .route("/:id/download", get(download_asset))
        .route("/:id/comments", post(add_comment))
        .route("/comments/:comment_id", delete(delete_comment))
        .route("/:id/reviews", post(submit_review))
        .route("/:id/media", post(upload_media))
        .route("/media/:media_id", delete(delete_media))
        .route("/:id/reviews/flag", post(flag_review))
        .route("/:id/reviews/helpful", post(mark_review_helpful))
        .route("/:id/delete", delete(delete_asset))
        .route("/:id/files/:file_id/download", get(download_single_file))
        .route("/:id/download-zip", get(download_all_zip))
        .route("/tags/submit", post(submit_tag))
        .route("/subcategories/submit", post(submit_subcategory))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_assets))
        .route("/categories", get(list_categories))
        .route("/subcategories", get(list_subcategories))
        .route("/tags", get(search_tags))
        .route("/detail/:slug", get(get_asset))
        .route("/:id/comments", get(list_comments))
        .route("/:id/reviews", get(list_reviews))
        .route("/:id/media", get(list_media))
        .route("/:id/asset-files", get(list_asset_files))
        .route("/:id/preview-file", get(preview_file_proxy))
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
        params.subcategory.as_deref(),
        params.tag.as_deref(),
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
                views: a.views,
                creator_name: a.creator_name,
                creator_avatar_url: a.creator_avatar_url,
                rating_avg,
                rating_count: a.rating_count,
                tags: a.tags,
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
    connect_info: axum::extract::ConnectInfo<std::net::SocketAddr>,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<AssetDetail>, ApiError> {
    let asset = Asset::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Extract authenticated user (if any)
    let user_id = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| crate::jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub);

    // Record view (deduplicated by IP, 24h cooldown)
    let ip = client_ip(&headers, &connect_info);
    let ip_hash = hash_ip(&ip);
    let _ = Asset::record_view(&state.db, asset.id, &ip_hash, user_id).await;

    let creator = User::find_by_id(&state.db, asset.creator_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    let owned = match user_id {
        Some(uid) if uid == asset.creator_id => Some(true),
        Some(uid) => Some(asset::user_owns_asset(&state.db, uid, asset.id).await?),
        None => None,
    };

    let mut detail = asset_to_detail(&asset, &creator, owned);

    // Populate file list with preview/download URLs based on ownership
    let asset_files = AssetFile::list_by_asset(&state.db, asset.id).await?;
    if !asset_files.is_empty() {
        let is_owned = owned.unwrap_or(false);
        let is_free = asset.price_credits == 0;
        detail.files = build_file_infos(&state, &asset_files, is_owned || is_free).await;
    }

    Ok(Json(detail))
}

/// Upload a new asset.
///
/// Multipart fields:
/// - `metadata` (required): JSON with name, description, category, price_credits, version,
///    tags, licence, ai_generated, metadata (material details etc.), zip_action ("keep"|"extract")
/// - `file` (required, repeatable): One or more asset files. If a single .zip with zip_action="extract",
///    the server will unpack it into individual files.
/// - `thumbnail` (optional): Cover image (.png, .jpg, .webp)
/// - `screenshot_0`..`screenshot_9` (optional): Gallery screenshots
/// - `video` (optional): Video preview (.mp4, .webm)
/// - `audio` (optional): Audio preview (.mp3, .wav, .ogg)
async fn upload_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<AssetDetail>, ApiError> {
    let mut metadata: Option<UploadAssetRequest> = None;
    let mut uploaded_files: Vec<(String, Vec<u8>)> = Vec::new(); // (filename, data)
    let mut thumb_path: Option<String> = None;
    let mut screenshots: Vec<String> = Vec::new();
    let mut video_url: Option<String> = None;
    let mut audio_url: Option<String> = None;

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
                if uploaded_files.len() >= 20 {
                    return Err(ApiError::Validation("Maximum 20 files per upload".into()));
                }
                let filename = field.file_name().unwrap_or("asset.zip").to_string();
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read file: {e}")))?;
                if data.len() > 200 * 1024 * 1024 {
                    return Err(ApiError::Validation("File exceeds 200MB limit".into()));
                }
                uploaded_files.push((filename, data.to_vec()));
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                validate_image_extension(&filename)?;
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read thumbnail: {e}")))?;
                if data.len() > 10 * 1024 * 1024 {
                    return Err(ApiError::Validation("Thumbnail exceeds 10MB limit".into()));
                }
                thumb_path = Some(upload_to_storage(&state, "thumbnails", &filename, data.to_vec()).await?);
            }
            name if name.starts_with("screenshot") => {
                if screenshots.len() >= 10 {
                    return Err(ApiError::Validation("Maximum 10 screenshots".into()));
                }
                let filename = field.file_name().unwrap_or("screenshot.png").to_string();
                validate_image_extension(&filename)?;
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read screenshot: {e}")))?;
                if data.len() > 10 * 1024 * 1024 {
                    return Err(ApiError::Validation("Screenshot exceeds 10MB limit".into()));
                }
                screenshots.push(upload_to_storage(&state, "gallery", &filename, data.to_vec()).await?);
            }
            "video" => {
                let filename = field.file_name().unwrap_or("preview.mp4").to_string();
                validate_video_extension(&filename)?;
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read video: {e}")))?;
                if data.len() > 100 * 1024 * 1024 {
                    return Err(ApiError::Validation("Video exceeds 100MB limit".into()));
                }
                video_url = Some(upload_to_storage(&state, "gallery", &filename, data.to_vec()).await?);
            }
            "audio" => {
                let filename = field.file_name().unwrap_or("preview.mp3").to_string();
                validate_audio_extension(&filename)?;
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read audio: {e}")))?;
                if data.len() > 50 * 1024 * 1024 {
                    return Err(ApiError::Validation("Audio exceeds 50MB limit".into()));
                }
                audio_url = Some(upload_to_storage(&state, "gallery", &filename, data.to_vec()).await?);
            }
            _ => {}
        }
    }

    let meta = metadata.ok_or(ApiError::Validation("Missing metadata field".into()))?;

    // ── Validate all fields ──

    // Name: 1-128 characters
    let name = meta.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::Validation("Name must be 1-128 characters".into()));
    }

    // Description: 1-5000 characters
    let description = meta.description.trim();
    if description.is_empty() || description.len() > 5000 {
        return Err(ApiError::Validation("Description must be 1-5000 characters".into()));
    }

    // Category: must exist in DB
    let cat = renzora_models::category::Category::find_by_slug(&state.db, &meta.category).await?;
    if cat.is_none() {
        return Err(ApiError::Validation(format!("Unknown category: '{}'", meta.category)));
    }

    // Price: non-negative
    if meta.price_credits < 0 {
        return Err(ApiError::Validation("Price cannot be negative".into()));
    }

    // Version: semver-like, 1-32 chars
    let version = meta.version.trim();
    if version.is_empty() || version.len() > 32 {
        return Err(ApiError::Validation("Version must be 1-32 characters".into()));
    }

    // Tags: max 5, each 1-32 chars, alphanumeric + hyphens
    let tags: Vec<String> = meta.tags.iter()
        .take(5)
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty() && t.len() <= 32)
        .collect();

    // Licence: must be valid
    if !renzora_common::types::VALID_LICENCES.contains(&meta.licence.as_str()) {
        return Err(ApiError::Validation(format!(
            "Invalid licence '{}'. Valid options: {}",
            meta.licence,
            renzora_common::types::VALID_LICENCES.join(", ")
        )));
    }

    // Metadata: must be an object, validate known keys
    if !meta.metadata.is_null() && !meta.metadata.is_object() {
        return Err(ApiError::Validation("metadata must be a JSON object".into()));
    }
    if let Some(obj) = meta.metadata.as_object() {
        // Validate texture_resolution if present
        if let Some(res) = obj.get("texture_resolution") {
            if let Some(s) = res.as_str() {
                if !s.is_empty() && !s.contains('x') {
                    return Err(ApiError::Validation("texture_resolution should be in format 'WIDTHxHEIGHT' (e.g. '2048x2048')".into()));
                }
            }
        }
        // Validate render_pipeline if present
        if let Some(rp) = obj.get("render_pipeline") {
            if let Some(s) = rp.as_str() {
                let valid = ["pbr", "unlit", "custom", "forward", "deferred"];
                if !valid.contains(&s) {
                    return Err(ApiError::Validation(format!("render_pipeline must be one of: {}", valid.join(", "))));
                }
            }
        }
        // Validate poly_count if present
        if let Some(pc) = obj.get("poly_count") {
            if let Some(n) = pc.as_i64() {
                if n < 0 {
                    return Err(ApiError::Validation("poly_count cannot be negative".into()));
                }
            }
        }
    }

    // At least one file is required
    if uploaded_files.is_empty() {
        return Err(ApiError::Validation("Asset file is required".into()));
    }

    // ── Create asset ──

    // Auto-populate download_filename from the first uploaded file if not explicitly set
    let download_filename = if meta.download_filename.is_empty() {
        uploaded_files.first().map(|(n, _)| n.clone()).unwrap_or_default()
    } else {
        meta.download_filename.clone()
    };

    let asset = Asset::create_full(
        &state.db,
        auth.user_id,
        name,
        description,
        &meta.category,
        meta.price_credits,
        version,
        &tags,
        &meta.licence,
        meta.ai_generated,
        meta.metadata.clone(),
        &download_filename,
        &meta.subcategory,
        &meta.credit_name,
        &meta.credit_url,
    )
    .await?;

    // ── Process files: multi-file or zip extract ──
    let is_paid = meta.price_credits > 0 && meta.credit_name.is_empty();
    let zip_action = meta.zip_action.as_str();

    // Determine if this is a single zip that should be extracted
    let should_extract = uploaded_files.len() == 1
        && zip_action == "extract"
        && uploaded_files[0].0.to_lowercase().ends_with(".zip");

    let multi_file;
    if should_extract {
        // Extract zip into individual files
        let (_, zip_data) = &uploaded_files[0];
        let extracted = extract_zip_files(zip_data)?;
        multi_file = extracted.len() > 1;

        for (i, (entry_name, entry_data)) in extracted.iter().enumerate() {
            let mime = mime_from_extension(entry_name);
            let file_key = upload_to_storage_private(
                &state,
                &format!("private/assets/{}", asset.id),
                entry_name,
                entry_data.clone(),
            ).await?;

            // Generate preview for paid assets with previewable content
            let preview_key = if is_paid && preview::is_previewable(&mime) {
                generate_preview_key(&state, asset.id, entry_name, entry_data, &mime).await.ok()
            } else {
                None
            };

            AssetFile::insert(
                &state.db, asset.id, &file_key,
                preview_key.as_deref(), entry_name,
                entry_data.len() as i64, &mime, i as i32,
            ).await?;
        }
    } else {
        // Store files as-is (multiple individual files or single zip kept as zip)
        multi_file = uploaded_files.len() > 1;

        for (i, (filename, data)) in uploaded_files.iter().enumerate() {
            let mime = mime_from_extension(filename);
            let file_key = upload_to_storage_private(
                &state,
                &format!("private/assets/{}", asset.id),
                filename,
                data.clone(),
            ).await?;

            let preview_key = if is_paid && preview::is_previewable(&mime) {
                generate_preview_key(&state, asset.id, filename, data, &mime).await.ok()
            } else {
                None
            };

            AssetFile::insert(
                &state.db, asset.id, &file_key,
                preview_key.as_deref(), filename,
                data.len() as i64, &mime, i as i32,
            ).await?;
        }
    }

    // Set multi_file flag
    if multi_file {
        sqlx::query("UPDATE assets SET multi_file = true WHERE id = $1")
            .bind(asset.id)
            .execute(&state.db)
            .await?;
    }

    // For backwards compatibility, set file_url to first file's key
    let first_file = AssetFile::list_by_asset(&state.db, asset.id).await?.into_iter().next();
    if let Some(f) = &first_file {
        Asset::update_file_url(&state.db, asset.id, &f.file_key).await?;
    }

    if let Some(url) = &thumb_path {
        Asset::update_thumbnail_url(&state.db, asset.id, url).await?;
    }

    // Insert gallery media (screenshots, video, audio)
    for (i, url) in screenshots.iter().enumerate() {
        sqlx::query("INSERT INTO asset_media (asset_id, media_type, url, sort_order) VALUES ($1, 'image', $2, $3)")
            .bind(asset.id)
            .bind(url)
            .bind(i as i32)
            .execute(&state.db)
            .await?;
    }
    if let Some(url) = &video_url {
        sqlx::query("INSERT INTO asset_media (asset_id, media_type, url, sort_order) VALUES ($1, 'video', $2, 100)")
            .bind(asset.id)
            .bind(url)
            .execute(&state.db)
            .await?;
    }
    if let Some(url) = &audio_url {
        sqlx::query("INSERT INTO asset_media (asset_id, media_type, url, sort_order) VALUES ($1, 'audio', $2, 200)")
            .bind(asset.id)
            .bind(url)
            .execute(&state.db)
            .await?;
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

fn validate_image_extension(filename: &str) -> Result<(), ApiError> {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "gif" => Ok(()),
        _ => Err(ApiError::Validation(format!("Invalid image format '.{ext}'. Allowed: png, jpg, jpeg, webp, gif"))),
    }
}

fn validate_video_extension(filename: &str) -> Result<(), ApiError> {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "mp4" | "webm" | "mov" => Ok(()),
        _ => Err(ApiError::Validation(format!("Invalid video format '.{ext}'. Allowed: mp4, webm, mov"))),
    }
}

fn validate_audio_extension(filename: &str) -> Result<(), ApiError> {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "mp3" | "wav" | "ogg" | "flac" => Ok(()),
        _ => Err(ApiError::Validation(format!("Invalid audio format '.{ext}'. Allowed: mp3, wav, ogg, flac"))),
    }
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

    // Validate tags if provided
    if let Some(tags) = &body.tags {
        if tags.len() > 5 {
            return Err(ApiError::Validation("Maximum 5 tags".into()));
        }
    }

    // Validate licence if provided
    if let Some(licence) = &body.licence {
        if !renzora_common::types::VALID_LICENCES.contains(&licence.as_str()) {
            return Err(ApiError::Validation(format!("Invalid licence '{licence}'")));
        }
    }

    Asset::update_metadata(
        &state.db,
        id,
        body.name.as_deref(),
        body.description.as_deref(),
        body.price_credits,
        body.version.as_deref(),
        body.published,
    )
    .await?;

    // Update extended fields
    let tags_cleaned: Option<Vec<String>> = body.tags.as_ref().map(|t|
        t.iter().take(5).map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect()
    );
    Asset::update_extended(
        &state.db,
        id,
        tags_cleaned.as_deref(),
        body.licence.as_deref(),
        body.ai_generated,
        body.metadata.clone(),
        body.download_filename.as_deref(),
        body.subcategory.as_deref(),
        body.credit_name.as_deref(),
        body.credit_url.as_deref(),
    ).await?;

    // Re-fetch to get all updated fields
    let updated = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::Internal("Asset not found".into()))?;

    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    Ok(Json(asset_to_detail(&updated, &creator, Some(true))))
}

/// Update asset file(s) and/or thumbnail (multipart).
///
/// Replaces all existing asset files with the new upload. Supports multiple `file` fields
/// and a `zip_action` field ("keep" or "extract").
async fn update_asset_files(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<AssetDetail>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let mut new_files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut zip_action = "keep".to_string();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::Validation(format!("Failed to read multipart field: {e}"))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "file" => {
                let filename = field.file_name().unwrap_or("asset.zip").to_string();
                let data = field.bytes().await
                    .map_err(|e| ApiError::Validation(format!("Failed to read file: {e}")))?;
                if data.len() > 200 * 1024 * 1024 {
                    return Err(ApiError::Validation("File exceeds 200MB limit".into()));
                }
                new_files.push((filename, data.to_vec()));
            }
            "zip_action" => {
                let val = field.text().await.unwrap_or_default();
                if val == "extract" { zip_action = val; }
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                let data = field.bytes().await
                    .map_err(|e| ApiError::Validation(format!("Failed to read thumbnail: {e}")))?;
                let url = upload_to_storage(&state, "thumbnails", &filename, data.to_vec()).await?;
                Asset::update_thumbnail_url(&state.db, id, &url).await?;
            }
            _ => {}
        }
    }

    // If new files were uploaded, replace existing asset_files
    if !new_files.is_empty() {
        // Delete old files from storage
        let old_files = AssetFile::delete_by_asset(&state.db, id).await?;
        for af in &old_files {
            delete_from_storage_by_key(&state, &af.file_key).await;
            if let Some(pk) = &af.preview_key {
                delete_from_storage(&state, pk).await?;
            }
        }

        let is_paid = asset.price_credits > 0 && asset.credit_name.is_empty();
        let should_extract = new_files.len() == 1
            && zip_action == "extract"
            && new_files[0].0.to_lowercase().ends_with(".zip");

        let entries: Vec<(String, Vec<u8>)> = if should_extract {
            extract_zip_files(&new_files[0].1)?
        } else {
            new_files
        };

        let multi_file = entries.len() > 1;

        for (i, (filename, data)) in entries.iter().enumerate() {
            let mime = mime_from_extension(filename);
            let file_key = upload_to_storage_private(
                &state,
                &format!("private/assets/{}", id),
                filename,
                data.clone(),
            ).await?;

            let preview_key = if is_paid && preview::is_previewable(&mime) {
                generate_preview_key(&state, id, filename, data, &mime).await.ok()
            } else {
                None
            };

            AssetFile::insert(
                &state.db, id, &file_key,
                preview_key.as_deref(), filename,
                data.len() as i64, &mime, i as i32,
            ).await?;
        }

        // Update backwards-compat fields
        let first = AssetFile::list_by_asset(&state.db, id).await?.into_iter().next();
        if let Some(f) = &first {
            Asset::update_file_url(&state.db, id, &f.file_key).await?;
        }
        sqlx::query("UPDATE assets SET multi_file = $1 WHERE id = $2")
            .bind(multi_file)
            .bind(id)
            .execute(&state.db)
            .await?;
    }

    let updated = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::Internal("Asset not found after update".into()))?;
    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    Ok(Json(asset_to_detail(&updated, &creator, Some(true))))
}

/// Download an asset (requires auth + ownership or free).
///
/// Returns presigned download URLs for all files.
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

    // Increment download counter
    Asset::increment_downloads(&state.db, id).await?;

    // Check for multi-file asset
    let asset_files = AssetFile::list_by_asset(&state.db, id).await?;

    if !asset_files.is_empty() {
        // Multi-file: return presigned URLs for all files
        let file_infos = build_file_infos(&state, &asset_files, true).await;

        // Primary download URL = first file
        let first = asset_files.first().unwrap();
        let primary_url = generate_presigned_url(&state, &first.file_key).await?;
        let filename = if !asset.download_filename.is_empty() {
            asset.download_filename.clone()
        } else {
            first.original_filename.clone()
        };

        Ok(Json(DownloadResponse {
            download_url: primary_url,
            download_filename: filename,
            files: file_infos,
        }))
    } else {
        // Legacy single-file: use file_url directly
        let file_url = asset
            .file_url
            .ok_or(ApiError::Internal("Asset has no file".into()))?;

        let download_url = if file_url.starts_with("private/") {
            generate_presigned_url(&state, &file_url).await?
        } else {
            // Legacy public URL
            file_url.clone()
        };

        let filename = if !asset.download_filename.is_empty() {
            asset.download_filename.clone()
        } else {
            file_url.rsplit('/').next().unwrap_or("asset").to_string()
        };

        Ok(Json(DownloadResponse {
            download_url,
            download_filename: filename,
            files: vec![],
        }))
    }
}

/// Delete an asset and all its associated files from storage.
/// Only the asset creator or an admin can delete.
async fn delete_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Only creator or admin can delete
    if asset.creator_id != auth.user_id {
        let user = User::find_by_id(&state.db, auth.user_id)
            .await?
            .ok_or(ApiError::Unauthorized)?;
        if user.role != "admin" {
            return Err(ApiError::Unauthorized);
        }
    }

    // Delete asset_files from storage (private keys)
    let deleted_files = AssetFile::delete_by_asset(&state.db, id).await?;
    for af in &deleted_files {
        delete_from_storage_by_key(&state, &af.file_key).await;
        if let Some(pk) = &af.preview_key {
            delete_from_storage(&state, pk).await?;
        }
    }

    // Delete legacy file from storage
    if let Some(url) = &asset.file_url {
        // Could be a private key or a public URL
        if url.starts_with("private/") {
            delete_from_storage_by_key(&state, url).await;
        } else {
            delete_from_storage(&state, url).await?;
        }
    }
    if let Some(url) = &asset.thumbnail_url {
        delete_from_storage(&state, url).await?;
    }

    // Delete associated media files from storage
    let media_rows = sqlx::query_as::<_, (String, Option<String>)>(
        "SELECT url, thumbnail_url FROM asset_media WHERE asset_id = $1"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    for (url, thumb_url) in &media_rows {
        delete_from_storage(&state, url).await?;
        if let Some(thumb) = thumb_url {
            delete_from_storage(&state, thumb).await?;
        }
    }

    // Delete DB records (cascading: media, reviews, comments, purchases)
    sqlx::query("DELETE FROM asset_media WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM reviews WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM asset_comments WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM user_assets WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM transactions WHERE asset_id = $1").bind(id).execute(&state.db).await?;
    sqlx::query("DELETE FROM assets WHERE id = $1").bind(id).execute(&state.db).await?;

    Ok(Json(serde_json::json!({ "message": "Asset deleted", "id": id.to_string() })))
}

/// Proxy an asset's file for the live preview (avoids CORS issues with CDN).
/// For paid assets without ownership, serves the preview (watermarked) version.
/// For free assets or public files, serves the original.
async fn preview_file_proxy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::response::Response, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    // For preview proxy, try to serve from asset_files first
    let asset_files = AssetFile::list_by_asset(&state.db, id).await?;

    let (fetch_url, content_type_hint) = if let Some(first) = asset_files.first() {
        if asset.price_credits > 0 {
            // Paid asset: serve preview if available, otherwise deny
            if let Some(pk) = &first.preview_key {
                (format!("{}/{}", state.s3_public_url, pk), first.mime_type.clone())
            } else {
                return Err(ApiError::Unauthorized);
            }
        } else {
            // Free asset: generate a presigned URL to fetch from
            let url = generate_presigned_url(&state, &first.file_key).await?;
            (url, first.mime_type.clone())
        }
    } else {
        // Legacy: use file_url
        let file_url = asset.file_url.ok_or(ApiError::NotFound)?;
        if file_url.starts_with("private/") {
            let url = generate_presigned_url(&state, &file_url).await?;
            (url, "application/octet-stream".to_string())
        } else {
            (file_url, "application/octet-stream".to_string())
        }
    };

    let client = reqwest::Client::new();
    let resp = client.get(&fetch_url).send().await
        .map_err(|e| ApiError::Internal(format!("Failed to fetch file: {e}")))?;

    let content_type = resp.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or(&content_type_hint)
        .to_string();

    let bytes = resp.bytes().await
        .map_err(|e| ApiError::Internal(format!("Failed to read file: {e}")))?;

    Ok(axum::response::Response::builder()
        .header("content-type", content_type)
        .header("cache-control", "public, max-age=3600")
        .body(axum::body::Body::from(bytes))
        .unwrap())
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
                views: a.views,
                creator_name: a.creator_name,
                creator_avatar_url: a.creator_avatar_url,
                rating_avg,
                rating_count: a.rating_count,
                tags: a.tags,
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

    // Free assets: anyone can comment. Paid assets: must own it or be the creator.
    if asset.price_credits > 0 {
        let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
        if !owns && asset.creator_id != auth.user_id {
            return Err(ApiError::Validation("You must own this asset to comment".into()));
        }
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

    let asset = Asset::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Can't review your own asset
    if asset.creator_id == auth.user_id {
        return Err(ApiError::Validation("You cannot review your own asset".into()));
    }

    // Free assets: anyone can review. Paid assets: must own it.
    if asset.price_credits > 0 {
        let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
        if !owns {
            return Err(ApiError::Validation("You must own this asset to review it".into()));
        }
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

// ── Tags ──

#[derive(Deserialize)]
struct TagQuery {
    q: Option<String>,
}

/// Search/list approved tags (autocomplete).
async fn search_tags(
    State(state): State<AppState>,
    Query(params): Query<TagQuery>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let tags = if let Some(q) = &params.q {
        if q.is_empty() {
            Tag::list_approved(&state.db).await?
        } else {
            Tag::search(&state.db, q, 20).await?
        }
    } else {
        Tag::list_approved(&state.db).await?
    };

    let result: Vec<serde_json::Value> = tags
        .iter()
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "name": t.name,
                "slug": t.slug,
            })
        })
        .collect();

    Ok(Json(result))
}

#[derive(Deserialize)]
struct SubmitTagBody {
    name: String,
}

/// Submit a new tag for review (authenticated).
async fn submit_tag(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SubmitTagBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() || name.len() > 64 {
        return Err(ApiError::Validation(
            "Tag name must be 1-64 characters".into(),
        ));
    }

    let tag = Tag::submit(&state.db, name, auth.user_id).await?;

    Ok(Json(serde_json::json!({
        "id": tag.id,
        "name": tag.name,
        "slug": tag.slug,
        "approved": tag.approved,
    })))
}

// ── Subcategories ──

#[derive(Deserialize)]
struct SubcategoryQuery {
    category: Option<String>,
}

/// List approved subcategories, optionally filtered by category slug.
async fn list_subcategories(
    State(state): State<AppState>,
    Query(params): Query<SubcategoryQuery>,
) -> Result<Json<Vec<serde_json::Value>>, ApiError> {
    let subs = if let Some(cat_slug) = &params.category {
        let cat = Category::find_by_slug(&state.db, cat_slug)
            .await?
            .ok_or(ApiError::NotFound)?;
        Subcategory::list_for_category(&state.db, cat.id).await?
    } else {
        Subcategory::list_all_approved(&state.db).await?
    };

    let result: Vec<serde_json::Value> = subs
        .iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "category_id": s.category_id,
                "name": s.name,
                "slug": s.slug,
            })
        })
        .collect();

    Ok(Json(result))
}

#[derive(Deserialize)]
struct SubmitSubcategoryBody {
    category_slug: String,
    name: String,
}

/// Submit a new subcategory for review (authenticated).
async fn submit_subcategory(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<SubmitSubcategoryBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::Validation(
            "Subcategory name must be 1-128 characters".into(),
        ));
    }

    let cat = Category::find_by_slug(&state.db, &body.category_slug)
        .await?
        .ok_or(ApiError::Validation(format!(
            "Unknown category: '{}'",
            body.category_slug
        )))?;

    let sub = Subcategory::submit(&state.db, cat.id, name, auth.user_id).await?;

    Ok(Json(serde_json::json!({
        "id": sub.id,
        "category_id": sub.category_id,
        "name": sub.name,
        "slug": sub.slug,
        "approved": sub.approved,
    })))
}

/// Generate a clean storage key from a folder and original filename.
/// Returns `folder/uuid.ext` — strips the original name, keeps only the extension.
fn storage_key(folder: &str, original_filename: &str) -> String {
    let ext = original_filename
        .rsplit('.')
        .next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    if ext.is_empty() {
        format!("{}/{}", folder, Uuid::new_v4())
    } else {
        format!("{}/{}.{}", folder, Uuid::new_v4(), ext)
    }
}

/// Detect content type from a file extension in the key.
fn content_type_for_key(key: &str) -> &'static str {
    match key.rsplit('.').next().map(|e| e.to_lowercase()).as_deref() {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("wav") => "audio/wav",
        Some("ogg") => "audio/ogg",
        Some("flac") => "audio/flac",
        Some("aac") => "audio/aac",
        Some("zip") => "application/zip",
        _ => "application/octet-stream",
    }
}

/// Upload a file to S3 (DigitalOcean Spaces) or fall back to local disk.
///
/// Pass the folder (e.g. "thumbnails") and original filename (e.g. "Screenshot 2026.png").
/// A clean key like `thumbnails/a04ea8f5-...-.ext` is generated automatically.
pub async fn upload_to_storage(
    state: &AppState,
    folder: &str,
    original_filename: &str,
    data: Vec<u8>,
) -> Result<String, ApiError> {
    let key = storage_key(folder, original_filename);
    let content_type = content_type_for_key(&key);

    if let Some(bucket) = &state.s3_bucket {
        // Upload to S3-compatible storage (Cloudflare R2)
        let response = bucket
            .put_object_with_content_type(&key, &data, content_type)
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

/// Delete a file from S3 or local disk given its public URL.
pub async fn delete_from_storage(state: &AppState, url: &str) -> Result<(), ApiError> {
    // Extract the S3 key from the public URL
    // e.g. "https://assets.renzora.com/assets/uuid.wgsl" -> "assets/uuid.wgsl"
    let key = if url.starts_with(&state.s3_public_url) {
        url.strip_prefix(&state.s3_public_url)
            .unwrap_or(url)
            .trim_start_matches('/')
            .to_string()
    } else if url.starts_with(&state.upload_base_url) {
        url.strip_prefix(&state.upload_base_url)
            .unwrap_or(url)
            .trim_start_matches('/')
            .to_string()
    } else {
        return Ok(()); // Unknown URL format, skip
    };

    if key.is_empty() {
        return Ok(());
    }

    if let Some(bucket) = &state.s3_bucket {
        let _ = bucket.delete_object(&key).await; // Best effort
    } else {
        let path = format!("{}/{}", state.upload_dir, key);
        let _ = tokio::fs::remove_file(&path).await; // Best effort
    }

    Ok(())
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
                if val == "video" || val == "image" || val == "audio" { media_type = val; }
            }
            "file" => {
                let filename = field.file_name().unwrap_or("media.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                file_url = Some(upload_to_storage(&state, "gallery", &filename, data.to_vec()).await?);
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                thumb_url = Some(upload_to_storage(&state, "gallery/thumbs", &filename, data.to_vec()).await?);
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
        views: asset.views,
        published: asset.published,
        rating_sum: asset.rating_sum,
        rating_count: asset.rating_count,
        tags: asset.tags.clone(),
        licence: asset.licence.clone(),
        ai_generated: asset.ai_generated,
        metadata: asset.metadata.clone(),
        download_filename: asset.download_filename.clone(),
        subcategory: asset.subcategory.clone(),
        credit_name: asset.credit_name.clone(),
        credit_url: asset.credit_url.clone(),
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
        files: vec![],
    }
}

// ── Asset files endpoints ──

/// List asset files (public). Returns preview URLs for unowned paid assets,
/// download URLs for owned/free assets. Auth is optional.
async fn list_asset_files(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<Vec<AssetFileInfo>>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let user_id = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| crate::jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub);

    let has_access = if asset.price_credits == 0 {
        true
    } else {
        match user_id {
            Some(uid) if uid == asset.creator_id => true,
            Some(uid) => asset::user_owns_asset(&state.db, uid, id).await?,
            None => false,
        }
    };

    let files = AssetFile::list_by_asset(&state.db, id).await?;
    let infos = build_file_infos(&state, &files, has_access).await;
    Ok(Json(infos))
}

/// Download a single file from a multi-file asset (auth + ownership).
async fn download_single_file(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((asset_id, file_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DownloadResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, asset_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if asset.price_credits > 0 {
        let owns = asset::user_owns_asset(&state.db, auth.user_id, asset_id).await?;
        if !owns && asset.creator_id != auth.user_id {
            return Err(ApiError::Unauthorized);
        }
    }

    let file = AssetFile::find_by_id(&state.db, file_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if file.asset_id != asset_id {
        return Err(ApiError::NotFound);
    }

    let download_url = generate_presigned_url(&state, &file.file_key).await?;

    Ok(Json(DownloadResponse {
        download_url,
        download_filename: file.original_filename,
        files: vec![],
    }))
}

/// Download all files as a zip (auth + ownership). Streams files from S3 into an in-memory zip.
async fn download_all_zip(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<axum::response::Response, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if asset.price_credits > 0 {
        let owns = asset::user_owns_asset(&state.db, auth.user_id, id).await?;
        if !owns && asset.creator_id != auth.user_id {
            return Err(ApiError::Unauthorized);
        }
    }

    let files = AssetFile::list_by_asset(&state.db, id).await?;
    if files.is_empty() {
        return Err(ApiError::NotFound);
    }

    // If there's only one file and it's a zip, just redirect to it
    if files.len() == 1 && files[0].mime_type == "application/zip" {
        let url = generate_presigned_url(&state, &files[0].file_key).await?;
        return Ok(axum::response::Response::builder()
            .status(302)
            .header("location", url)
            .body(axum::body::Body::empty())
            .unwrap());
    }

    // Build zip in memory from all files
    let mut zip_buf = Vec::new();
    {
        let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buf));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for file in &files {
            let file_bytes = fetch_file_from_storage(&state, &file.file_key).await?;
            zip_writer
                .start_file(&file.original_filename, options)
                .map_err(|e| ApiError::Internal(format!("Zip write failed: {e}")))?;
            std::io::Write::write_all(&mut zip_writer, &file_bytes)
                .map_err(|e| ApiError::Internal(format!("Zip write failed: {e}")))?;
        }

        zip_writer
            .finish()
            .map_err(|e| ApiError::Internal(format!("Zip finalize failed: {e}")))?;
    }

    Asset::increment_downloads(&state.db, id).await?;

    let filename = if !asset.download_filename.is_empty() {
        format!("{}.zip", asset.download_filename.trim_end_matches(".zip"))
    } else {
        format!("{}.zip", asset.slug)
    };

    Ok(axum::response::Response::builder()
        .header("content-type", "application/zip")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(zip_buf))
        .unwrap())
}

// ── Private storage and presigned URL helpers ──

/// Upload a file to private S3 storage. Returns the bare S3 key (not a public URL).
async fn upload_to_storage_private(
    state: &AppState,
    folder: &str,
    original_filename: &str,
    data: Vec<u8>,
) -> Result<String, ApiError> {
    let key = storage_key(folder, original_filename);
    let content_type = content_type_for_key(&key);

    if let Some(bucket) = &state.s3_bucket {
        let response = bucket
            .put_object_with_content_type(&key, &data, content_type)
            .await
            .map_err(|e| ApiError::Internal(format!("S3 upload failed: {e}")))?;

        if response.status_code() != 200 {
            return Err(ApiError::Internal(format!(
                "S3 upload returned status {}",
                response.status_code()
            )));
        }

        Ok(key) // Return bare key, not public URL
    } else {
        // Local fallback
        let path = format!("{}/{}", state.upload_dir, key);
        if let Some(parent) = std::path::Path::new(&path).parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| ApiError::Internal(format!("Failed to create dir: {e}")))?;
        }
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| ApiError::Internal(format!("Failed to write file: {e}")))?;
        Ok(key) // Return key for local too, presigned fallback will use it
    }
}

/// Generate a presigned download URL for a private S3 key (5-minute expiry).
async fn generate_presigned_url(state: &AppState, key: &str) -> Result<String, ApiError> {
    if let Some(bucket) = &state.s3_bucket {
        bucket
            .presign_get(key, 300, None)
            .await
            .map_err(|e| ApiError::Internal(format!("Presign failed: {e}")))
    } else {
        // Local fallback: serve via local URL
        Ok(format!("{}/{}", state.upload_base_url, key))
    }
}

/// Fetch file bytes from S3 by key (for zip generation, preview proxy, etc.)
async fn fetch_file_from_storage(state: &AppState, key: &str) -> Result<Vec<u8>, ApiError> {
    if let Some(bucket) = &state.s3_bucket {
        let resp = bucket
            .get_object(key)
            .await
            .map_err(|e| ApiError::Internal(format!("S3 get failed: {e}")))?;
        Ok(resp.to_vec())
    } else {
        let path = format!("{}/{}", state.upload_dir, key);
        tokio::fs::read(&path)
            .await
            .map_err(|e| ApiError::Internal(format!("Failed to read local file: {e}")))
    }
}

/// Delete a file from S3 by its bare key (not a URL). Best effort.
async fn delete_from_storage_by_key(state: &AppState, key: &str) {
    if key.is_empty() {
        return;
    }
    if let Some(bucket) = &state.s3_bucket {
        let _ = bucket.delete_object(key).await;
    } else {
        let path = format!("{}/{}", state.upload_dir, key);
        let _ = tokio::fs::remove_file(&path).await;
    }
}

/// Build `AssetFileInfo` list with appropriate preview/download URLs.
async fn build_file_infos(
    state: &AppState,
    files: &[AssetFile],
    has_access: bool,
) -> Vec<AssetFileInfo> {
    let mut infos = Vec::with_capacity(files.len());
    for f in files {
        let download_url = if has_access {
            generate_presigned_url(state, &f.file_key).await.ok()
        } else {
            None
        };

        let preview_url = if !has_access {
            f.preview_key
                .as_ref()
                .map(|pk| format!("{}/{}", state.s3_public_url, pk))
        } else {
            None
        };

        infos.push(AssetFileInfo {
            id: f.id,
            original_filename: f.original_filename.clone(),
            file_size: f.file_size,
            mime_type: f.mime_type.clone(),
            sort_order: f.sort_order,
            preview_url,
            download_url,
        });
    }
    infos
}

/// Generate a preview for a file and upload it to public storage.
/// Returns the public preview key on success.
async fn generate_preview_key(
    state: &AppState,
    asset_id: Uuid,
    filename: &str,
    data: &[u8],
    mime: &str,
) -> Result<String, ApiError> {
    let preview_data = if mime.starts_with("image/") {
        preview::generate_image_preview(data)?
    } else if mime.starts_with("audio/") {
        let ext = filename.rsplit('.').next().unwrap_or("mp3");
        preview::generate_audio_preview(data, ext).await?
    } else {
        return Err(ApiError::Internal("Not previewable".into()));
    };

    let preview_ext = if mime.starts_with("audio/") { "mp3" } else { "jpg" };
    let preview_filename = format!("preview_{}.{}", Uuid::new_v4(), preview_ext);

    // Upload to public previews path
    upload_to_storage(
        state,
        &format!("public/previews/{}", asset_id),
        &preview_filename,
        preview_data,
    )
    .await
}

/// Extract files from a zip archive in memory with safety checks.
fn extract_zip_files(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>, ApiError> {
    use std::io::Read;

    let reader = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| ApiError::Validation(format!("Invalid zip file: {e}")))?;

    if archive.len() > 100 {
        return Err(ApiError::Validation(
            "Zip contains too many entries (max 100)".into(),
        ));
    }

    let mut files = Vec::new();
    let mut total_size: u64 = 0;
    let max_total: u64 = 500 * 1024 * 1024; // 500MB
    let max_single: u64 = 200 * 1024 * 1024; // 200MB

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| ApiError::Validation(format!("Failed to read zip entry: {e}")))?;

        // Skip directories
        if entry.is_dir() {
            continue;
        }

        let name = entry.name().to_string();

        // Path traversal check
        if name.contains("..") || name.starts_with('/') || name.starts_with('\\') {
            continue;
        }

        // Skip nested zips
        if name.to_lowercase().ends_with(".zip") {
            continue;
        }

        // Skip macOS resource fork files
        if name.contains("__MACOSX") || name.starts_with('.') {
            continue;
        }

        // Size check
        let size = entry.size();
        if size > max_single {
            return Err(ApiError::Validation(format!(
                "File '{}' exceeds 200MB limit",
                name
            )));
        }
        total_size += size;
        if total_size > max_total {
            return Err(ApiError::Validation(
                "Total uncompressed size exceeds 500MB limit".into(),
            ));
        }

        let mut buf = Vec::with_capacity(size as usize);
        entry
            .read_to_end(&mut buf)
            .map_err(|e| ApiError::Validation(format!("Failed to extract '{}': {e}", name)))?;

        // Use just the filename (strip directory paths from zip)
        let clean_name = name.rsplit('/').next().unwrap_or(&name).to_string();
        if clean_name.is_empty() {
            continue;
        }

        files.push((clean_name, buf));
    }

    if files.is_empty() {
        return Err(ApiError::Validation("Zip contains no extractable files".into()));
    }

    Ok(files)
}

/// Derive MIME type from file extension.
fn mime_from_extension(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "zip" => "application/zip",
        "wgsl" => "text/plain",
        "glb" | "gltf" => "model/gltf-binary",
        "fbx" => "application/octet-stream",
        "obj" => "model/obj",
        "lua" => "text/x-lua",
        "rhai" => "text/plain",
        "json" => "application/json",
        "ron" => "text/plain",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// Extract the real client IP, checking common proxy headers first.
pub fn client_ip(
    headers: &axum::http::HeaderMap,
    connect_info: &axum::extract::ConnectInfo<std::net::SocketAddr>,
) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| connect_info.0.ip().to_string())
}

/// Hash an IP address for privacy (we don't need to store raw IPs).
pub fn hash_ip(ip: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    format!("{:x}", hasher.finalize())
}
