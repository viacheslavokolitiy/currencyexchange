use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow, Clone)]
#[serde_as]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub currency_id: i32,
    pub currency_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub currency_id: i32
}