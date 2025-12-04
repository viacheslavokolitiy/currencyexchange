use crate::models::{CreateUserRequest, CreateUserResponse, DatabaseUser, UserId};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::testing::TestTermination;
use sqlx::PgPool;
use std::error::Error;
use std::hash::Hash;
use time::OffsetDateTime;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn check_if_exists(
        &self, 
        username: &str
    ) -> Result<Option<UserId>, Box<dyn Error>>;
    
    async fn create_user(
        &self, 
        request: &CreateUserRequest
    ) -> Result<Option<CreateUserResponse>, Box<dyn Error>>;

    async fn verify_password(&self, user_id: i32, password: &str) -> Result<bool, Box<dyn Error>>;

    async fn hash_password(&self, password: &str) -> Result<String, Box<dyn Error>> {
        let password_bytes = password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate();
        let hashed = argon2.hash_password(password_bytes, &salt)
            .unwrap()
            .to_string();
        Ok(hashed)
    }
}

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn check_if_exists(
        &self, 
        username: &str
    ) -> Result<Option<UserId>, Box<dyn Error>> {
        let query = sqlx::query_as!(UserId, "SELECT id FROM users WHERE username = $1;", username)
            .fetch_optional(&self.pool)
            .await?;
        if query.is_some() {
            Ok(query)
        } else {
            Ok(None)
        }
    }

    async fn create_user(
        &self, 
        request: &CreateUserRequest
    ) -> Result<Option<CreateUserResponse>, Box<dyn Error>> {
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let (username, email, password, firstname, middlename, lastname) = (
            &request.username, 
            &request.email,
            &request.password,
            &request.firstname,
            &request.middlename,
            &request.lastname);
        let user_middle_name = middlename.clone().unwrap_or("".to_string());
        let hashed_password = self.hash_password(&password).await?;
        let query = sqlx::query_as!(CreateUserResponse, 
            "INSERT INTO users (username, email, password, firstname, middlename, lastname, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, username, email, firstname, middlename, lastname, created_at, updated_at",
            username, email, hashed_password, firstname, user_middle_name, lastname, created_at, updated_at)
            .fetch_optional(&self.pool)
            .await?;
        Ok(query)
    }

    async fn verify_password(&self, user_id: i32, password: &str) -> Result<bool, Box<dyn Error>> {
        let query = sqlx::query_as!(DatabaseUser, "SELECT * FROM users WHERE id = $1;", user_id)
            .fetch_optional(&self.pool)
            .await
            .expect("Database user query failed");
        if query.is_some() {
            let user_password = query.unwrap().password;
            let parsed_hash = PasswordHash::new(&user_password)?;

            let verify_result = Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();
            Ok(verify_result)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod user_repository_spec {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

    #[test]
    fn should_verify_pwds() {
        let password = "qwerty12345";
        let password_bytes = password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate();
        let hashed = argon2.hash_password(password_bytes, &salt)
            .unwrap()
            .to_string();
        println!("Hashed password{}", hashed);

        let parsed_hash = PasswordHash::new(hashed.as_str());
        assert!(parsed_hash.is_ok());

        let x = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash.unwrap())
            .is_ok();
        assert_eq!(x, true);
    }
}