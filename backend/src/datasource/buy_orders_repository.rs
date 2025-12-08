use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::models::{BuyOrder, CreateBuyOrderRequest};
use crate::repository::Repository;
use std::error::Error;
use std::ops::Add;
use time::{Duration, OffsetDateTime};

#[async_trait::async_trait]
pub trait BuyOrdersRepository {
    async fn fetch_buy_orders(&self) -> Result<Vec<BuyOrder>, Box<dyn Error>>;

    async fn fetch_user_buy_orders(&self, issuer_id: &i32) -> Result<Vec<BuyOrder>, Box<dyn Error>>;

    async fn create_buy_order(
        &self, 
        issuer_id: &i32, 
        order: &CreateBuyOrderRequest
    ) -> Result<Option<BuyOrder>, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl BuyOrdersRepository for Repository {
    
    async fn fetch_buy_orders(&self) -> Result<Vec<BuyOrder>, Box<dyn Error>> {
        let query = sqlx::query_as!(BuyOrder, "SELECT * FROM buy_orders;")
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }

    async fn fetch_user_buy_orders(&self, issuer_id: &i32) -> Result<Vec<BuyOrder>, Box<dyn Error>> {
        let query = sqlx::query_as!(BuyOrder, "SELECT * FROM buy_orders WHERE issuer_id = $1;", issuer_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }

    async fn create_buy_order(
        &self, 
        issuer_id: &i32, 
        order: &CreateBuyOrderRequest
    ) -> Result<Option<BuyOrder>, Box<dyn Error>> {
        let buy_volume = order.buy_volume;
        let buy_currency_code = &order.buy_currency_code;
        let sell_currency_code = &order.sell_currency_code;

        // find currency exchange values for pairs
        // let say we want to buy 1 USD and our offered currency is EUR
        // exchange rate is for instance for 1 USD you get 1.15 EUR
        let ratio = self.find_exchange_ratio_by_codes(
            buy_currency_code,
            sell_currency_code
        ).await?;
        if let Some (r) = ratio {
            let first_currency_value = r.first_currency_value.unwrap_or(0.0);
            let second_currency_value = r.second_currency_value.unwrap_or(0.0);
            // now we need ensure that user has enough balance on wallet that has offered currency
            let offered_currency_balance = self.check_currency_balance(
                &issuer_id,
                sell_currency_code
            ).await?.unwrap_or(0.0);
            if offered_currency_balance > 0.0 {
                // now we need to check if user has enough currency after conversion
                let required_balance = second_currency_value * buy_volume as f32;
                if offered_currency_balance > required_balance {
                    let created_at = OffsetDateTime::now_utc();
                    let updated_at = OffsetDateTime::now_utc();
                    let expires_at = created_at.add(Duration::days(7));
                    let exchange_ratio = first_currency_value / second_currency_value;
                    let query = sqlx::query_as!(BuyOrder,
                        "INSERT INTO buy_orders(issuer_id, buy_volume, buy_currency_code, sell_currency_code, buy_sell_exchange_ratio, created_at, updated_at, expires_at)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", 
                        issuer_id, buy_volume, buy_currency_code, sell_currency_code, exchange_ratio, created_at, updated_at, expires_at)
                        .fetch_optional(&self.pool)
                        .await?;
                    Ok(query)
                } else {
                    Err("Not enough balance to put buy order".into())
                }
            } else {
                Err("Offered currency balance is empty".into())
            }
        } else {
            Err("Exchange rates are unavailable".into())
        }
    }
}