use std::error::Error;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use crate::models::{CreateUserRequest, CreateUserResponse, UserId};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn check_if_user_exists(
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
    
    async fn check_if_user_exists_by_id(&self, user_id: &i32) -> Result<Option<UserId>, Box<dyn Error>>;
}