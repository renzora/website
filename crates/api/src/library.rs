use axum::{
    extract::{Extension, Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use renzora_models::asset::Asset;
use renzora_models::licensing::*;
use renzora_models::notification::Notification;
use renzora_models::team::{Team, TeamMember};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/licenses", get(list_license_types))
        .route("/team/:team_id", get(list_team_library))
        .route("/team/:team_id/add", post(add_to_library))
        .route("/team/:team_id/remove/:asset_id", delete(remove_from_library))
        .route("/team/:team_id/request", post(request_asset))
        .route("/team/:team_id/requests", get(list_requests))
        .route("/team/:team_id/requests/:request_id/approve", post(approve_request))
        .route("/team/:team_id/requests/:request_id/deny", post(deny_request))
        .route("/team/:team_id/storage", get(storage_usage))
        .route("/pool/current", get(current_pool))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

/// List available license types (public info).
async fn list_license_types(
    State(state): State<AppState>,
) -> Result<Json<Vec<LicenseType>>, ApiError> {
    let types = LicenseType::list(&state.db).await?;
    Ok(Json(types))
}

/// List all assets in a team's cloud library.
async fn list_team_library(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<TeamLibraryItemWithAsset>>, ApiError> {
    // Must be a member
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_browse_library {
        return Err(ApiError::Unauthorized);
    }

    let items = TeamLibraryItem::list_for_team(&state.db, team_id).await?;
    Ok(Json(items))
}

#[derive(Deserialize)]
struct AddToLibraryRequest {
    asset_id: Uuid,
}

/// Add an asset to the team's cloud library (owner/manager/lead only).
async fn add_to_library(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
    Json(body): Json<AddToLibraryRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_add_to_library {
        return Err(ApiError::Validation("Your role does not have permission to add assets. Use the request feature instead.".into()));
    }

    // Check asset exists
    let asset = Asset::find_by_id(&state.db, body.asset_id).await?
        .ok_or(ApiError::NotFound)?;

    // Check not already in library
    if TeamLibraryItem::exists(&state.db, team_id, body.asset_id).await? {
        return Err(ApiError::Validation("Asset already in team library".into()));
    }

    // Check storage quota
    let team = Team::find_by_id(&state.db, team_id).await?.ok_or(ApiError::NotFound)?;
    let max_storage = renzora_models::subscription::max_storage_bytes(&state.db, team.owner_id).await?;
    let current_storage = TeamLibraryItem::total_size(&state.db, team_id).await?;

    // Estimate asset size (use 50MB as default if no file)
    let asset_size: i64 = 50 * 1024 * 1024; // TODO: track actual file sizes

    if current_storage + asset_size > max_storage {
        return Err(ApiError::Validation("Insufficient storage. Upgrade your plan or add extra storage.".into()));
    }

    // Check monthly allowance
    let allowance_max = 50; // TODO: make configurable per plan
    let (used, _max) = get_monthly_allowance(&state.db, auth.user_id, allowance_max).await?;
    if used >= allowance_max {
        return Err(ApiError::Validation("Monthly library allowance reached. Resets next month.".into()));
    }

    // Determine license type based on team's subscription
    let sub = renzora_models::subscription::Subscription::find_by_user(&state.db, team.owner_id).await?;
    let license_type = match sub.as_ref().map(|s| s.plan_id.as_str()) {
        Some("studio") => "enterprise",
        Some("indie") => "team",
        _ => "team",
    };

    // Grant license
    let grant = LicenseGrant::grant(
        &state.db, body.asset_id, None, Some(team_id),
        license_type, "library", 0, None,
    ).await?;

    // Add to library
    TeamLibraryItem::add(&state.db, team_id, body.asset_id, auth.user_id, grant.id, asset_size).await?;

    // Record in pool (weighted by price, capped at 500)
    CreatorPool::current(&state.db).await?;
    CreatorPool::record_library_add(&state.db, body.asset_id, asset.creator_id, team_id, auth.user_id, asset.price_credits).await?;

    // Increment allowance
    increment_allowance(&state.db, auth.user_id, allowance_max).await?;

    Ok(Json(serde_json::json!({
        "message": "Asset added to team library",
        "license_type": license_type,
        "asset": asset.name,
    })))
}

/// Remove an asset from the team library (owner/manager only).
async fn remove_from_library(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((team_id, asset_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_remove_from_library {
        return Err(ApiError::Unauthorized);
    }

    TeamLibraryItem::remove(&state.db, team_id, asset_id).await?;
    Ok(Json(serde_json::json!({ "removed": true })))
}

/// Request an asset be added (for designers/programmers who need approval).
async fn request_asset(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
    Json(body): Json<AddToLibraryRequest>,
) -> Result<Json<LibraryRequest>, ApiError> {
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_request_assets {
        return Err(ApiError::Unauthorized);
    }

    let asset = Asset::find_by_id(&state.db, body.asset_id).await?
        .ok_or(ApiError::NotFound)?;

    let request = LibraryRequest::create(&state.db, team_id, body.asset_id, auth.user_id).await?;

    // Notify team managers
    let members = TeamMember::list_for_team(&state.db, team_id).await?;
    let requester = renzora_models::user::User::find_by_id(&state.db, auth.user_id).await?;
    let req_name = requester.map(|u| u.username).unwrap_or_default();

    for m in &members {
        if m.role == "owner" || m.role == "manager" {
            let _ = Notification::create(
                &state.db, m.user_id, "library_request",
                "Asset request",
                &format!("{} requested \"{}\" be added to the team library.", req_name, asset.name),
                Some(&format!("/teams")),
            ).await;
        }
    }

    Ok(Json(request))
}

async fn list_requests(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<LibraryRequest>>, ApiError> {
    TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let requests = LibraryRequest::list_pending(&state.db, team_id).await?;
    Ok(Json(requests))
}

async fn approve_request(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((team_id, request_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_add_to_library {
        return Err(ApiError::Unauthorized);
    }

    let req = LibraryRequest::approve(&state.db, request_id, auth.user_id).await?;

    // Notify requester
    Notification::create(
        &state.db, req.requested_by, "library_request_approved",
        "Asset request approved",
        "Your asset request was approved and added to the team library.",
        Some("/teams"),
    ).await?;

    // Actually add the asset (reuse add logic)
    let asset = Asset::find_by_id(&state.db, req.asset_id).await?
        .ok_or(ApiError::NotFound)?;

    let team = Team::find_by_id(&state.db, team_id).await?.ok_or(ApiError::NotFound)?;
    let sub = renzora_models::subscription::Subscription::find_by_user(&state.db, team.owner_id).await?;
    let license_type = match sub.as_ref().map(|s| s.plan_id.as_str()) {
        Some("studio") => "enterprise",
        _ => "team",
    };

    let grant = LicenseGrant::grant(
        &state.db, req.asset_id, None, Some(team_id),
        license_type, "library", 0, None,
    ).await?;

    let asset_size: i64 = 50 * 1024 * 1024;
    TeamLibraryItem::add(&state.db, team_id, req.asset_id, auth.user_id, grant.id, asset_size).await?;
    CreatorPool::current(&state.db).await?;
    CreatorPool::record_library_add(&state.db, req.asset_id, asset.creator_id, team_id, auth.user_id, asset.price_credits).await?;

    Ok(Json(serde_json::json!({ "approved": true })))
}

async fn deny_request(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((team_id, request_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member = TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let perms = TeamRolePermission::find(&state.db, &member.role).await?
        .ok_or(ApiError::Internal("Role not found".into()))?;

    if !perms.can_add_to_library {
        return Err(ApiError::Unauthorized);
    }

    let req = LibraryRequest::deny(&state.db, request_id, auth.user_id).await?;

    Notification::create(
        &state.db, req.requested_by, "library_request_denied",
        "Asset request denied",
        "Your asset request was denied by a team manager.",
        None,
    ).await?;

    Ok(Json(serde_json::json!({ "denied": true })))
}

#[derive(serde::Serialize)]
struct StorageResponse {
    used_bytes: i64,
    max_bytes: i64,
    used_label: String,
    max_label: String,
    percent: f64,
    items: i64,
}

async fn storage_usage(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<StorageResponse>, ApiError> {
    TeamMember::find(&state.db, team_id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let team = Team::find_by_id(&state.db, team_id).await?.ok_or(ApiError::NotFound)?;
    let used = TeamLibraryItem::total_size(&state.db, team_id).await?;
    let max = renzora_models::subscription::max_storage_bytes(&state.db, team.owner_id).await?;

    let items = TeamLibraryItem::list_for_team(&state.db, team_id).await?.len() as i64;

    let format_bytes = |b: i64| -> String {
        if b >= 1024 * 1024 * 1024 { format!("{:.1}GB", b as f64 / 1073741824.0) }
        else if b >= 1024 * 1024 { format!("{:.1}MB", b as f64 / 1048576.0) }
        else { format!("{}KB", b / 1024) }
    };

    Ok(Json(StorageResponse {
        used_bytes: used,
        max_bytes: max,
        used_label: format_bytes(used),
        max_label: format_bytes(max),
        percent: if max > 0 { (used as f64 / max as f64) * 100.0 } else { 0.0 },
        items,
    }))
}

async fn current_pool(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthUser>,
) -> Result<Json<CreatorPool>, ApiError> {
    let pool = CreatorPool::current(&state.db).await?;
    Ok(Json(pool))
}
