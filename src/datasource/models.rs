use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct CreatedUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct CreatedUserWithToken {
    pub user: CreatedUser,
    pub token: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Users {
    pub user_count: Option<i64>
}

impl CreatedUserWithToken {
    pub fn new(user: CreatedUser, token: String) -> Self {
        Self { user, token }
    }
}