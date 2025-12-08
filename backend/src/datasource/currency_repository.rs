use crate::models::Currency;
use crate::repository::Repository;
use std::error::Error;

#[async_trait::async_trait]
pub trait CurrencyRepository {
    async fn find_currency_by_code(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>>;

    async fn create_new_currency(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>>;
    
    async fn find_all_currencies(&self) -> Result<Vec<Currency>, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl CurrencyRepository for Repository {
    async fn find_currency_by_code(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>> {
        let query = sqlx::query_as!(Currency, "SELECT * FROM currencies WHERE currency_code = $1;", code)
            .fetch_optional(&self.pool)
            .await?;
        Ok(query)
    }
    async fn create_new_currency(&self, code: &str) -> Result<Option<Currency>, Box<dyn Error>> {
        let db_currency = self.find_currency_by_code(code).await?;
        if db_currency.is_none() {
            let query = sqlx::query_as!(Currency,
                "INSERT INTO currencies(currency_code) VALUES ($1) RETURNING *", code)
                .fetch_optional(&self.pool)
                .await?;
            Ok(query)
        } else {
            Err("currency code already exists".into())
        }
    }

    async fn find_all_currencies(&self) -> Result<Vec<Currency>, Box<dyn Error>> {
        let query = sqlx::query_as!(Currency, "SELECT * FROM currencies;")
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }
}