use std::error::Error;
use crate::models::CurrencyExchangeRatio;

#[async_trait::async_trait]
pub trait CurrencyExchangeRatioRepository {
    async fn find_exchange_ratio_by_codes(
        &self,
        first_currency_code: &str,
        second_currency_code: &str,
    ) -> Result<Option<CurrencyExchangeRatio>, Box<dyn Error>>;

    async fn add_exchange_ratio(
        &self,
        first_currency_code: &str,
        second_currency_code: &str,
        first_currency_value: f32,
        second_currency_value: f32
    ) -> Result<Option<CurrencyExchangeRatio>, Box<dyn Error>>;
}