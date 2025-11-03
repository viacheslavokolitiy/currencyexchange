use crate::datasource::api_models::CreateUserRequest;
use crate::datasource::errors::DataError;
use crate::datasource::models::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
    async fn create_user(&self, user: &CreateUserRequest) -> Result<Option<User>, DataError>;
}