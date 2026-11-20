//! Prebuilt plugins: binaries the engine's CI compiled for one exact engine
//! build, served to editors of that build so they need neither the SDK nor a
//! Rust toolchain to install a plugin.
//!
//! See `migrations/059_prebuilt_plugins.sql` for the data and
//! `renzora_plugin_build::prebuilt` in the engine for the trust model. In short:
//! CI signs each binary's statement with a key only CI holds, and the editor
//! checks that signature against keys compiled into the engine. This site stores
//! and serves; it checks what it cheaply can (the bytes match the statement, the
//! listing exists and is a plugin) but it is not what makes a binary trusted.
//!
//! Routes, under `/api/prebuilt-plugins`:
//!
//! - `GET /:asset_id?engine=<stamp>&platform=<id>`: the binary for this exact
//!   build, or 404, which tells the editor to install from source.
//! - `GET /:asset_id/latest`: the newest binary per platform, for the listing
//!   page, which cannot know which build its visitor runs.
//! - `GET /:asset_id/file/:id`: the binary's bytes, by redirect, counted.
//! - `POST /upload`: CI's upload, with an admin's API token.

use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::middleware::{self, AuthUser};
use crate::AppState;
use renzora_models::asset::{self, Asset};
use renzora_models::asset_release::AssetRelease;
use renzora_models::prebuilt_plugin::{NewPrebuilt, PrebuiltPlugin};

/// Platforms a binary may be filed under: the engine's `dist/` directory names.
const PLATFORMS: &[&str] = &[
    "windows-x64",
    "windows-arm64",
    "macos-x64",
    "macos-arm64",
    "linux-x64",
    "linux-arm64",
];

pub fn router() -> Router<AppState> {
    let public = Router::new()
        .route("/:asset_id", get(lookup))
        .route("/:asset_id/latest", get(latest))
        .route("/:asset_id/file/:id", get(file))
        .layer(axum::middleware::from_fn(middleware::optional_auth));

    let upload = Router::new()
        .route("/upload", post(upload))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new().merge(upload).merge(public)
}

#[derive(Deserialize)]
struct LookupQuery {
    engine: String,
    platform: String,
}

/// What an editor needs to fetch and check one binary. The statement fields
/// are returned exactly as CI signed them, so the editor can rebuild the signed
/// text without this site being able to alter it unnoticed.
#[derive(Serialize)]
struct PrebuiltResponse {
    id: Uuid,
    marketplace_id: String,
    version: String,
    platform: String,
    engine_stamp: String,
    engine_tag: String,
    sha256: String,
    size: i64,
    file_name: String,
    signature: String,
    download_url: String,
}

/// Who may download a listing's binaries: whoever may download the listing.
async fn authorize(state: &AppState, auth: &Option<AuthUser>, asset: &Asset) -> Result<(), ApiError> {
    let owns = match auth {
        Some(u) => asset::user_owns_asset(&state.db, u.user_id, asset.id).await?,
        None => false,
    };
    crate::marketplace::authorize_download(asset, auth, owns)
}

/// `download_url` is this site's own counted route, relative, not a storage
/// URL: a presigned link expires in minutes, and the listing page keeps this
/// response around for as long as it is open.
fn response_for(asset: &Asset, p: PrebuiltPlugin) -> PrebuiltResponse {
    PrebuiltResponse {
        download_url: format!("/api/prebuilt-plugins/{}/file/{}", asset.id, p.id),
        id: p.id,
        marketplace_id: asset.marketplace_id.clone(),
        version: p.plugin_version,
        platform: p.platform,
        engine_stamp: p.engine_stamp,
        engine_tag: p.engine_tag,
        sha256: p.sha256,
        size: p.byte_size,
        file_name: p.file_name,
        signature: p.signature,
    }
}

async fn lookup(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Path(asset_id): Path<Uuid>,
    Query(q): Query<LookupQuery>,
) -> Result<Json<PrebuiltResponse>, ApiError> {
    let asset = Asset::find_by_id(&state.db, asset_id).await?.ok_or(ApiError::NotFound)?;
    authorize(&state, &auth, &asset).await?;
    let prebuilt = PrebuiltPlugin::find(&state.db, asset_id, &q.engine, &q.platform)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(response_for(&asset, prebuilt)))
}

async fn latest(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<Vec<PrebuiltResponse>>, ApiError> {
    let asset = Asset::find_by_id(&state.db, asset_id).await?.ok_or(ApiError::NotFound)?;
    authorize(&state, &auth, &asset).await?;
    let rows = PrebuiltPlugin::latest_per_platform(&state.db, asset_id).await?;
    Ok(Json(rows.into_iter().map(|p| response_for(&asset, p)).collect()))
}

/// The bytes, by redirect to a short-lived storage URL, counted as a download
/// of the binary and of the listing.
async fn file(
    State(state): State<AppState>,
    Extension(auth): Extension<Option<AuthUser>>,
    Path((asset_id, id)): Path<(Uuid, Uuid)>,
) -> Result<axum::response::Response, ApiError> {
    let asset = Asset::find_by_id(&state.db, asset_id).await?.ok_or(ApiError::NotFound)?;
    authorize(&state, &auth, &asset).await?;
    let prebuilt = PrebuiltPlugin::find_by_id(&state.db, id)
        .await?
        .filter(|p| p.asset_id == asset_id)
        .ok_or(ApiError::NotFound)?;
    let url = crate::marketplace::generate_presigned_url(&state, &prebuilt.file_key).await?;
    PrebuiltPlugin::increment_downloads(&state.db, prebuilt.id).await?;
    Asset::increment_downloads(&state.db, asset_id).await?;
    Ok(axum::response::Response::builder()
        .status(302)
        .header("location", url)
        .header("cache-control", "no-store")
        .body(axum::body::Body::empty())
        .unwrap())
}

/// The statement CI wrote beside each binary (`prebuilt.json`).
#[derive(Deserialize)]
struct Statement {
    id: String,
    version: String,
    platform: String,
    engine_stamp: String,
    sha256: String,
    size: u64,
    file_name: String,
    signature: String,
}

/// CI's upload: multipart with `statement` (the `prebuilt.json` text),
/// `engine_tag`, optional `engine_version`, and `library` (the binary).
async fn upload(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    crate::admin::verify_admin(&state, auth.user_id).await?;

    let mut statement: Option<Statement> = None;
    let mut engine_tag = String::new();
    let mut engine_version: Option<String> = None;
    let mut library: Option<Vec<u8>> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::Validation(format!("bad upload: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| ApiError::Validation(format!("bad upload field {name}: {e}")))?;
        match name.as_str() {
            "statement" => {
                statement = Some(
                    serde_json::from_slice(&bytes)
                        .map_err(|e| ApiError::Validation(format!("statement is not valid: {e}")))?,
                )
            }
            "engine_tag" => engine_tag = String::from_utf8_lossy(&bytes).trim().to_string(),
            "engine_version" => {
                let v = String::from_utf8_lossy(&bytes).trim().to_string();
                engine_version = (!v.is_empty()).then_some(v);
            }
            "library" => library = Some(bytes.to_vec()),
            _ => {}
        }
    }
    let statement = statement.ok_or_else(|| ApiError::Validation("missing statement".into()))?;
    let library = library.ok_or_else(|| ApiError::Validation("missing library".into()))?;

    validate(&statement, &library)?;

    let asset = Asset::find_by_marketplace_id(&state.db, &statement.id)
        .await?
        .ok_or_else(|| ApiError::Validation(format!("no listing has the marketplace id '{}'", statement.id)))?;
    if !matches!(asset.category.as_str(), "plugins" | "plugin") {
        return Err(ApiError::Validation(format!("'{}' is not a plugin listing", statement.id)));
    }
    let release_id = AssetRelease::find_by_version(&state.db, asset.id, &statement.version)
        .await?
        .map(|r| r.id);

    let (file_key, blob_sha) =
        crate::marketplace::store_blob(&state, asset.id, &statement.file_name, &library).await?;

    let row = PrebuiltPlugin::upsert(
        &state.db,
        &NewPrebuilt {
            asset_id: asset.id,
            release_id,
            plugin_version: &statement.version,
            platform: &statement.platform,
            engine_stamp: &statement.engine_stamp,
            engine_tag: &engine_tag,
            engine_version: engine_version.as_deref(),
            file_name: &statement.file_name,
            blob_sha: &blob_sha,
            file_key: &file_key,
            byte_size: library.len() as i64,
            sha256: &statement.sha256,
            signature: &statement.signature,
        },
    )
    .await?;

    Ok(Json(serde_json::json!({
        "id": row.id,
        "marketplace_id": statement.id,
        "platform": row.platform,
        "release_matched": release_id.is_some(),
    })))
}

/// What this site can check about an upload without the signing keys: the
/// statement is well formed and signed, and the bytes are the ones it names.
fn validate(s: &Statement, library: &[u8]) -> Result<(), ApiError> {
    let line = |v: &str| !v.is_empty() && v.len() <= 160 && v.chars().all(|c| c.is_ascii_graphic() || c == ' ');
    if !(line(&s.id) && line(&s.version) && line(&s.engine_stamp) && line(&s.file_name)) {
        return Err(ApiError::Validation("a statement field is empty or not one plain line".into()));
    }
    if s.file_name.contains('/') || s.file_name.contains('\\') {
        return Err(ApiError::Validation("file_name must be a bare file name".into()));
    }
    if !PLATFORMS.contains(&s.platform.as_str()) {
        return Err(ApiError::Validation(format!("unknown platform '{}'", s.platform)));
    }
    let hex = |v: &str, n: usize| v.len() == n && v.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase());
    if !hex(&s.sha256, 64) {
        return Err(ApiError::Validation("sha256 must be 64 lowercase hex digits".into()));
    }
    // An unsigned statement is what a fork's CI writes; storing it would serve a
    // binary every editor refuses.
    if !hex(&s.signature, 128) {
        return Err(ApiError::Validation("the statement is not signed".into()));
    }
    if s.size != library.len() as u64 {
        return Err(ApiError::Validation(format!(
            "the library is {} bytes but the statement says {}",
            library.len(),
            s.size
        )));
    }
    if renzora_models::storage_blob::sha256_hex(library) != s.sha256 {
        return Err(ApiError::Validation("the library does not match the statement's sha256".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn statement(library: &[u8]) -> Statement {
        Statement {
            id: "water".into(),
            version: "1.2.0".into(),
            platform: "windows-x64".into(),
            engine_stamp: "abc+rustc-1.100.0".into(),
            sha256: renzora_models::storage_blob::sha256_hex(library),
            size: library.len() as u64,
            file_name: "water.dll".into(),
            signature: "ab".repeat(64),
        }
    }

    #[test]
    fn a_matching_signed_upload_is_valid() {
        let lib = b"a library";
        assert!(validate(&statement(lib), lib).is_ok());
    }

    #[test]
    fn mismatched_bytes_and_unsigned_statements_are_refused() {
        let lib = b"a library";
        assert!(validate(&statement(lib), b"another library").is_err());
        let mut unsigned = statement(lib);
        unsigned.signature = String::new();
        assert!(validate(&unsigned, lib).is_err());
        let mut odd = statement(lib);
        odd.platform = "amiga".into();
        assert!(validate(&odd, lib).is_err());
        let mut path = statement(lib);
        path.file_name = "../evil.dll".into();
        assert!(validate(&path, lib).is_err());
    }
}
