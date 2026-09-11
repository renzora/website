use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use renzora_common::types::*;
use renzora_models::asset::{self, Asset};
use renzora_models::asset_file::AssetFile;
use renzora_models::asset_release::AssetRelease;
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
        .route("/:id/releases", post(create_release))
        .route("/:id/releases/:release_id", put(update_release).delete(delete_release))
        .route("/:id/comments", post(add_comment))
        .route("/comments/:comment_id", delete(delete_comment))
        .route("/:id/reviews", post(submit_review))
        .route("/:id/media", post(upload_media))
        .route("/media/:media_id", delete(delete_media))
        .route("/:id/reviews/flag", post(flag_review))
        .route("/:id/reviews/helpful", post(mark_review_helpful))
        .route("/:id/delete", delete(delete_asset))
        .route("/tags/submit", post(submit_tag))
        .route("/subcategories/submit", post(submit_subcategory))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    // Downloads authenticate optionally: a free asset is downloadable by
    // anyone, a paid one still needs a signed-in owner (see `authorize_download`).
    let downloads = Router::new()
        .route("/:id/download", get(download_asset))
        .route("/:id/files/:file_id/download", get(download_single_file))
        .route("/:id/download-zip", get(download_all_zip))
        .layer(axum::middleware::from_fn(middleware::optional_auth));

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
        .route("/:id/releases", get(list_releases))
        .route("/:id/tree", get(asset_tree))
        .route("/:id/file", get(view_file))
        .route("/:id/raw", get(raw_file))
        .route("/:id/preview-file", get(preview_file_proxy))
        .route("/plugin-updates", post(plugin_updates))
        .merge(downloads)
        .merge(protected)
}

/// Browse/search marketplace assets.
async fn list_assets(
    State(state): State<AppState>,
    Query(params): Query<MarketplaceQuery>,
) -> Result<Json<MarketplaceListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page: i64 = 100;
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

    // A plugin ships as one zip of buildable source: the editor extracts it
    // straight into `plugins/` and the SDK compiles it there. Check that here
    // rather than letting a buyer discover it at build time, and record the
    // crate name the editor needs to name the directory.
    let mut meta_extra = meta.metadata.clone();
    if is_plugin_category(&meta.category) {
        if uploaded_files.len() != 1 {
            return Err(ApiError::Validation(
                "A plugin must be a single .zip of its source".into(),
            ));
        }
        let (fname, data) = &uploaded_files[0];
        if !fname.to_lowercase().ends_with(".zip") {
            return Err(ApiError::Validation(
                "A plugin must be uploaded as a .zip of its source".into(),
            ));
        }
        let crate_name = plugin_crate_name(data)?;
        // The editor reads this to name the folder it extracts into, and it is
        // the dll stem the loader looks for afterwards.
        if let Some(obj) = meta_extra.as_object_mut() {
            obj.insert("crate_name".into(), crate_name.into());
        } else {
            meta_extra = serde_json::json!({ "crate_name": crate_name });
        }
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
        meta_extra.clone(),
        &download_filename,
        &meta.subcategory,
        &meta.credit_name,
        &meta.credit_url,
    )
    .await?;

    // ── Process files: multi-file or zip extract ──
    let is_paid = meta.price_credits > 0 && meta.credit_name.is_empty();

    // Every asset starts with one release. Files hang off it rather than off
    // the asset, so a later version can be published without destroying this
    // one for the people who already bought it.
    let release = AssetRelease::create_current(&state.db, asset.id, &asset.version, "").await?;

    // A plugin's zip is the deliverable, not a container to unpack: the editor
    // extracts the whole source tree into `plugins/<crate>/` and builds it, so
    // the bytes have to stay exactly as uploaded. Its contents are still
    // indexed for browsing — see `store_release_files`.
    let stored_action = effective_zip_action(&meta.zip_action, &meta.category);
    let entries = archive_entries(uploaded_files, stored_action)?;
    let stored = store_release_files(&state, asset.id, release.id, is_paid, entries).await?;
    let multi_file = stored.multi_file;

    // A CHANGELOG.md in the archive seeds the release notes, the same way the
    // README becomes the asset's documentation.
    if let Some(notes) = &stored.changelog {
        let _ = AssetRelease::update_notes(&state.db, release.id, notes).await;
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

    // Auto-publish the asset
    Asset::update_metadata(&state.db, asset.id, None, None, None, None, Some(true)).await?;

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

    // Award XP for uploading
    let _ = renzora_models::xp::award_xp(&state.db, auth.user_id, renzora_models::xp::XP_UPLOAD_ASSET, "upload_asset", Some(asset.id)).await;

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

    // Editing the version renames the current release rather than letting the
    // asset and its release drift apart.
    if let Some(new_version) = body.version.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
        if new_version != asset.version {
            if let Some(current) = AssetRelease::find_current(&state.db, id).await? {
                let clash = AssetRelease::find_by_version(&state.db, id, new_version)
                    .await?
                    .is_some_and(|r| r.id != current.id);
                if clash {
                    return Err(ApiError::Validation(format!(
                        "Version '{new_version}' already belongs to another release of this asset"
                    )));
                }
                AssetRelease::update_version(&state.db, current.id, new_version).await?;
            }
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

    // New files here *replace the current release in place* — this is the edit
    // path, for fixing a bad upload. Shipping a new version is a release
    // (`POST /:id/releases`), which keeps the old files downloadable.
    if !new_files.is_empty() {
        let release = match AssetRelease::find_current(&state.db, id).await? {
            Some(r) => r,
            // An asset from before the release system, or one that never got
            // files, gets its first release now.
            None => AssetRelease::create_current(&state.db, id, &asset.version, "").await?,
        };

        // Drop the current release's files (and their storage), leaving every
        // other release untouched.
        let old_files = AssetFile::delete_by_release(&state.db, release.id).await?;
        for af in &old_files {
            delete_from_storage_by_key(&state, &af.file_key).await;
            if let Some(pk) = &af.preview_key {
                delete_from_storage(&state, pk).await?;
            }
        }

        let is_paid = asset.price_credits > 0 && asset.credit_name.is_empty();

        // Same rule as upload: a plugin is one zip of source, kept whole. Its
        // crate name is re-read here because a rename is a legitimate update
        // and the editor keys the install directory off it.
        if is_plugin_category(&asset.category) {
            if new_files.len() != 1 || !new_files[0].0.to_lowercase().ends_with(".zip") {
                return Err(ApiError::Validation(
                    "A plugin must be a single .zip of its source".into(),
                ));
            }
            let crate_name = plugin_crate_name(&new_files[0].1)?;
            let mut meta = asset.metadata.clone();
            match meta.as_object_mut() {
                Some(obj) => {
                    obj.insert("crate_name".into(), crate_name.into());
                }
                None => meta = serde_json::json!({ "crate_name": crate_name }),
            }
            sqlx::query("UPDATE assets SET metadata = $1, updated_at = NOW() WHERE id = $2")
                .bind(&meta)
                .bind(id)
                .execute(&state.db)
                .await?;
        }

        let stored_action = effective_zip_action(&zip_action, &asset.category);
        let entries = archive_entries(new_files, stored_action)?;
        store_release_files(&state, id, release.id, is_paid, entries).await?;
    }

    let updated = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::Internal("Asset not found after update".into()))?;
    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::Internal("Creator not found".into()))?;

    Ok(Json(asset_to_detail(&updated, &creator, Some(true))))
}

/// Gate a download. Free published assets are open to everyone, signed in or
/// not; a paid asset requires a signed-in user who owns it. The creator can
/// always fetch their own asset, which is also the only way to reach one that
/// is still unpublished — anonymous downloads must not expose drafts.
fn authorize_download(asset: &Asset, auth: &Option<AuthUser>, owns: bool) -> Result<(), ApiError> {
    let is_creator = auth.as_ref().is_some_and(|u| asset.creator_id == u.user_id);
    if is_creator {
        return Ok(());
    }
    if !asset.published {
        return Err(ApiError::NotFound);
    }
    if asset.price_credits == 0 {
        return Ok(());
    }
    if owns {
        Ok(())
    } else {
        Err(ApiError::Unauthorized)
    }
}

/// Download an asset. Free published assets need no account; paid ones need
/// a signed-in owner.
///
/// Returns presigned download URLs for all files. `?release=` picks an older
/// version — buyers keep access to every release they paid for, not just the
/// newest one.
async fn download_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Path(id): Path<Uuid>,
    Query(params): Query<ReleaseQuery>,
) -> Result<Json<DownloadResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let owns = match &auth {
        Some(u) => asset::user_owns_asset(&state.db, u.user_id, id).await?,
        None => false,
    };
    authorize_download(&asset, &auth, owns)?;

    // Increment download counter
    Asset::increment_downloads(&state.db, id).await?;

    // Files of the requested release (the current one by default).
    let release = AssetRelease::resolve(&state.db, id, params.release.as_deref()).await?;
    let asset_files = match &release {
        Some(r) => {
            AssetRelease::increment_downloads(&state.db, r.id).await?;
            AssetFile::list_by_release(&state.db, r.id).await?
        }
        None => AssetFile::list_by_asset(&state.db, id).await?,
    };

    if !asset_files.is_empty() {
        // Multi-file: return presigned URLs for all files
        let file_infos = build_file_infos(&state, &asset_files, true).await;

        // Primary download URL = first file
        let first = asset_files.first().unwrap();
        let primary_url = generate_presigned_url(&state, &first.file_key).await?;
        // The file's own name, NOT `asset.download_filename`.
        //
        // That column records what the SELLER uploaded, which for a zip the
        // server then extracted is an archive that no longer exists — the rows
        // are its contents. Serving the first row's bytes under the archive's
        // name produced downloads like `banner_ui_02.zip` holding an FBX, which
        // no extractor can open and which the editor refused to unpack because
        // the extension said one thing and the bytes another.
        //
        // For a genuinely single-file upload the two agree, so nothing changes
        // there; the legacy branch below still uses the asset-level name,
        // because without rows it is the only name there is.
        let filename = first.original_filename.clone();

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
    sqlx::query("DELETE FROM asset_reviews WHERE asset_id = $1").bind(id).execute(&state.db).await?;
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

    // The upstream's status, checked before its body is handed on.
    //
    // Without this the proxy answered 200 with whatever the storage host
    // returned — and for a missing or unreachable object that is an HTML error
    // page, served to the caller as if it were the asset. The editor's theme
    // preview then fed `<!DOCTYPE html>` to a TOML parser and reported a syntax
    // error at line 1, which describes the symptom and hides the cause: the
    // client had checked its own response status, and that status was 200.
    //
    // A failed fetch is the proxy's failure, not the caller's, so it surfaces as
    // one rather than as a file the caller cannot read.
    if !resp.status().is_success() {
        return Err(ApiError::Internal(format!(
            "Preview file unavailable: storage returned HTTP {}",
            resp.status().as_u16()
        )));
    }

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

    // Award XP for reviewing
    let _ = renzora_models::xp::award_xp(&state.db, auth.user_id, renzora_models::xp::XP_REVIEW, "review", Some(id)).await;
    // Award seller XP to asset creator for receiving a review
    if body.rating >= 4 {
        let _ = renzora_models::xp::award_seller_xp(&state.db, asset.creator_id, renzora_models::xp::SELLER_XP_REVIEW, "review_received", Some(id)).await;
    }

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
    Extension(auth): Extension<Option<AuthUser>>,
    Path((asset_id, file_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DownloadResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, asset_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let owns = match &auth {
        Some(u) => asset::user_owns_asset(&state.db, u.user_id, asset_id).await?,
        None => false,
    };
    authorize_download(&asset, &auth, owns)?;

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
    Extension(auth): Extension<Option<AuthUser>>,
    Path(id): Path<Uuid>,
    Query(params): Query<ReleaseQuery>,
) -> Result<axum::response::Response, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let owns = match &auth {
        Some(u) => asset::user_owns_asset(&state.db, u.user_id, id).await?,
        None => false,
    };
    authorize_download(&asset, &auth, owns)?;

    let release = AssetRelease::resolve(&state.db, id, params.release.as_deref()).await?;
    let files = match &release {
        Some(r) => AssetFile::list_by_release(&state.db, r.id).await?,
        None => AssetFile::list_by_asset(&state.db, id).await?,
    };
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
            // Write the archive path, not just the filename, so the download
            // unpacks with the same structure the creator uploaded.
            zip_writer
                .start_file(&file.path, options)
                .map_err(|e| ApiError::Internal(format!("Zip write failed: {e}")))?;
            std::io::Write::write_all(&mut zip_writer, &file_bytes)
                .map_err(|e| ApiError::Internal(format!("Zip write failed: {e}")))?;
        }

        zip_writer
            .finish()
            .map_err(|e| ApiError::Internal(format!("Zip finalize failed: {e}")))?;
    }

    Asset::increment_downloads(&state.db, id).await?;

    let stem = if !asset.download_filename.is_empty() {
        asset.download_filename.trim_end_matches(".zip").to_string()
    } else {
        asset.slug.clone()
    };
    // Name an older download after the version in it, so two releases don't
    // land in the downloads folder with the same name.
    let filename = match &release {
        Some(r) if !r.is_current => format!("{stem}-{}.zip", r.version),
        _ => format!("{stem}.zip"),
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

#[derive(Deserialize)]
struct PluginUpdatesRequest {
    /// Asset ids of the plugins the caller has installed.
    ids: Vec<Uuid>,
}

/// What the editor needs to decide whether an installed plugin should update.
#[derive(Serialize)]
struct PluginUpdate {
    id: Uuid,
    slug: String,
    name: String,
    /// The version currently published.
    version: String,
    /// Minimum engine release, from the asset's metadata. Empty means "any" —
    /// the editor treats it that way rather than guessing a floor.
    min_engine_version: String,
    /// False once a creator unpublishes: the editor keeps what it has rather
    /// than offering an update it cannot fetch.
    published: bool,
}

/// Latest published version for a set of installed plugins, in one request.
///
/// A round trip per plugin would be a burst of requests every time the editor
/// starts, for something that is almost always "no change". Public: the version
/// and the engine floor are on the listing already, and an update check should
/// not require being signed in.
async fn plugin_updates(
    State(state): State<AppState>,
    Json(body): Json<PluginUpdatesRequest>,
) -> Result<Json<Vec<PluginUpdate>>, ApiError> {
    if body.ids.is_empty() {
        return Ok(Json(Vec::new()));
    }
    if body.ids.len() > 200 {
        return Err(ApiError::Validation("Too many ids (max 200)".into()));
    }

    let rows = sqlx::query(
        "SELECT id, slug, name, version, published, metadata FROM assets WHERE id = ANY($1)",
    )
    .bind(&body.ids)
    .fetch_all(&state.db)
    .await?;

    let updates = rows
        .iter()
        .map(|r| {
            let metadata: serde_json::Value = r.get("metadata");
            let min_engine_version = metadata
                .get("min_engine_version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            PluginUpdate {
                id: r.get("id"),
                slug: r.get("slug"),
                name: r.get("name"),
                version: r.get("version"),
                min_engine_version,
                published: r.get("published"),
            }
        })
        .collect();
    Ok(Json(updates))
}

/// Categories whose uploads are buildable plugin source.
fn is_plugin_category(slug: &str) -> bool {
    matches!(slug, "plugins" | "plugin")
}

/// Read the `[package] name` out of a plugin zip's `Cargo.toml`.
///
/// Accepts the manifest at the archive root or one directory down, which is the
/// difference between zipping the crate's contents and zipping its folder —
/// both are things people do, and neither is wrong.
fn plugin_crate_name(data: &[u8]) -> Result<String, ApiError> {
    use std::io::Read;

    let reader = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| ApiError::Validation(format!("Invalid zip file: {e}")))?;

    let mut manifest: Option<String> = None;
    for i in 0..archive.len() {
        let mut f = archive
            .by_index(i)
            .map_err(|e| ApiError::Validation(format!("Invalid zip entry: {e}")))?;
        let name = f.name().to_string();
        // Depth 0 or 1 only: a Cargo.toml further down belongs to a vendored
        // dependency or a workspace member, not to the plugin itself.
        let depth = name.trim_end_matches('/').matches('/').count();
        if !name.ends_with("Cargo.toml") || depth > 1 {
            continue;
        }
        let mut text = String::new();
        if f.read_to_string(&mut text).is_err() {
            return Err(ApiError::Validation("Cargo.toml is not valid UTF-8".into()));
        }
        // Prefer the shallowest one.
        if depth == 0 {
            manifest = Some(text);
            break;
        }
        manifest.get_or_insert(text);
    }

    let Some(text) = manifest else {
        return Err(ApiError::Validation(
            "Plugin zip has no Cargo.toml at its root — upload the plugin's source, \
             either the crate folder or its contents"
                .into(),
        ));
    };

    let parsed: toml::Table = text
        .parse()
        .map_err(|e| ApiError::Validation(format!("Cargo.toml is not valid TOML: {e}")))?;
    let name = parsed
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| ApiError::Validation("Cargo.toml has no [package] name".into()))?;

    // The name becomes a directory and a dll stem, so keep it to what cargo
    // itself allows.
    if name.is_empty()
        || name.len() > 64
        || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(ApiError::Validation(format!(
            "Invalid crate name '{name}' in Cargo.toml"
        )));
    }
    Ok(name.to_string())
}

/// Extract a zip into `(archive path, bytes)` pairs, **keeping the directory
/// structure** — that tree is what the marketplace file browser renders and
/// what lets a `README.md` link to `docs/install.md`.
///
/// A single wrapping top-level folder is stripped, because that's what almost
/// every archive has (`my-plugin-1.2.0/src/...`) and nobody wants to click
/// through it on every visit.
fn extract_zip_files(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>, ApiError> {
    use std::io::Read;

    let reader = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| ApiError::Validation(format!("Invalid zip file: {e}")))?;

    if archive.len() > MAX_ARCHIVE_ENTRIES {
        return Err(ApiError::Validation(format!(
            "Zip contains too many entries (max {MAX_ARCHIVE_ENTRIES})"
        )));
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

        // Skip nested zips
        if name.to_lowercase().ends_with(".zip") {
            continue;
        }

        // macOS resource forks and metadata are noise in a file tree.
        if name.contains("__MACOSX") || name.contains(".DS_Store") {
            continue;
        }

        // Normalise separators and reject anything that tries to climb out of
        // the archive (`../`), is absolute, or hides in a dotfile directory.
        let Some(path) = sanitize_archive_path(&name) else {
            continue;
        };

        // Size check
        let size = entry.size();
        if size > max_single {
            return Err(ApiError::Validation(format!(
                "File '{}' exceeds 200MB limit",
                path
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
            .map_err(|e| ApiError::Validation(format!("Failed to extract '{}': {e}", path)))?;

        files.push((path, buf));
    }

    if files.is_empty() {
        return Err(ApiError::Validation("Zip contains no extractable files".into()));
    }

    strip_common_root(&mut files);
    Ok(files)
}

/// Maximum number of entries accepted from one uploaded archive. Generous
/// enough for a plugin with sources and a `docs/` folder, bounded so a zip
/// bomb can't fan out into unbounded rows.
const MAX_ARCHIVE_ENTRIES: usize = 500;

/// Normalise a path out of an archive, or reject it.
///
/// Rejects absolute paths, `..` traversal, Windows drive letters and UNC
/// paths, and dot-directories/dotfiles (`.git/`, `.env`), which are never
/// content anyone meant to publish.
pub(crate) fn sanitize_archive_path(raw: &str) -> Option<String> {
    let normalized = raw.replace('\\', "/");

    // `C:/...` or `//server/share`
    if normalized.starts_with('/') || normalized.starts_with("//") {
        return None;
    }
    if normalized.len() >= 2 && normalized.as_bytes()[1] == b':' {
        return None;
    }

    let mut parts: Vec<&str> = Vec::new();
    for seg in normalized.split('/') {
        match seg {
            "" | "." => continue,
            ".." => return None,
            s if s.starts_with('.') => return None,
            s => parts.push(s),
        }
    }

    if parts.is_empty() {
        return None;
    }
    let path = parts.join("/");
    // Postgres TEXT is unbounded but a path this long is a red flag, and the
    // tree UI can't render it usefully either.
    (path.len() <= 1024).then_some(path)
}

/// If every entry sits under the same top-level directory, drop it.
pub(crate) fn strip_common_root(files: &mut [(String, Vec<u8>)]) {
    let Some(first_root) = files
        .first()
        .and_then(|(p, _)| p.split_once('/'))
        .map(|(root, _)| root.to_string())
    else {
        return; // a file lives at the root, so there's no wrapper to strip
    };

    let shared = files
        .iter()
        .all(|(p, _)| p.split_once('/').is_some_and(|(root, _)| root == first_root));
    if !shared {
        return;
    }

    let cut = first_root.len() + 1;
    for (p, _) in files.iter_mut() {
        *p = p[cut..].to_string();
    }
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

// ══════════════════════════════════════════════════════════════════════════
//  Releases, file tree and README/docs
//
//  An asset's uploaded archive is browsable like a repository: the *tree* is
//  public (filenames, sizes, structure), while file *contents* need ownership
//  — except documentation, which is the point of publishing it. Markdown and
//  licence files are readable by anyone, so a README can sell the plugin.
// ══════════════════════════════════════════════════════════════════════════

/// Text files larger than this are shown truncated rather than in full.
const MAX_TEXT_VIEW_BYTES: usize = 512 * 1024;

/// Docs are readable without owning the asset: markdown (the documentation
/// system) and licence files (which buyers need to read *before* buying).
pub(crate) fn is_public_doc(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".md") || lower.ends_with(".markdown") {
        return true;
    }
    let stem = lower.split('.').next().unwrap_or(&lower);
    matches!(stem, "license" | "licence" | "copying" | "notice" | "authors")
}

/// Is this the archive's README? The rank is its depth, so the one closest to
/// the root wins when a package vendors READMEs of its own.
fn readme_rank(path: &str) -> Option<usize> {
    let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    let is_readme = matches!(
        name.as_str(),
        "readme.md" | "readme.markdown" | "readme" | "readme.txt"
    );
    is_readme.then(|| path.matches('/').count())
}

fn is_changelog(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    matches!(
        name.as_str(),
        "changelog.md" | "changelog" | "changelog.txt" | "changes.md" | "history.md"
    )
}

/// highlight.js language hint for a source file.
fn language_for(path: &str) -> Option<&'static str> {
    let lower = path.to_ascii_lowercase();
    let name = lower.rsplit('/').next().unwrap_or(&lower);
    match name {
        "dockerfile" => return Some("dockerfile"),
        "makefile" => return Some("makefile"),
        _ => {}
    }
    Some(match name.rsplit('.').next()? {
        "rs" => "rust",
        "lua" => "lua",
        // Rhai is Rust-shaped, and reads far better highlighted as Rust than
        // as nothing at all.
        "rhai" => "rust",
        "wgsl" | "glsl" | "vert" | "frag" => "glsl",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "tsx" | "jsx" => "typescript",
        "py" => "python",
        "sh" | "bash" | "zsh" => "bash",
        "json" => "json",
        "toml" | "ron" | "ini" | "cfg" | "conf" => "ini",
        "yaml" | "yml" => "yaml",
        "xml" | "svg" | "html" | "htm" => "xml",
        "css" => "css",
        "sql" => "sql",
        "c" | "h" => "c",
        "cpp" | "cc" | "hpp" => "cpp",
        "cs" => "csharp",
        "go" => "go",
        "java" => "java",
        "kt" => "kotlin",
        "rb" => "ruby",
        "swift" => "swift",
        "md" | "markdown" => "markdown",
        "txt" | "log" => "plaintext",
        _ => return None,
    })
}

/// Whether a file's bytes are plain text we can show in the source viewer.
fn is_text_file(path: &str, mime: &str) -> bool {
    mime.starts_with("text/") || language_for(path).is_some()
}

/// How an upload should be stored, given the category.
///
/// A plugin is always kept as the single zip it arrived as: the editor
/// extracts that archive into `plugins/<crate>/` and the SDK builds it, so the
/// bytes have to survive the round trip untouched. Its contents are still
/// indexed for the file browser — see `store_release_files`.
fn effective_zip_action<'a>(requested: &'a str, category: &str) -> &'a str {
    if is_plugin_category(category) {
        "keep"
    } else {
        requested
    }
}

/// Turn a multipart upload into the archive entries to store: either the
/// unpacked contents of a single zip, or the uploaded files as they came.
fn archive_entries(
    uploaded: Vec<(String, Vec<u8>)>,
    zip_action: &str,
) -> Result<Vec<(String, Vec<u8>)>, ApiError> {
    if uploaded.is_empty() {
        return Ok(Vec::new());
    }
    let should_extract = uploaded.len() == 1
        && zip_action == "extract"
        && uploaded[0].0.to_lowercase().ends_with(".zip");

    if should_extract {
        return extract_zip_files(&uploaded[0].1);
    }

    // Not extracted: each upload is a root-level file. Strip any directory
    // component the browser sent along with the name.
    Ok(uploaded
        .into_iter()
        .map(|(name, data)| {
            let clean = name
                .replace('\\', "/")
                .rsplit('/')
                .next()
                .unwrap_or("file")
                .to_string();
            (clean, data)
        })
        .filter(|(name, _)| !name.is_empty())
        .collect())
}

/// What `store_release_files` found while writing an archive out.
struct StoredRelease {
    multi_file: bool,
    /// Contents of a CHANGELOG in the archive, if there was one.
    changelog: Option<String>,
}

/// Write an archive's entries out as the files of `release_id`.
///
/// Doc files get their text cached on the row so the README renders from the
/// database; everything else lives only in object storage.
async fn store_release_files(
    state: &AppState,
    asset_id: Uuid,
    release_id: Uuid,
    is_paid: bool,
    entries: Vec<(String, Vec<u8>)>,
) -> Result<StoredRelease, ApiError> {
    let multi_file = entries.len() > 1;
    let mut changelog = None;

    for (i, (path, data)) in entries.iter().enumerate() {
        let filename = path.rsplit('/').next().unwrap_or(path);
        let mime = mime_from_extension(filename);

        let file_key = upload_to_storage_private(
            state,
            &format!("private/assets/{asset_id}"),
            filename,
            data.clone(),
        )
        .await?;

        // Generate preview for paid assets with previewable content
        let preview_key = if is_paid && preview::is_previewable(&mime) {
            generate_preview_key(state, asset_id, filename, data, &mime)
                .await
                .ok()
        } else {
            None
        };

        // Cache the text of public docs so reading them never touches storage.
        let text = if is_public_doc(path) && data.len() <= MAX_TEXT_VIEW_BYTES {
            String::from_utf8(data.clone()).ok()
        } else {
            None
        };

        if changelog.is_none() && is_changelog(path) {
            changelog = String::from_utf8(data.clone()).ok();
        }

        AssetFile::insert(
            &state.db,
            asset_id,
            Some(release_id),
            &file_key,
            preview_key.as_deref(),
            filename,
            path,
            data.len() as i64,
            &mime,
            i as i32,
            false,
            text.as_deref(),
        )
        .await?;

        // A zip kept whole is still worth reading. Index what's inside it so
        // the asset gets a file tree and its README renders, without touching
        // the stored bytes — which for a plugin the editor has to build from.
        if mime == "application/zip" {
            index_archive_contents(state, asset_id, release_id, &file_key, data).await;
        }
    }

    // Keep the legacy single-file pointer and the multi-file flag in step with
    // whatever the current release now holds.
    if let Some(f) = AssetFile::list_by_asset(&state.db, asset_id)
        .await?
        .into_iter()
        .next()
    {
        Asset::update_file_url(&state.db, asset_id, &f.file_key).await?;
    }
    sqlx::query("UPDATE assets SET multi_file = $1 WHERE id = $2")
        .bind(multi_file)
        .bind(asset_id)
        .execute(&state.db)
        .await?;

    Ok(StoredRelease {
        multi_file,
        changelog,
    })
}

/// Index the entries of a stored archive so they can be browsed and their
/// docs rendered, without unpacking it into separate objects.
///
/// Indexed rows carry `archived = true` and point their `file_key` at the
/// containing zip; `read_archived_bytes` pulls one entry back out. Best
/// effort — an archive we can't read just doesn't get a tree.
async fn index_archive_contents(
    state: &AppState,
    asset_id: Uuid,
    release_id: Uuid,
    container_key: &str,
    data: &[u8],
) {
    let entries = match extract_zip_files(data) {
        Ok(e) => e,
        Err(e) => {
            tracing::debug!("not indexing archive for asset {asset_id}: {e}");
            return;
        }
    };

    for (i, (path, bytes)) in entries.iter().enumerate() {
        let filename = path.rsplit('/').next().unwrap_or(path);
        let text = if is_public_doc(path) && bytes.len() <= MAX_TEXT_VIEW_BYTES {
            String::from_utf8(bytes.clone()).ok()
        } else {
            None
        };

        if let Err(e) = AssetFile::insert_indexed(
            &state.db,
            asset_id,
            release_id,
            container_key,
            filename,
            path,
            bytes.len() as i64,
            &mime_from_extension(filename),
            i as i32,
            text.as_deref(),
        )
        .await
        {
            tracing::warn!("could not index {path} for asset {asset_id}: {e}");
        }
    }
}

/// Largest archive we'll pull back out of storage just to index it on a read.
///
/// Indexing happens once per release and is then cached, but this runs on a
/// public endpoint, so an unbounded fetch is not something to leave open.
const MAX_LAZY_INDEX_BYTES: i64 = 64 * 1024 * 1024;

/// Index a release's archive if that hasn't happened yet.
///
/// Assets uploaded before the file browser existed have their zip in storage
/// and nothing indexed — migration 053 could give them a release and a path,
/// but not reach into object storage to read the archive. So the first person
/// to open the file browser triggers it, and everyone after that gets the
/// cached tree. Returns whether anything was indexed.
///
/// Best effort throughout: an archive we can't fetch or read just keeps
/// showing as a single zip, which is what it did before.
async fn ensure_release_indexed(
    state: &AppState,
    asset_id: Uuid,
    release_id: Uuid,
    deliverables: &[AssetFile],
) -> bool {
    // Only archives are worth indexing, and only once.
    let Some(zip) = deliverables.iter().find(|f| {
        f.mime_type == "application/zip" || f.path.to_ascii_lowercase().ends_with(".zip")
    }) else {
        return false;
    };
    if zip.file_size > MAX_LAZY_INDEX_BYTES {
        tracing::debug!(
            "not indexing {} for asset {asset_id}: {} bytes is over the lazy-index limit",
            zip.path,
            zip.file_size
        );
        return false;
    }
    match AssetFile::has_indexed(&state.db, release_id).await {
        Ok(true) => return false,
        Ok(false) => {}
        Err(e) => {
            tracing::warn!("could not check the index for release {release_id}: {e}");
            return false;
        }
    }

    let bytes = match fetch_file_from_storage(state, &zip.file_key).await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!("could not fetch {} to index it: {e}", zip.file_key);
            return false;
        }
    };
    index_archive_contents(state, asset_id, release_id, &zip.file_key, &bytes).await;
    true
}

/// Find a browsable file, indexing the release's archive first if a deep link
/// arrives before anyone has opened the file browser.
async fn find_browsable_file(
    state: &AppState,
    asset_id: Uuid,
    release_id: Uuid,
    path: &str,
) -> Result<AssetFile, ApiError> {
    if let Some(f) = AssetFile::find_by_path(&state.db, release_id, path).await? {
        return Ok(f);
    }
    let deliverables = AssetFile::list_by_release(&state.db, release_id).await?;
    if ensure_release_indexed(state, asset_id, release_id, &deliverables).await {
        if let Some(f) = AssetFile::find_by_path(&state.db, release_id, path).await? {
            return Ok(f);
        }
    }
    Err(ApiError::NotFound)
}

/// Pull one entry's bytes out of the archive an indexed row lives in.
async fn read_archived_bytes(state: &AppState, file: &AssetFile) -> Result<Vec<u8>, ApiError> {
    use std::io::Read;

    let zip_bytes = fetch_file_from_storage(state, &file.file_key).await?;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes))
        .map_err(|e| ApiError::Internal(format!("Could not open the stored archive: {e}")))?;

    // Entry names are matched through the same normalisation the index used,
    // so a stripped wrapper directory still resolves.
    let mut names: Vec<String> = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        names.push(archive.by_index(i).map(|f| f.name().to_string()).unwrap_or_default());
    }
    let mut normalized: Vec<(String, Vec<u8>)> = names
        .iter()
        .filter_map(|n| sanitize_archive_path(n).map(|p| (p, Vec::new())))
        .collect();
    strip_common_root(&mut normalized);

    let idx = normalized
        .iter()
        .position(|(p, _)| *p == file.path)
        .ok_or(ApiError::NotFound)?;
    // `normalized` skips entries the sanitiser rejected, so map back by name.
    let raw_index = names
        .iter()
        .enumerate()
        .filter(|(_, n)| sanitize_archive_path(n).is_some())
        .nth(idx)
        .map(|(i, _)| i)
        .ok_or(ApiError::NotFound)?;

    let mut entry = archive
        .by_index(raw_index)
        .map_err(|e| ApiError::Internal(format!("Could not read the archive entry: {e}")))?;
    let mut buf = Vec::with_capacity(entry.size() as usize);
    entry
        .read_to_end(&mut buf)
        .map_err(|e| ApiError::Internal(format!("Could not extract the archive entry: {e}")))?;
    Ok(buf)
}

/// A file's bytes, whether it is a stored object or an entry inside one.
async fn file_bytes(state: &AppState, file: &AssetFile) -> Result<Vec<u8>, ApiError> {
    if file.archived {
        read_archived_bytes(state, file).await
    } else {
        fetch_file_from_storage(state, &file.file_key).await
    }
}

/// The authenticated user, if the request carries a valid access token. Used
/// by the public endpoints, which behave differently for owners.
fn optional_user(
    headers: &axum::http::HeaderMap,
    jwt_secret: &crate::middleware::JwtSecret,
) -> Option<Uuid> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| crate::jwt::validate_token(token, &jwt_secret.0).ok())
        .filter(|c| c.token_type == "access")
        .map(|c| c.sub)
}

/// May this user read the asset's file *contents*? Free assets are open, paid
/// ones need a purchase (the creator always counts as an owner).
async fn has_file_access(
    state: &AppState,
    asset: &Asset,
    user_id: Option<Uuid>,
) -> Result<bool, ApiError> {
    if asset.price_credits == 0 {
        return Ok(true);
    }
    Ok(match user_id {
        Some(uid) if uid == asset.creator_id => true,
        Some(uid) => asset::user_owns_asset(&state.db, uid, asset.id).await?,
        None => false,
    })
}

#[derive(Deserialize)]
pub struct ReleaseQuery {
    /// A release id or version string. Absent means the current release.
    pub release: Option<String>,
}

#[derive(Deserialize)]
pub struct FileQuery {
    pub path: String,
    pub release: Option<String>,
}

/// A `?release=` suffix that pins generated links to a non-current release.
fn release_query_for(release: &AssetRelease) -> String {
    if release.is_current {
        String::new()
    } else {
        format!("?release={}", urlencode_component(&release.version))
    }
}

fn urlencode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn to_doc_headings(md: &str) -> Vec<DocHeading> {
    crate::markdown::outline(md)
        .into_iter()
        .map(|(level, text, anchor)| DocHeading {
            level,
            text,
            anchor,
        })
        .collect()
}

fn release_info(
    r: &AssetRelease,
    ctx: &crate::markdown::MdContext,
    file_count: i64,
    total_size: i64,
) -> ReleaseInfo {
    ReleaseInfo {
        id: r.id,
        version: r.version.clone(),
        notes: r.notes.clone(),
        notes_html: if r.notes.trim().is_empty() {
            String::new()
        } else {
            crate::markdown::render(&r.notes, ctx)
        },
        is_current: r.is_current,
        downloads: r.downloads,
        file_count,
        total_size,
        // RFC 3339, so the browser can parse it directly. (`Display` for
        // OffsetDateTime emits a single-digit hour before 10:00, which
        // `new Date()` chokes on.)
        created_at: r
            .created_at
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| r.created_at.to_string()),
    }
}

/// Read a doc file's text, using the cached copy when there is one and
/// backfilling it from storage when there isn't.
async fn doc_text(state: &AppState, file: &AssetFile) -> Option<String> {
    if let Some(t) = &file.text_content {
        return Some(t.clone());
    }
    if file.file_size as usize > MAX_TEXT_VIEW_BYTES {
        return None;
    }
    let bytes = file_bytes(state, file).await.ok()?;
    let text = String::from_utf8(bytes).ok()?;
    // Best effort: a failed backfill just means we fetch again next time.
    let _ = AssetFile::cache_text(&state.db, file.id, &text).await;
    Some(text)
}

/// `GET /api/marketplace/:id/releases` — version history, newest first.
async fn list_releases(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ReleaseInfo>>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let rows = AssetRelease::list_by_asset(&state.db, id).await?;

    let infos = rows
        .into_iter()
        .map(|r| {
            let release = AssetRelease {
                id: r.id,
                asset_id: r.asset_id,
                version: r.version,
                notes: r.notes,
                is_current: r.is_current,
                downloads: r.downloads,
                created_at: r.created_at,
            };
            let ctx = crate::markdown::MdContext::new(
                id,
                &asset.slug,
                "CHANGELOG.md",
                &release_query_for(&release),
            );
            release_info(&release, &ctx, r.file_count, r.total_size)
        })
        .collect();

    Ok(Json(infos))
}

/// `GET /api/marketplace/:id/tree` — the whole file tree of one release, plus
/// its rendered README. Public: the structure is browsable by anyone.
async fn asset_tree(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<ReleaseQuery>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<AssetTreeResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let release = AssetRelease::resolve(&state.db, id, params.release.as_deref())
        .await?
        .ok_or(ApiError::NotFound)?;

    let user_id = optional_user(&headers, &jwt_secret);
    let has_access = has_file_access(&state, &asset, user_id).await?;

    let mut files = AssetFile::list_release_tree(&state.db, release.id).await?;
    // An asset uploaded before the file browser existed has only its zip; index
    // it now so this view, and every later one, shows the real tree.
    if ensure_release_indexed(&state, id, release.id, &files).await {
        files = AssetFile::list_release_tree(&state.db, release.id).await?;
    }
    let entries = build_tree(&files);

    // The README nearest the root becomes the asset's front page.
    let readme_file = files
        .iter()
        .filter_map(|f| readme_rank(&f.path).map(|d| (d, f)))
        .min_by_key(|(d, _)| *d)
        .map(|(_, f)| f);

    let readme = match readme_file {
        Some(f) => doc_text(&state, f).await.map(|md| {
            let ctx = crate::markdown::MdContext::new(
                id,
                &asset.slug,
                &f.path,
                &release_query_for(&release),
            );
            RenderedDoc {
                path: f.path.clone(),
                html: crate::markdown::render(&md, &ctx),
                outline: to_doc_headings(&md),
            }
        }),
        None => None,
    };

    let file_count = files.len() as i64;
    let total_size = files.iter().map(|f| f.file_size).sum();
    let notes_ctx = crate::markdown::MdContext::new(
        id,
        &asset.slug,
        "CHANGELOG.md",
        &release_query_for(&release),
    );

    Ok(Json(AssetTreeResponse {
        release: release_info(&release, &notes_ctx, file_count, total_size),
        entries,
        readme,
        has_access,
    }))
}

/// Flatten a release's files into tree nodes: every file, plus a synthesised
/// directory for each path prefix, sized by what it contains.
fn build_tree(files: &[AssetFile]) -> Vec<AssetTreeEntry> {
    use std::collections::BTreeMap;

    let mut dirs: BTreeMap<String, i64> = BTreeMap::new();
    let mut entries: Vec<AssetTreeEntry> = Vec::with_capacity(files.len());

    for f in files {
        // Every ancestor directory accumulates this file's size.
        let segments: Vec<&str> = f.path.split('/').collect();
        let mut prefix = String::new();
        for seg in &segments[..segments.len().saturating_sub(1)] {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(seg);
            *dirs.entry(prefix.clone()).or_insert(0) += f.file_size;
        }

        entries.push(AssetTreeEntry {
            id: Some(f.id),
            path: f.path.clone(),
            name: segments.last().copied().unwrap_or(f.path.as_str()).to_string(),
            kind: "file".into(),
            size: f.file_size,
            mime_type: f.mime_type.clone(),
            is_doc: is_public_doc(&f.path),
        });
    }

    for (path, size) in dirs {
        let name = path.rsplit('/').next().unwrap_or(&path).to_string();
        entries.push(AssetTreeEntry {
            id: None,
            path,
            name,
            kind: "dir".into(),
            size,
            mime_type: "inode/directory".into(),
            is_doc: false,
        });
    }

    // Shallowest first, directories before files, alphabetical within each —
    // the ordering every file browser uses.
    entries.sort_by(|a, b| {
        let depth = |e: &AssetTreeEntry| e.path.matches('/').count();
        let dirs_first = |e: &AssetTreeEntry| u8::from(e.kind != "dir");
        depth(a)
            .cmp(&depth(b))
            .then_with(|| dirs_first(a).cmp(&dirs_first(b)))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    entries
}

/// `GET /api/marketplace/:id/file?path=…` — open one file in the viewer.
///
/// Markdown and licence files render for anyone. Everything else needs
/// ownership, and comes back as `kind: "locked"` when the caller lacks it.
async fn view_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<FileQuery>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<Json<AssetFileView>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let release = AssetRelease::resolve(&state.db, id, params.release.as_deref())
        .await?
        .ok_or(ApiError::NotFound)?;
    let file = find_browsable_file(&state, id, release.id, &params.path).await?;

    let user_id = optional_user(&headers, &jwt_secret);
    let has_access = has_file_access(&state, &asset, user_id).await?;
    let name = file
        .path
        .rsplit('/')
        .next()
        .unwrap_or(&file.path)
        .to_string();

    let base = AssetFileView {
        path: file.path.clone(),
        name,
        kind: "binary".into(),
        mime_type: file.mime_type.clone(),
        size: file.file_size,
        html: None,
        content: None,
        language: language_for(&file.path).map(str::to_string),
        outline: Vec::new(),
        download_url: None,
        truncated: false,
    };

    // Documentation is public — that's the whole point of shipping a README.
    if is_public_doc(&file.path) {
        if let Some(md) = doc_text(&state, &file).await {
            let ctx = crate::markdown::MdContext::new(
                id,
                &asset.slug,
                &file.path,
                &release_query_for(&release),
            );
            return Ok(Json(AssetFileView {
                kind: "markdown".into(),
                outline: to_doc_headings(&md),
                html: Some(crate::markdown::render(&md, &ctx)),
                content: Some(md),
                ..base
            }));
        }
        return Ok(Json(base));
    }

    if !has_access {
        return Ok(Json(AssetFileView {
            kind: "locked".into(),
            ..base
        }));
    }

    if is_text_file(&file.path, &file.mime_type) && (file.file_size as usize) <= MAX_TEXT_VIEW_BYTES
    {
        let bytes = file_bytes(&state, &file).await?;
        let truncated = bytes.len() > MAX_TEXT_VIEW_BYTES;
        let slice = &bytes[..bytes.len().min(MAX_TEXT_VIEW_BYTES)];
        if let Ok(text) = String::from_utf8(slice.to_vec()) {
            return Ok(Json(AssetFileView {
                kind: "text".into(),
                content: Some(text),
                truncated,
                ..base
            }));
        }
    }

    // An indexed entry has no object of its own to presign, so it is served
    // through our raw endpoint, which extracts it from the containing archive.
    let download_url = if file.archived {
        format!(
            "/api/marketplace/{}/raw?path={}{}",
            id,
            urlencode_component(&file.path),
            if release.is_current {
                String::new()
            } else {
                format!("&release={}", urlencode_component(&release.version))
            }
        )
    } else {
        generate_presigned_url(&state, &file.file_key).await?
    };
    Ok(Json(AssetFileView {
        kind: if file.mime_type.starts_with("image/") {
            "image".into()
        } else {
            "binary".into()
        },
        download_url: Some(download_url),
        ..base
    }))
}

/// `GET /api/marketplace/:id/raw?path=…` — the file's bytes.
///
/// Backs images referenced from a README. Same access rule as the viewer, so
/// an image inside a paid asset 403s for people who haven't bought it.
async fn raw_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<FileQuery>,
    headers: axum::http::HeaderMap,
    Extension(jwt_secret): Extension<crate::middleware::JwtSecret>,
) -> Result<axum::response::Response, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    let release = AssetRelease::resolve(&state.db, id, params.release.as_deref())
        .await?
        .ok_or(ApiError::NotFound)?;
    let file = find_browsable_file(&state, id, release.id, &params.path).await?;

    if !is_public_doc(&file.path) {
        let user_id = optional_user(&headers, &jwt_secret);
        if !has_file_access(&state, &asset, user_id).await? {
            return Err(ApiError::Unauthorized);
        }
    }

    let bytes = file_bytes(&state, &file).await?;
    // Only images are served as themselves; anything else downloads rather
    // than rendering, so an uploaded .html can't become a page on our origin.
    // SVG is included because README logos are usually SVG — the `sandbox`
    // CSP below puts it in an opaque origin with scripting off, which is what
    // makes serving user SVG safe.
    let content_type = if file.mime_type.starts_with("image/") {
        file.mime_type.clone()
    } else {
        "application/octet-stream".to_string()
    };

    Ok(axum::response::Response::builder()
        .header("content-type", content_type)
        // Immutable per release, but access depends on the caller, so this may
        // only ever be cached privately.
        .header("cache-control", "private, max-age=3600")
        .header("x-content-type-options", "nosniff")
        .header("content-security-policy", "default-src 'none'; sandbox")
        .body(axum::body::Body::from(bytes))
        .unwrap())
}

/// `POST /api/marketplace/:id/releases` — publish a new version (multipart).
///
/// The previous release keeps its files, so buyers can still download the
/// version they were already using.
async fn create_release(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<ReleaseInfo>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let mut meta: Option<CreateReleaseRequest> = None;
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::Validation(format!("Failed to read multipart field: {e}")))?
    {
        match field.name().unwrap_or("") {
            "metadata" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read metadata: {e}")))?;
                meta = Some(
                    serde_json::from_str(&text)
                        .map_err(|e| ApiError::Validation(format!("Invalid metadata JSON: {e}")))?,
                );
            }
            "file" => {
                if files.len() >= 20 {
                    return Err(ApiError::Validation("Maximum 20 files per release".into()));
                }
                let filename = field.file_name().unwrap_or("asset.zip").to_string();
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::Validation(format!("Failed to read file: {e}")))?;
                if data.len() > 200 * 1024 * 1024 {
                    return Err(ApiError::Validation("File exceeds 200MB limit".into()));
                }
                files.push((filename, data.to_vec()));
            }
            _ => {}
        }
    }

    let meta = meta.ok_or_else(|| ApiError::Validation("Missing metadata".into()))?;
    let version = meta.version.trim().to_string();
    if version.is_empty() || version.len() > 32 {
        return Err(ApiError::Validation(
            "Version must be 1-32 characters".into(),
        ));
    }
    if files.is_empty() {
        return Err(ApiError::Validation(
            "A release needs at least one file".into(),
        ));
    }
    if AssetRelease::find_by_version(&state.db, id, &version)
        .await?
        .is_some()
    {
        return Err(ApiError::Validation(format!(
            "Version '{version}' already exists for this asset"
        )));
    }

    // A release has to satisfy the same packaging rules as the original
    // upload, or an update could break the editor's install for every buyer.
    if is_plugin_category(&asset.category) {
        if files.len() != 1 || !files[0].0.to_lowercase().ends_with(".zip") {
            return Err(ApiError::Validation(
                "A plugin must be a single .zip of its source".into(),
            ));
        }
        let crate_name = plugin_crate_name(&files[0].1)?;
        let mut asset_meta = asset.metadata.clone();
        match asset_meta.as_object_mut() {
            Some(obj) => {
                obj.insert("crate_name".into(), crate_name.into());
            }
            None => asset_meta = serde_json::json!({ "crate_name": crate_name }),
        }
        sqlx::query("UPDATE assets SET metadata = $1 WHERE id = $2")
            .bind(&asset_meta)
            .bind(id)
            .execute(&state.db)
            .await?;
    }

    let release = AssetRelease::create_current(&state.db, id, &version, meta.notes.trim()).await?;

    let is_paid = asset.price_credits > 0 && asset.credit_name.is_empty();
    let stored_action = effective_zip_action(&meta.zip_action, &asset.category);
    let entries = archive_entries(files, stored_action)?;
    let stored = store_release_files(&state, id, release.id, is_paid, entries).await?;

    // A CHANGELOG in the archive fills in notes the creator didn't write.
    let release = match (&stored.changelog, release.notes.trim().is_empty()) {
        (Some(notes), true) => AssetRelease::update_notes(&state.db, release.id, notes)
            .await?
            .unwrap_or(release),
        _ => release,
    };

    // The asset's headline version follows its current release.
    Asset::update_metadata(&state.db, id, None, None, None, Some(&version), None).await?;

    let files = AssetFile::list_release_tree(&state.db, release.id).await?;
    let ctx = crate::markdown::MdContext::new(
        id,
        &asset.slug,
        "CHANGELOG.md",
        &release_query_for(&release),
    );
    Ok(Json(release_info(
        &release,
        &ctx,
        files.len() as i64,
        files.iter().map(|f| f.file_size).sum(),
    )))
}

/// `PUT /api/marketplace/:id/releases/:release_id` — edit release notes.
async fn update_release(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((id, release_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateReleaseRequest>,
) -> Result<Json<ReleaseInfo>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }
    let release = AssetRelease::find_by_id(&state.db, release_id)
        .await?
        .filter(|r| r.asset_id == id)
        .ok_or(ApiError::NotFound)?;

    let release = match body.notes {
        Some(notes) => AssetRelease::update_notes(&state.db, release.id, notes.trim())
            .await?
            .unwrap_or(release),
        None => release,
    };

    let files = AssetFile::list_release_tree(&state.db, release.id).await?;
    let ctx = crate::markdown::MdContext::new(
        id,
        &asset.slug,
        "CHANGELOG.md",
        &release_query_for(&release),
    );
    Ok(Json(release_info(
        &release,
        &ctx,
        files.len() as i64,
        files.iter().map(|f| f.file_size).sum(),
    )))
}

/// `DELETE /api/marketplace/:id/releases/:release_id` — remove an old release.
///
/// The current release can't be deleted; publish a newer one first. That keeps
/// every asset that has files having something to download.
async fn delete_release(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((id, release_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let asset = Asset::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if asset.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }
    let release = AssetRelease::find_by_id(&state.db, release_id)
        .await?
        .filter(|r| r.asset_id == id)
        .ok_or(ApiError::NotFound)?;

    if release.is_current {
        return Err(ApiError::Validation(
            "Can't delete the current release. Publish a newer version first.".into(),
        ));
    }

    let files = AssetFile::delete_by_release(&state.db, release.id).await?;
    for f in &files {
        delete_from_storage_by_key(&state, &f.file_key).await;
        if let Some(pk) = &f.preview_key {
            let _ = delete_from_storage(&state, pk).await;
        }
    }
    AssetRelease::delete(&state.db, release.id).await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}

#[cfg(test)]
mod plugin_source_tests {
    use super::*;
    use std::io::Write;

    /// Build a zip in memory from `(path, contents)` pairs.
    fn zip_of(entries: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            for (name, body) in entries {
                w.start_file::<_, ()>(*name, zip::write::SimpleFileOptions::default())
                    .unwrap();
                w.write_all(body.as_bytes()).unwrap();
            }
            w.finish().unwrap();
        }
        buf
    }

    const MANIFEST: &str = "[package]\nname = \"renzora_lumen\"\nversion = \"0.1.0\"\n";

    #[test]
    fn reads_the_crate_name_from_a_root_manifest() {
        let zip = zip_of(&[("Cargo.toml", MANIFEST), ("src/lib.rs", "")]);
        assert_eq!(plugin_crate_name(&zip).unwrap(), "renzora_lumen");
    }

    /// Zipping the crate *folder* rather than its contents is just as common,
    /// and equally correct.
    #[test]
    fn reads_the_crate_name_one_directory_down() {
        let zip = zip_of(&[
            ("renzora_lumen/Cargo.toml", MANIFEST),
            ("renzora_lumen/src/lib.rs", ""),
        ]);
        assert_eq!(plugin_crate_name(&zip).unwrap(), "renzora_lumen");
    }

    /// A manifest deeper than that belongs to a vendored dependency or a
    /// workspace member, so it must not be mistaken for the plugin's own.
    #[test]
    fn ignores_a_manifest_nested_too_deep() {
        let zip = zip_of(&[("a/b/c/Cargo.toml", MANIFEST), ("a/b/c/src/lib.rs", "")]);
        assert!(plugin_crate_name(&zip).is_err());
    }

    /// The root manifest wins over a nested one, whatever order they are in.
    #[test]
    fn prefers_the_shallowest_manifest() {
        let nested = "[package]\nname = \"vendored_dep\"\n";
        let zip = zip_of(&[("sub/Cargo.toml", nested), ("Cargo.toml", MANIFEST)]);
        assert_eq!(plugin_crate_name(&zip).unwrap(), "renzora_lumen");
    }

    #[test]
    fn rejects_an_archive_with_no_manifest() {
        let zip = zip_of(&[("readme.txt", "hello"), ("src/lib.rs", "")]);
        let err = plugin_crate_name(&zip).unwrap_err().to_string();
        assert!(err.contains("Cargo.toml"), "unhelpful error: {err}");
    }

    #[test]
    fn rejects_a_manifest_without_a_package_name() {
        let zip = zip_of(&[("Cargo.toml", "[workspace]\nmembers = []\n")]);
        assert!(plugin_crate_name(&zip).is_err());
    }

    #[test]
    fn rejects_malformed_toml() {
        let zip = zip_of(&[("Cargo.toml", "[package\nname =")]);
        assert!(plugin_crate_name(&zip).is_err());
    }

    /// The name becomes a directory and a library filename, so anything that
    /// could escape either is refused.
    #[test]
    fn rejects_a_crate_name_that_is_not_a_safe_path_segment() {
        for bad in ["../evil", "with space", "semi;colon", ""] {
            let manifest = format!("[package]\nname = \"{bad}\"\n");
            let zip = zip_of(&[("Cargo.toml", manifest.as_str())]);
            assert!(plugin_crate_name(&zip).is_err(), "accepted {bad:?}");
        }
    }

    #[test]
    fn rejects_a_zip_that_is_not_a_zip() {
        assert!(plugin_crate_name(b"not a zip at all").is_err());
    }

    #[test]
    fn plugin_categories_are_recognised() {
        assert!(is_plugin_category("plugins"));
        assert!(is_plugin_category("plugin"));
        assert!(!is_plugin_category("scripts"));
        assert!(!is_plugin_category("materials"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str, size: i64) -> AssetFile {
        AssetFile {
            id: Uuid::new_v4(),
            asset_id: Uuid::nil(),
            release_id: None,
            file_key: "private/assets/x/y".into(),
            preview_key: None,
            original_filename: path.rsplit('/').next().unwrap_or(path).into(),
            path: path.into(),
            file_size: size,
            mime_type: mime_from_extension(path),
            sort_order: 0,
            archived: false,
            text_content: None,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
        }
    }

    // ── Archive paths ──────────────────────────────────────────────────────

    #[test]
    fn archive_paths_keep_their_directories() {
        assert_eq!(
            sanitize_archive_path("docs/guide/install.md").as_deref(),
            Some("docs/guide/install.md")
        );
    }

    #[test]
    fn windows_separators_are_normalised() {
        assert_eq!(
            sanitize_archive_path(r"src\plugin\main.lua").as_deref(),
            Some("src/plugin/main.lua")
        );
    }

    #[test]
    fn traversal_and_absolute_paths_are_rejected() {
        for bad in [
            "../../etc/passwd",
            "a/../../b",
            "/etc/passwd",
            "//server/share/x",
            "C:/Windows/system32/evil.dll",
            r"..\..\secrets.txt",
        ] {
            assert!(
                sanitize_archive_path(bad).is_none(),
                "accepted a bad path: {bad}"
            );
        }
    }

    #[test]
    fn dotfiles_and_dot_directories_are_skipped() {
        assert!(sanitize_archive_path(".env").is_none());
        assert!(sanitize_archive_path(".git/config").is_none());
        assert!(sanitize_archive_path("src/.hidden/key.pem").is_none());
    }

    #[test]
    fn redundant_segments_collapse() {
        assert_eq!(
            sanitize_archive_path("./docs//install.md").as_deref(),
            Some("docs/install.md")
        );
    }

    #[test]
    fn a_single_wrapping_folder_is_stripped() {
        let mut files = vec![
            ("my-plugin-1.0/README.md".to_string(), vec![]),
            ("my-plugin-1.0/src/main.lua".to_string(), vec![]),
        ];
        strip_common_root(&mut files);
        assert_eq!(files[0].0, "README.md");
        assert_eq!(files[1].0, "src/main.lua");
    }

    #[test]
    fn differing_roots_are_left_alone() {
        let mut files = vec![
            ("src/main.lua".to_string(), vec![]),
            ("docs/install.md".to_string(), vec![]),
        ];
        strip_common_root(&mut files);
        assert_eq!(files[0].0, "src/main.lua");
        assert_eq!(files[1].0, "docs/install.md");
    }

    #[test]
    fn a_root_level_file_stops_the_strip() {
        let mut files = vec![
            ("README.md".to_string(), vec![]),
            ("my-plugin/src/main.lua".to_string(), vec![]),
        ];
        strip_common_root(&mut files);
        assert_eq!(files[1].0, "my-plugin/src/main.lua");
    }

    // ── What's public ──────────────────────────────────────────────────────

    #[test]
    fn markdown_and_licences_are_public_docs() {
        for p in [
            "README.md",
            "docs/install.markdown",
            "LICENSE",
            "LICENCE.txt",
            "COPYING",
        ] {
            assert!(is_public_doc(p), "should be public: {p}");
        }
    }

    #[test]
    fn source_and_binaries_are_not_public_docs() {
        for p in [
            "src/main.lua",
            "plugin.wasm",
            "textures/diffuse.png",
            "notes.txt",
        ] {
            assert!(!is_public_doc(p), "should not be public: {p}");
        }
    }

    #[test]
    fn the_shallowest_readme_wins() {
        assert_eq!(readme_rank("README.md"), Some(0));
        assert_eq!(readme_rank("vendor/dep/README.md"), Some(2));
        assert_eq!(readme_rank("src/main.lua"), None);
    }

    // ── Tree building ──────────────────────────────────────────────────────

    #[test]
    fn directories_are_synthesised_from_paths() {
        let files = vec![
            file("README.md", 100),
            file("docs/install.md", 200),
            file("docs/api/events.md", 300),
        ];
        let tree = build_tree(&files);

        let dir = |p: &str| {
            tree.iter()
                .find(|e| e.path == p && e.kind == "dir")
                .unwrap_or_else(|| panic!("no dir {p} in {tree:?}"))
                .size
        };
        // A directory's size is everything beneath it, at any depth.
        assert_eq!(dir("docs"), 500);
        assert_eq!(dir("docs/api"), 300);
    }

    #[test]
    fn tree_lists_directories_before_files_at_each_depth() {
        let files = vec![file("a.txt", 1), file("zz/b.txt", 1), file("m.txt", 1)];
        let tree = build_tree(&files);
        let top: Vec<&str> = tree
            .iter()
            .filter(|e| !e.path.contains('/'))
            .map(|e| e.name.as_str())
            .collect();
        assert_eq!(top, vec!["zz", "a.txt", "m.txt"]);
    }

    #[test]
    fn tree_marks_which_entries_are_readable_without_owning() {
        let files = vec![file("README.md", 10), file("src/main.lua", 20)];
        let tree = build_tree(&files);
        let doc = |p: &str| tree.iter().find(|e| e.path == p).unwrap().is_doc;
        assert!(doc("README.md"));
        assert!(!doc("src/main.lua"));
    }

    #[test]
    fn a_flat_upload_still_produces_a_tree() {
        let files = vec![file("model.glb", 42)];
        let tree = build_tree(&files);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].kind, "file");
        assert_eq!(tree[0].name, "model.glb");
    }

    // ── Upload handling ────────────────────────────────────────────────────

    #[test]
    fn loose_uploads_are_flattened_to_the_root() {
        let entries = archive_entries(
            vec![
                (r"C:\Users\me\main.lua".to_string(), vec![1]),
                ("plugin.toml".to_string(), vec![2]),
            ],
            "keep",
        )
        .unwrap();
        let names: Vec<&str> = entries.iter().map(|(p, _)| p.as_str()).collect();
        assert_eq!(names, vec!["main.lua", "plugin.toml"]);
    }

    #[test]
    fn changelog_and_readme_are_recognised() {
        assert!(is_changelog("CHANGELOG.md"));
        assert!(is_changelog("changes.md"));
        assert!(!is_changelog("docs/changelog-format.md"));
    }

    #[test]
    fn known_extensions_get_a_highlight_language() {
        assert_eq!(language_for("src/main.lua"), Some("lua"));
        assert_eq!(language_for("build.rs"), Some("rust"));
        assert_eq!(language_for("shader.wgsl"), Some("glsl"));
        assert_eq!(language_for("Dockerfile"), Some("dockerfile"));
        assert_eq!(language_for("model.glb"), None);
    }

    #[test]
    fn only_non_current_releases_pin_a_release_query() {
        let mut r = AssetRelease {
            id: Uuid::nil(),
            asset_id: Uuid::nil(),
            version: "1.2.0".into(),
            notes: String::new(),
            is_current: true,
            downloads: 0,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
        };
        assert_eq!(release_query_for(&r), "");
        r.is_current = false;
        assert_eq!(release_query_for(&r), "?release=1.2.0");
    }
}
