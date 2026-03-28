use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub credit_balance: i64,
    pub referral_code: Option<String>,
    pub referred_by: Option<Uuid>,
    pub stripe_connect_id: Option<String>,
    pub stripe_connect_onboarded: bool,
    pub creator_policy_accepted_at: Option<OffsetDateTime>,
    pub discord_id: Option<String>,
    pub discord_username: Option<String>,
    pub discord_avatar: Option<String>,
    pub discord_linked_at: Option<OffsetDateTime>,
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    pub totp_backup_codes: Option<Vec<String>>,
    pub totp_enforced_by_role: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, sqlx::Error> {
        Self::create_with_referral(pool, username, email, password, None).await
    }

    pub async fn create_with_referral(
        pool: &PgPool,
        username: &str,
        email: &str,
        password: &str,
        referred_by: Option<Uuid>,
    ) -> Result<Self, sqlx::Error> {
        let password_hash = hash_password(password).map_err(|e| {
            sqlx::Error::Protocol(format!("password hash error: {e}"))
        })?;
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        let referral_code = generate_referral_code(id);

        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, credit_balance, referral_code, referred_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'user', 0, $5, $6, $7, $7)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .bind(&referral_code)
        .bind(referred_by)
        .bind(now)
        .fetch_one(pool)
        .await
    }

    /// Find a user by their referral code.
    pub async fn find_by_referral_code(pool: &PgPool, code: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE referral_code = $1")
            .bind(code.to_uppercase())
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn link_discord(
        pool: &PgPool,
        user_id: Uuid,
        discord_id: &str,
        discord_username: &str,
        discord_avatar: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET discord_id = $2, discord_username = $3, discord_avatar = $4, discord_linked_at = $5, updated_at = $5
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(discord_id)
        .bind(discord_username)
        .bind(discord_avatar)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(pool)
        .await
    }

    pub async fn unlink_discord(pool: &PgPool, user_id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET discord_id = NULL, discord_username = NULL, discord_avatar = NULL, discord_linked_at = NULL, updated_at = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_discord_id(pool: &PgPool, discord_id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE discord_id = $1")
            .bind(discord_id)
            .fetch_optional(pool)
            .await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let parsed = match PasswordHash::new(&self.password_hash) {
            Ok(h) => h,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    /// Check if this user's role requires 2FA
    pub fn role_requires_2fa(&self) -> bool {
        matches!(self.role.as_str(), "admin" | "moderator")
    }

    /// Save TOTP secret (during setup, before verification)
    pub async fn set_totp_secret(pool: &PgPool, user_id: Uuid, secret: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET totp_secret = $1, updated_at = NOW() WHERE id = $2")
            .bind(secret).bind(user_id).execute(pool).await?;
        Ok(())
    }

    /// Enable TOTP after successful verification, store backup codes
    pub async fn enable_totp(pool: &PgPool, user_id: Uuid, backup_codes: &[String]) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET totp_enabled = true, totp_backup_codes = $1, totp_enforced_by_role = true, updated_at = NOW() WHERE id = $2"
        )
        .bind(backup_codes).bind(user_id).execute(pool).await?;
        Ok(())
    }

    /// Disable TOTP and clear secret/backup codes
    pub async fn disable_totp(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET totp_enabled = false, totp_secret = NULL, totp_backup_codes = NULL, totp_enforced_by_role = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id).execute(pool).await?;
        Ok(())
    }

    /// Use a backup code (removes it from the list)
    pub async fn use_backup_code(pool: &PgPool, user_id: Uuid, code: &str) -> Result<bool, sqlx::Error> {
        let user = Self::find_by_id(pool, user_id).await?.ok_or(sqlx::Error::RowNotFound)?;
        let codes = user.totp_backup_codes.unwrap_or_default();
        if let Some(pos) = codes.iter().position(|c| c == code) {
            let mut remaining = codes;
            remaining.remove(pos);
            sqlx::query("UPDATE users SET totp_backup_codes = $1, updated_at = NOW() WHERE id = $2")
                .bind(&remaining).bind(user_id).execute(pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Generate a short, unique referral code from the user's UUID.
fn generate_referral_code(id: Uuid) -> String {
    id.to_string()[..8].to_uppercase()
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}
