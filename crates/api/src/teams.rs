use axum::{
    extract::{Extension, Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use renzora_models::notification::Notification;
use renzora_models::team::{Team, TeamInvite, TeamMember};
use renzora_models::user::User;
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_my_teams))
        .route("/", post(create_team))
        .route("/:id", get(get_team))
        .route("/:id", delete(delete_team))
        .route("/:id/members", get(list_members))
        .route("/:id/members/:user_id/role", put(update_member_role))
        .route("/:id/members/:user_id", delete(remove_member))
        .route("/:id/invite", post(invite_member))
        .route("/invites", get(list_my_invites))
        .route("/invites/:invite_id/accept", post(accept_invite))
        .route("/invites/:invite_id/decline", post(decline_invite))
        .layer(axum::middleware::from_fn(middleware::require_auth))
}

#[derive(Deserialize)]
struct CreateTeamRequest {
    name: String,
    description: Option<String>,
}

async fn create_team(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateTeamRequest>,
) -> Result<Json<Team>, ApiError> {
    let name = body.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::Validation("Team name must be 1-128 characters".into()));
    }

    // Check subscription allows teams
    let max_members = renzora_models::subscription::max_team_members(&state.db, auth.user_id).await?;
    if max_members == 0 {
        return Err(ApiError::Validation("Your plan does not include team management. Upgrade to Indie or Studio to create teams.".into()));
    }

    // Check user doesn't own too many teams
    let existing = Team::list_for_user(&state.db, auth.user_id).await?;
    let owned_count = existing.iter().filter(|t| t.owner_id == auth.user_id).count();
    if owned_count >= 5 {
        return Err(ApiError::Validation("Maximum 5 teams per user".into()));
    }

    let team = Team::create(&state.db, name, auth.user_id, body.description.as_deref().unwrap_or("")).await?;
    Ok(Json(team))
}

async fn list_my_teams(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<Team>>, ApiError> {
    let teams = Team::list_for_user(&state.db, auth.user_id).await?;
    Ok(Json(teams))
}

async fn get_team(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let team = Team::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Must be a member
    TeamMember::find(&state.db, id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;

    let members = TeamMember::list_for_team(&state.db, id).await?;
    let invites = TeamInvite::list_for_team(&state.db, id).await?;

    Ok(Json(serde_json::json!({
        "team": team,
        "members": members,
        "invites": invites,
    })))
}

async fn delete_team(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let team = Team::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if team.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }
    Team::delete(&state.db, id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

async fn list_members(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<renzora_models::team::TeamMemberWithUser>>, ApiError> {
    TeamMember::find(&state.db, id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;
    let members = TeamMember::list_for_team(&state.db, id).await?;
    Ok(Json(members))
}

#[derive(Deserialize)]
struct InviteRequest {
    /// Username or email of the person to invite.
    identifier: String,
    role: Option<String>,
}

async fn invite_member(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<InviteRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let team = Team::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Must be owner or admin
    let member = TeamMember::find(&state.db, id, auth.user_id).await?
        .ok_or(ApiError::Unauthorized)?;
    if member.role != "owner" && member.role != "admin" {
        return Err(ApiError::Unauthorized);
    }

    // Check team size limit (plan base + extra seats)
    let max_members = renzora_models::subscription::max_team_members(&state.db, team.owner_id).await?;
    let member_count = TeamMember::count(&state.db, id).await?;
    if member_count >= max_members as i64 {
        return Err(ApiError::Validation(format!(
            "Team member limit reached ({}/{}). Add extra seats or upgrade your plan.",
            member_count, max_members
        )));
    }

    let role = body.role.as_deref().unwrap_or("member");
    if !["member", "admin"].contains(&role) {
        return Err(ApiError::Validation("Role must be 'member' or 'admin'".into()));
    }

    let identifier = body.identifier.trim();
    if identifier.is_empty() {
        return Err(ApiError::Validation("Username or email required".into()));
    }

    // Find user by username or email
    let target_user = if identifier.contains('@') {
        User::find_by_email(&state.db, identifier).await?
    } else {
        User::find_by_username(&state.db, identifier).await?
    };

    let (invited_user_id, invited_email) = match target_user {
        Some(u) => {
            // Can't invite yourself
            if u.id == auth.user_id {
                return Err(ApiError::Validation("Cannot invite yourself".into()));
            }
            // Check not already a member
            if TeamMember::find(&state.db, id, u.id).await?.is_some() {
                return Err(ApiError::Validation("User is already a team member".into()));
            }
            (Some(u.id), Some(u.email.clone()))
        }
        None => {
            if identifier.contains('@') {
                (None, Some(identifier.to_string()))
            } else {
                return Err(ApiError::Validation(format!("User '{}' not found", identifier)));
            }
        }
    };

    let invite = TeamInvite::create(
        &state.db, id, auth.user_id,
        invited_user_id, invited_email.as_deref(), role,
    ).await?;

    // Send notification to invited user
    if let Some(uid) = invited_user_id {
        let inviter = User::find_by_id(&state.db, auth.user_id).await?
            .map(|u| u.username).unwrap_or_default();
        Notification::create(
            &state.db, uid, "team_invite",
            &format!("Team invite: {}", team.name),
            &format!("{} invited you to join {} as {}.", inviter, team.name, role),
            Some("/settings"),
        ).await?;
    }

    Ok(Json(serde_json::json!({ "invite": invite })))
}

#[derive(Deserialize)]
struct UpdateRoleRequest {
    role: String,
}

async fn update_member_role(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateRoleRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let team = Team::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Only owner can change roles
    if team.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }
    if user_id == auth.user_id {
        return Err(ApiError::Validation("Cannot change your own role".into()));
    }
    if !["member", "admin"].contains(&body.role.as_str()) {
        return Err(ApiError::Validation("Role must be 'member' or 'admin'".into()));
    }

    TeamMember::update_role(&state.db, id, user_id, &body.role).await?;
    Ok(Json(serde_json::json!({ "message": "Role updated" })))
}

async fn remove_member(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let team = Team::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    // Owner can remove anyone, members can remove themselves
    if team.owner_id != auth.user_id && auth.user_id != user_id {
        let member = TeamMember::find(&state.db, id, auth.user_id).await?
            .ok_or(ApiError::Unauthorized)?;
        if member.role != "admin" {
            return Err(ApiError::Unauthorized);
        }
    }

    // Can't remove the owner
    if user_id == team.owner_id {
        return Err(ApiError::Validation("Cannot remove the team owner".into()));
    }

    TeamMember::remove(&state.db, id, user_id).await?;
    Ok(Json(serde_json::json!({ "removed": true })))
}

async fn list_my_invites(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<TeamInvite>>, ApiError> {
    let invites = TeamInvite::list_pending_for_user(&state.db, auth.user_id).await?;
    Ok(Json(invites))
}

async fn accept_invite(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(invite_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let invite = TeamInvite::find_by_id(&state.db, invite_id).await?
        .ok_or(ApiError::NotFound)?;

    if invite.invited_user_id != Some(auth.user_id) {
        return Err(ApiError::Unauthorized);
    }
    if invite.status != "pending" {
        return Err(ApiError::Validation("Invite is no longer pending".into()));
    }
    if invite.expires_at < time::OffsetDateTime::now_utc() {
        return Err(ApiError::Validation("Invite has expired".into()));
    }

    TeamInvite::accept(&state.db, invite_id).await?;

    // Notify the inviter
    let team = Team::find_by_id(&state.db, invite.team_id).await?;
    let user = User::find_by_id(&state.db, auth.user_id).await?;
    if let (Some(team), Some(user)) = (team, user) {
        Notification::create(
            &state.db, invite.invited_by, "team_member_joined",
            &format!("{} joined {}", user.username, team.name),
            &format!("{} accepted the invite and joined your team.", user.username),
            None,
        ).await?;
    }

    Ok(Json(serde_json::json!({ "accepted": true })))
}

async fn decline_invite(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(invite_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let invite = TeamInvite::find_by_id(&state.db, invite_id).await?
        .ok_or(ApiError::NotFound)?;

    if invite.invited_user_id != Some(auth.user_id) {
        return Err(ApiError::Unauthorized);
    }

    TeamInvite::decline(&state.db, invite_id).await?;
    Ok(Json(serde_json::json!({ "declined": true })))
}
