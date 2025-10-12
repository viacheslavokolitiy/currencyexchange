use crate::datasource::models::{CreateUserRequest, CreatedUserWithToken, Users};
use crate::utils::errors::Errors;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: &CreateUserRequest) -> Result<CreatedUserWithToken, Errors>;
    async fn find_users(&self) -> Result<Users, Errors>;
}