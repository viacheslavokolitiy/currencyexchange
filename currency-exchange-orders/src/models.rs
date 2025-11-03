use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use currency_exchange_data::datasource::models::{BuyOrder, SellOrder, Wallet};

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

#[derive(Serialize, Deserialize)]
pub struct CurrencyExchange {
    buy_orders: Vec<BuyOrder>,
    sell_orders: Vec<SellOrder>,
}