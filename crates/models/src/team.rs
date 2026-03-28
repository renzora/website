use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub owner_id: Uuid,
    pub avatar_url: Option<String>,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMember {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMemberWithUser {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: OffsetDateTime,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamInvite {
    pub id: Uuid,
    pub team_id: Uuid,
    pub invited_by: Uuid,
    pub invited_user_id: Option<Uuid>,
    pub invited_email: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}

impl Team {
    pub async fn create(db: &PgPool, name: &str, owner_id: Uuid, description: &str) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let slug = crate::asset::slugify_text(name, id);

        let team: Team = sqlx::query_as(
            "INSERT INTO teams (id, name, slug, owner_id, description) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        ).bind(id).bind(name).bind(&slug).bind(owner_id).bind(description)
        .fetch_one(db).await?;

        // Add owner as member
        sqlx::query("INSERT INTO team_members (team_id, user_id, role) VALUES ($1, $2, 'owner')")
            .bind(id).bind(owner_id).execute(db).await?;

        Ok(team)
    }

    pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM teams WHERE id = $1")
            .bind(id).fetch_optional(db).await
    }

    pub async fn find_by_slug(db: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM teams WHERE slug = $1")
            .bind(slug).fetch_optional(db).await
    }

    pub async fn list_for_user(db: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT t.* FROM teams t JOIN team_members m ON m.team_id = t.id WHERE m.user_id = $1 ORDER BY t.name"
        ).bind(user_id).fetch_all(db).await
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM team_invites WHERE team_id = $1").bind(id).execute(db).await?;
        sqlx::query("DELETE FROM team_members WHERE team_id = $1").bind(id).execute(db).await?;
        sqlx::query("DELETE FROM teams WHERE id = $1").bind(id).execute(db).await?;
        Ok(())
    }
}

impl TeamMember {
    pub async fn list_for_team(db: &PgPool, team_id: Uuid) -> Result<Vec<TeamMemberWithUser>, sqlx::Error> {
        sqlx::query_as(
            "SELECT m.id, m.team_id, m.user_id, m.role, m.joined_at, u.username, u.email, u.avatar_url
             FROM team_members m JOIN users u ON u.id = m.user_id
             WHERE m.team_id = $1 ORDER BY m.role, m.joined_at"
        ).bind(team_id).fetch_all(db).await
    }

    pub async fn find(db: &PgPool, team_id: Uuid, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id).bind(user_id).fetch_optional(db).await
    }

    pub async fn update_role(db: &PgPool, team_id: Uuid, user_id: Uuid, role: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE team_members SET role = $3 WHERE team_id = $1 AND user_id = $2")
            .bind(team_id).bind(user_id).bind(role).execute(db).await?;
        Ok(())
    }

    pub async fn remove(db: &PgPool, team_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id).bind(user_id).execute(db).await?;
        Ok(())
    }

    pub async fn count(db: &PgPool, team_id: Uuid) -> Result<i64, sqlx::Error> {
        let r: (i64,) = sqlx::query_as("SELECT COUNT(*)::bigint FROM team_members WHERE team_id = $1")
            .bind(team_id).fetch_one(db).await?;
        Ok(r.0)
    }
}

impl TeamInvite {
    pub async fn create(
        db: &PgPool,
        team_id: Uuid,
        invited_by: Uuid,
        invited_user_id: Option<Uuid>,
        invited_email: Option<&str>,
        role: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO team_invites (team_id, invited_by, invited_user_id, invited_email, role)
             VALUES ($1, $2, $3, $4, $5) RETURNING *"
        ).bind(team_id).bind(invited_by).bind(invited_user_id).bind(invited_email).bind(role)
        .fetch_one(db).await
    }

    pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM team_invites WHERE id = $1")
            .bind(id).fetch_optional(db).await
    }

    pub async fn list_pending_for_user(db: &PgPool, user_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM team_invites WHERE invited_user_id = $1 AND status = 'pending' AND expires_at > NOW() ORDER BY created_at DESC"
        ).bind(user_id).fetch_all(db).await
    }

    pub async fn list_for_team(db: &PgPool, team_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM team_invites WHERE team_id = $1 ORDER BY created_at DESC")
            .bind(team_id).fetch_all(db).await
    }

    pub async fn accept(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        let invite: Self = sqlx::query_as("UPDATE team_invites SET status = 'accepted' WHERE id = $1 RETURNING *")
            .bind(id).fetch_one(db).await?;

        if let Some(user_id) = invite.invited_user_id {
            sqlx::query("INSERT INTO team_members (team_id, user_id, role) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                .bind(invite.team_id).bind(user_id).bind(&invite.role).execute(db).await?;
        }
        Ok(())
    }

    pub async fn decline(db: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE team_invites SET status = 'declined' WHERE id = $1")
            .bind(id).execute(db).await?;
        Ok(())
    }
}
