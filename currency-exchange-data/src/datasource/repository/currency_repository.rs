use crate::datasource::api_models::CreateCurrencyRequest;
use crate::datasource::errors::DataError;
use crate::datasource::models::Currency;

#[async_trait::async_trait]
pub trait CurrencyRepository {
    async fn create_currency(&self, request: &CreateCurrencyRequest) -> Result<Option<Currency>, DataError>;
}