use sqlx::PgPool;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicProfile {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub bio: String,
    pub website: String,
    pub location: String,
    pub gender: String,
    pub profile_color: String,
    pub banner_color: String,
    pub avatar_url: Option<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub post_count: i32,
    pub credit_balance: i64,
    pub is_following: bool,
    pub badges: Vec<BadgeInfo>,
    pub created_at: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BadgeInfo {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
}

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    role: String,
    bio: String,
    website: String,
    location: String,
    gender: String,
    profile_color: String,
    banner_color: String,
    avatar_url: Option<String>,
    follower_count: i32,
    following_count: i32,
    post_count: i32,
    credit_balance: i64,
    created_at: time::OffsetDateTime,
}

pub async fn get_profile(pool: &PgPool, username: &str, viewer_id: Option<Uuid>) -> Result<Option<PublicProfile>, sqlx::Error> {
    let user: Option<UserRow> = sqlx::query_as(
        "SELECT id, username, role, bio, website, location, gender, profile_color, banner_color, avatar_url, follower_count, following_count, post_count, credit_balance, created_at FROM users WHERE username=$1"
    ).bind(username).fetch_optional(pool).await?;

    let Some(u) = user else { return Ok(None) };

    let is_following = if let Some(vid) = viewer_id {
        let r: Option<(Uuid,)> = sqlx::query_as("SELECT follower_id FROM follows WHERE follower_id=$1 AND following_id=$2")
            .bind(vid).bind(u.id).fetch_optional(pool).await?;
        r.is_some()
    } else { false };

    let badges: Vec<BadgeInfo> = sqlx::query_as(
        "SELECT b.slug,b.name,b.description,b.icon,b.color FROM user_badges ub JOIN badges b ON b.id=ub.badge_id WHERE ub.user_id=$1"
    ).bind(u.id).fetch_all(pool).await?;

    Ok(Some(PublicProfile {
        id: u.id, username: u.username, role: u.role, bio: u.bio, website: u.website,
        location: u.location, gender: u.gender, profile_color: u.profile_color, banner_color: u.banner_color,
        avatar_url: u.avatar_url, follower_count: u.follower_count, following_count: u.following_count,
        post_count: u.post_count, credit_balance: u.credit_balance, is_following, badges,
        created_at: u.created_at.to_string(),
    }))
}

pub async fn toggle_follow(pool: &PgPool, follower_id: Uuid, following_id: Uuid) -> Result<bool, sqlx::Error> {
    if follower_id == following_id { return Ok(false); }
    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT follower_id FROM follows WHERE follower_id=$1 AND following_id=$2")
        .bind(follower_id).bind(following_id).fetch_optional(pool).await?;

    if existing.is_some() {
        sqlx::query("DELETE FROM follows WHERE follower_id=$1 AND following_id=$2").bind(follower_id).bind(following_id).execute(pool).await?;
        sqlx::query("UPDATE users SET follower_count=GREATEST(follower_count-1,0) WHERE id=$1").bind(following_id).execute(pool).await?;
        sqlx::query("UPDATE users SET following_count=GREATEST(following_count-1,0) WHERE id=$1").bind(follower_id).execute(pool).await?;
        Ok(false)
    } else {
        sqlx::query("INSERT INTO follows (follower_id,following_id) VALUES ($1,$2)").bind(follower_id).bind(following_id).execute(pool).await?;
        sqlx::query("UPDATE users SET follower_count=follower_count+1 WHERE id=$1").bind(following_id).execute(pool).await?;
        sqlx::query("UPDATE users SET following_count=following_count+1 WHERE id=$1").bind(follower_id).execute(pool).await?;
        Ok(true)
    }
}
