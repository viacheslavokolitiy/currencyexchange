use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::models::{CreateSellOrderRequest, SellOrder};
use crate::repository::Repository;
use std::error::Error;
use std::ops::Add;
use time::{Duration, OffsetDateTime};

#[async_trait::async_trait]
pub trait SellOrdersRepository {

    async fn fetch_sell_orders(&self) -> Result<Vec<SellOrder>, Box<dyn Error>>;
    
    async fn create_sell_order(
        &self, 
        order: &CreateSellOrderRequest
    ) -> Result<Option<SellOrder>, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl SellOrdersRepository for Repository {
    async fn fetch_sell_orders(&self) -> Result<Vec<SellOrder>, Box<dyn Error>> {
        let query = sqlx::query_as!(SellOrder, "SELECT * FROM sell_orders;")
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }

    async fn create_sell_order(
        &self,
        order: &CreateSellOrderRequest
    ) -> Result<Option<SellOrder>, Box<dyn Error>> {
        let sell_volume = order.sell_volume;
        let sell_currency_code = &order.buy_currency_code;
        let buy_currency_code = &order.buy_currency_code;
        let issuer_id = order.issuer_id;
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
            let sell_currency_balance = self.check_currency_balance(
                &issuer_id,
                &sell_currency_code
            ).await?.unwrap_or(0.0);
            if sell_currency_balance < sell_volume as f32 {
                Err("Not enough currency to put sell order".into())
            } else {
                let created_at = OffsetDateTime::now_utc();
                let updated_at = OffsetDateTime::now_utc();
                let expires_at = created_at.add(Duration::days(7));
                let exchange_ratio = first_currency_value / second_currency_value;
                let query = sqlx::query_as!(SellOrder, 
                "INSERT INTO sell_orders(issuer_id, sell_volume, sell_currency_code, buy_currency_code, buy_sell_exchange_ratio, created_at, updated_at, expires_at)
                VALUES($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
                issuer_id, sell_volume, sell_currency_code, buy_currency_code, exchange_ratio, created_at, updated_at, expires_at)
                    .fetch_optional(&self.pool).await?;
                Ok(query)
            }
        } else {
            Err("Exchange rates are unavailable".into())
        }
    }
}