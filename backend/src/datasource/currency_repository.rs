use crate::models::Currency;
use std::error::Error;

#[async_trait::async_trait]
pub trait CurrencyRepository {
    async fn find_currency_by_code(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>>;

    async fn create_new_currency(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>>;
    
    async fn find_all_currencies(&self) -> Result<Vec<Currency>, Box<dyn Error>>;
}