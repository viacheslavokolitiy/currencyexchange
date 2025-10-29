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

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Currency {
    pub currency_id: Option<i32>,
    pub currency_code: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct CurrencyAmount {
    pub amount: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct BuyOrder {
    pub buy_order_id: Option<i32>,
    pub issuer_id: Option<i32>,
    pub buy_currency_amount: Option<i32>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub expires_at: Option<OffsetDateTime>,
    pub buy_currency_id: Option<i32>,
    pub sell_currency_id: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct SellOrder {
    pub sell_order_id: Option<i32>,
    pub issuer_id: Option<i32>,
    pub sell_currency_amount: Option<i32>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub expires_at: Option<OffsetDateTime>,
    pub sell_currency_id: Option<i32>,
    pub buy_currency_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub currency_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyBalance {
    pub user_id: Option<i32>,
    pub wallet_id: Option<i32>,
    pub amount: Option<i32>,
    pub currency_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct IncomingCurrencyWallet {
    pub wallet_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub currency_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OutgoingCurrencyWallet {
    pub wallet_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub currency_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingCurrencyWalletWithBalance{
    pub wallet_id: i32,
    pub currency_id: i32,
    pub currency_code: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OutgoingCurrencyWalletWithBalance {
    pub wallet_id: i32,
    pub currency_id: i32,
    pub currency_code: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyAmountQuery {
    pub id: Option<i32>,
    pub amount: Option<i32>,
    pub currency_id: Option<i32>,
    pub wallet_id: Option<i32>,
}