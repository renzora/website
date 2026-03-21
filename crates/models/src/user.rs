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

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
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
