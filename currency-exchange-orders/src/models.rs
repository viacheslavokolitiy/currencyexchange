use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub currency_id: i32,
    pub currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub currencies: Vec<Currency>
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct OrderIssuer {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderIssuerWithWallet {
    issuer: OrderIssuer,
    wallet: Wallet,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct BuyOrder {
    buy_order_id: i32,
    issuer_id: i32,
    buy_currency_amount: i32,
    exchange_rate: f32,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    created_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    updated_at: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::TimestampSecondsWithFrac<String>")]
    expires_at: Option<OffsetDateTime>,
    buy_currency_id: i32,
    sell_currency_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
#[serde_as]
pub struct SellOrder {
    sell_order_id: i32,
    issuer_id: i32,
    sell_currency_amount: i32,
    exchange_rate: f32,
    created_at: Option<OffsetDateTime>,
    updated_at: Option<OffsetDateTime>,
    expires_at: Option<OffsetDateTime>,
    sell_currency_id: i32,
    buy_currency_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyExchange {
    buy_orders: Vec<BuyOrder>,
    sell_orders: Vec<SellOrder>,
}