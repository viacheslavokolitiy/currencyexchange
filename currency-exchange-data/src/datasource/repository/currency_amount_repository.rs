use crate::datasource::errors::DataError;
use crate::datasource::models::{Currency, CurrencyAmount, CurrencyAmountQuery, Wallet};

#[async_trait::async_trait]
pub trait CurrencyAmountRepository {
    async fn currency_amount(&self, currency_id: &i32) -> Result<Option<CurrencyAmount>, DataError>;
    
    async fn exchange_currencies(
        &self, 
        sum: i32, 
        incoming_currency_id: i32, 
        outgoing_currency_id: i32, 
        exchange_rate: f32, 
        incoming_currency_wallet_id: i32, 
        outgoing_currency_wallet_id: i32
    ) -> Result<Option<CurrencyAmountQuery>, DataError>;
}