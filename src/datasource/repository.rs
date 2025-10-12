use crate::datasource::models::{CreateUserRequest, CreatedUser, CreatedUserWithToken, Users};
use crate::datasource::user_repository::UserRepository;
use crate::utils;
use crate::utils::env_parser::EnvParser;
use crate::utils::errors::Errors;
use sqlx::PgPool;
use time::OffsetDateTime;

pub struct Repository {
    pool: PgPool
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn create_user(&self, user: &CreateUserRequest) -> Result<CreatedUserWithToken, Errors> {
        let username = &user.username;
        let email = &user.email;
        let password = &user.password;
        let first_name = &user.firstname;
        let middle_name = &user.middlename.clone().unwrap_or_else(|| "".to_string());
        let last_name = &user.lastname;
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();

        let result = sqlx::query_as!(CreatedUser,
        "INSERT INTO users (username, email, password, firstname, middlename, lastname, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING user_id, username, email, firstname, middlename, lastname, created_at, updated_at", username, email, password, first_name, middle_name, last_name, created_at, updated_at)
            .fetch_one(&self.pool)
            .await
            .expect("Error creating user");
        let uid = result.user_id;
        let parser = EnvParser::new();
        let token = utils::jwt::get_token(uid, &parser);
        if token.is_ok() {
            Ok(CreatedUserWithToken::new(
                result, token.unwrap()
            ))
        } else {
            Err(Errors::JwtError)
        }
    }

    async fn find_users(&self) -> Result<Users, Errors> {
        let result = sqlx::query_as!(Users, "SELECT COUNT(*) as user_count FROM users")
            .fetch_one(&self.pool)
            .await
            .expect("Error finding users");
        Ok(result)
    }
}