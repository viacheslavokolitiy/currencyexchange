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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
pub struct CreateSellOrderRequest {
    pub issuer_id: i32,
    pub sell_amount: i32,
    pub sell_currency_id: i32,
    pub buy_currency_id: i32,
    pub expiry_days: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BuyOrderQueryParams {
    pub count: i64
}

#[derive(Serialize, Deserialize)]
pub struct SellOrderQueryParams {
    pub count: i64
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeCurrencyRequest {
    pub sum: i32,
    pub rate: f32,
    pub order_issuer_id: i32,
    pub incoming_currency_id: i32,
    pub outgoing_currency_id: i32,
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

impl LoginRequest {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl CreateUserRequest {
    pub fn new<S: Into<String>>(
        username: S,
        email: S,
        password: S,
        firstname: S,
        middlename: Option<String>,
        lastname: S
    ) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            password: password.into(),
            firstname: firstname.into(),
            middlename,
            lastname: lastname.into(),
        }
    }
}

impl CreatedUser {
    pub fn new<S: Into<String>, I: Into<i32>>(
        id: I,
        username: S,
        email: S,
        firstname: S,
        middlename: Option<String>,
        lastname: S,
        created_at: Option<OffsetDateTime>,
        updated_at: Option<OffsetDateTime>
    ) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            email: email.into(),
            firstname: firstname.into(),
            middlename,
            lastname: lastname.into(),
            created_at,
            updated_at
        }
    }
}

impl CreateCurrencyRequest {
    pub fn new<S: Into<String>>(code: S) -> Self {
        Self {
            currency_code: code.into(),
        }
    }
}

impl CreateWalletRequest {
    pub fn new<I: Into<i32>>(user_id: I, currency_id: I) -> Self {
        Self {
            user_id: user_id.into(),
            currency_id: currency_id.into()
        }
    }
}

impl AddCurrencyRequest {
    pub fn new<S: Into<i32>>(user_id: S, currency_id: S) -> Self {
        Self {
            user_id: user_id.into(),
            currency_id: currency_id.into()
        }
    }
}

impl CreateBuyOrderRequest {
    pub fn new<I: Into<i32>>(
        issuer_id: I,
        buy_amount: I,
        buy_currency_id: I,
        sell_currency_id: I,
        expiry_days: I
    ) -> Self {
        Self {
            issuer_id: issuer_id.into(),
            buy_amount: buy_amount.into(),
            buy_currency_id: buy_currency_id.into(),
            sell_currency_id: sell_currency_id.into(),
            expiry_days: expiry_days.into()
        }
    }
}

impl CreateSellOrderRequest {
    pub fn new<I: Into<i32>>(
        issuer_id: I,
        sell_amount: I,
        sell_currency_id: I,
        buy_currency_id: I,
        expiry_days: I
    ) -> Self {
        Self {
            issuer_id: issuer_id.into(),
            sell_amount: sell_amount.into(),
            sell_currency_id: sell_currency_id.into(),
            buy_currency_id: buy_currency_id.into(),
            expiry_days: expiry_days.into()
        }
    }
}

impl ExchangeCurrencyRequest {
    pub fn new<I: Into<i32>, F: Into<f32>>(
        sum: I,
        rate: F,
        order_issuer_id: I,
        incoming_currency_id: I,
        outgoing_currency_id: I,
    ) -> Self {
        Self {
            sum: sum.into(),
            rate: rate.into(),
            order_issuer_id: order_issuer_id.into(),
            incoming_currency_id: incoming_currency_id.into(),
            outgoing_currency_id: outgoing_currency_id.into()
        }
    }
}