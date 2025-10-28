use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatedUser {
    pub id: i32,
    username: String,
    email: String,
    firstname: String,
    middlename: Option<String>,
    lastname: String,
    created_at: Option<OffsetDateTime>,
    updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<CreatedUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: i32,
    pub currency_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct AddCurrencyRequest {
    pub user_id: i32,
    pub currency_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CreateCurrencyRequest {
    pub currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceRequest {
    pub wallet_id: i32,
    pub user_id: i32,
    pub currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBuyOrderRequest {
    pub issuer_id: i32,
    pub buy_amount: i32,
    pub buy_currency_id: i32,
    pub sell_currency_id: i32,
    pub expiry_days: i32,
}
#[derive(Serialize, Deserialize)]
#[serde_as]
pub struct CreateSellOrderRequest {
    pub issuer_id: i32,
    pub sell_amount: i32,
    pub sell_currency_id: i32,
    pub buy_currency_id: i32,
    pub expiry_days: i32,
}

impl CreateUserResponse {
    pub fn new(message: Option<String>, user: Option<CreatedUser>, token: Option<String>) -> Self {
        Self {
            error_message: message,
            user,
            token,
        }
    }
}

impl CreatedUser {
    pub fn new(
        id: i32,
        username: String,
        email: String,
        firstname: String,
        middlename: Option<String>,
        lastname: String,
        created_at: Option<OffsetDateTime>,
        updated_at: Option<OffsetDateTime>
    ) -> Self {
        Self { id, username, email, firstname, middlename, lastname, created_at, updated_at }
    }
}