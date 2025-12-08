use crate::models::CurrencyExchangeRatio;
use crate::repository::Repository;
use std::error::Error;

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

#[async_trait::async_trait]
impl CurrencyExchangeRatioRepository for Repository {
    async fn find_exchange_ratio_by_codes(
        &self,
        first_currency_code: &str,
        second_currency_code: &str
    ) -> Result<Option<CurrencyExchangeRatio>, Box<dyn Error>> {
        let query = sqlx::query_as!(CurrencyExchangeRatio,
            "SELECT * FROM currency_exchange_ratios WHERE first_currency_code = $1 AND second_currency_code = $2;",
            first_currency_code, second_currency_code
        ).fetch_optional(&self.pool).await?;
        Ok(query)
    }

    async fn add_exchange_ratio(
        &self,
        first_currency_code: &str,
        second_currency_code: &str,
        first_currency_value: f32,
        second_currency_value: f32
    ) -> Result<Option<CurrencyExchangeRatio>, Box<dyn Error>> {
        let exchange_ratio = self.find_exchange_ratio_by_codes(
            first_currency_code,
            second_currency_code
        ).await?;
        if exchange_ratio.is_none() {
            let query = sqlx::query_as!(CurrencyExchangeRatio,
            "INSERT INTO currency_exchange_ratios(first_currency_code, second_currency_code, first_currency_value, second_currency_value)
            VALUES ($1, $2, $3, $4) RETURNING *", first_currency_code, second_currency_code, first_currency_value, second_currency_value)
                .fetch_optional(&self.pool).await?;
            Ok(query)
        } else {
            Err("Exchange rate pair is already exists".into())
        }
    }
}

#[cfg(test)]
mod spec {
    use crate::database_connector::DatabaseConnector;
    use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
    use crate::env_parser::EnvParser;
    use crate::repository::Repository;
    use sqlx::PgPool;

    #[test]
    fn should_add_exchange_ratio() {
        let first_currency_code = "USD";
        let second_currency_code = "EUR";
        let first_currency_value:f32 = 1.00;
        let second_currency_value:f32 = 1.15;

        let parser = EnvParser::new();
        let conn = DatabaseConnector::new(
            parser.database_url(),
            parser.max_connections()
        );
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let pool = conn.connect()
                .await;
            let repo = Repository::new(pool.clone());
            let query = repo.add_exchange_ratio(
                first_currency_code,
                second_currency_code,
                first_currency_value, second_currency_value
            ).await;
            assert!(query.is_ok());
            let ratio = query.unwrap();
            assert!(ratio.is_some());
            let _ = cleanup_exchange_rates(&pool).await;
        })
    }

    #[test]
    fn should_not_add_if_pair_exists() {
        let first_currency_code = "USD";
        let second_currency_code = "EUR";
        let first_currency_value:f32 = 1.00;
        let second_currency_value:f32 = 1.15;

        let parser = EnvParser::new();
        let conn = DatabaseConnector::new(
            parser.database_url(),
            parser.max_connections()
        );
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let pool = conn.connect()
                .await;
            let repo = Repository::new(pool.clone());
            let query = repo.add_exchange_ratio(
                first_currency_code,
                second_currency_code,
                first_currency_value, second_currency_value
            ).await;
            assert!(query.is_ok());
            let ratio = query.unwrap();
            assert!(ratio.is_some());

            let query2 = repo.add_exchange_ratio(
                first_currency_code,
                second_currency_code,
                first_currency_value, second_currency_value
            ).await;
            assert!(query2.is_err());
            let _ = cleanup_exchange_rates(&pool).await;
        })
    }

    #[test]
    fn should_add_reverse_pair_if_pair_exists() {
        let first_currency_code = "USD";
        let second_currency_code = "EUR";
        let first_currency_value:f32 = 1.00;
        let second_currency_value:f32 = 1.15;

        let parser = EnvParser::new();
        let conn = DatabaseConnector::new(
            parser.database_url(),
            parser.max_connections()
        );
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let pool = conn.connect()
                .await;
            let repo = Repository::new(pool.clone());
            let query = repo.add_exchange_ratio(
                first_currency_code,
                second_currency_code,
                first_currency_value, second_currency_value
            ).await;
            assert!(query.is_ok());
            let ratio = query.unwrap();
            assert!(ratio.is_some());

            let query2 = repo.add_exchange_ratio(
                second_currency_code,
                first_currency_code,
                1.00,
                0.87
            ).await;
            assert!(query2.is_ok());
            let _ = cleanup_exchange_rates(&pool).await;
        })
    }

    async fn cleanup_exchange_rates(executor: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let res = sqlx::query!("DELETE FROM currency_exchange_ratios")
            .execute(executor)
            .await;
        if let Ok(data) = res {
            if data.rows_affected() > 0 {
                println!("Cleaning up");
                Ok(())
            } else {
                println!("Empty rows");
                Ok(())
            }
        } else {
            Err("Cleanup table failed".into())
        }
    }
}