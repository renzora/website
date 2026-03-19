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
        let password_hash = hash_password(password).map_err(|e| {
            sqlx::Error::Protocol(format!("password hash error: {e}"))
        })?;
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();

        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, credit_balance, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'user', 0, $5, $5)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .bind(now)
        .fetch_one(pool)
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

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}
