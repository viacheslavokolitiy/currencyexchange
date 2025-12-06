use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::user_repository::UserRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::models::{BuyOrder, CreateBuyOrderRequest, CreateSellOrderRequest, CreateUserRequest, CreateUserResponse, Currency, CurrencyExchangeRatio, DatabaseUser, SellOrder, UserId, Wallet};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use std::error::Error;
use std::ops::Add;
use time::{Duration, OffsetDateTime};
use crate::datasource::buy_orders_repository::BuyOrdersRepository;
use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
use crate::datasource::sell_orders_repository::SellOrdersRepository;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn check_if_user_exists(
        &self, 
        username: &str
    ) -> Result<Option<UserId>, Box<dyn Error>> {
        let query = sqlx::query_as!(UserId, "SELECT id FROM users WHERE username = $1;", username)
            .fetch_optional(&self.pool)
            .await?;
        if query.is_some() {
            Ok(query)
        } else {
            Ok(None)
        }
    }

    async fn create_user(
        &self, 
        request: &CreateUserRequest
    ) -> Result<Option<CreateUserResponse>, Box<dyn Error>> {
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let (username, email, password, firstname, middlename, lastname) = (
            &request.username, 
            &request.email,
            &request.password,
            &request.firstname,
            &request.middlename,
            &request.lastname);
        let user_middle_name = middlename.clone().unwrap_or("".to_string());
        let hashed_password = self.hash_password(&password).await?;
        let query = sqlx::query_as!(CreateUserResponse, 
            "INSERT INTO users (username, email, password, firstname, middlename, lastname, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, username, email, firstname, middlename, lastname, created_at, updated_at",
            username, email, hashed_password, firstname, user_middle_name, lastname, created_at, updated_at)
            .fetch_optional(&self.pool)
            .await?;
        Ok(query)
    }

    async fn verify_password(&self, user_id: i32, password: &str) -> Result<bool, Box<dyn Error>> {
        let query = sqlx::query_as!(DatabaseUser, "SELECT * FROM users WHERE id = $1;", user_id)
            .fetch_optional(&self.pool)
            .await
            .expect("Database user query failed");
        if query.is_some() {
            let user_password = query.unwrap().password;
            let parsed_hash = PasswordHash::new(&user_password)?;

            let verify_result = Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();
            Ok(verify_result)
        } else {
            Ok(false)
        }
    }

    async fn check_if_user_exists_by_id(&self, user_id: &i32) -> Result<Option<UserId>, Box<dyn Error>> {
        let query = sqlx::query_as!(UserId, "SELECT id FROM users WHERE id = $1;", user_id)
            .fetch_optional(&self.pool)
            .await?;
        if query.is_some() {
            Ok(query)
        } else {
            Ok(None)
        }
    }
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

#[async_trait::async_trait]
impl WalletRepository for Repository {
    async fn check_if_wallet_exists(
        &self,
        user_id: &i32,
        wallet_currency: &str
    ) -> Result<Option<Wallet>, Box<dyn Error>> {

        let user_exists = self.check_if_user_exists_by_id(user_id)
            .await?;
        if user_exists.is_none() {
            Ok(None)
        } else {
            let query = sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1 AND currency_code = $2", user_id, wallet_currency)
                .fetch_optional(&self.pool)
                .await?;
            Ok(query)
        }
    }

    async fn create_wallet(
        &self,
        user_id: &i32,
        wallet_currency: &str
    ) -> Result<Option<Wallet>, Box<dyn Error>> {
        let default_currency_amount:f32 = 0.0;
        let query = sqlx::query_as!(Wallet,
                "INSERT INTO wallets(currency_amount, currency_code, user_id)
                 VALUES ($1, $2, $3) RETURNING * ", default_currency_amount, wallet_currency, user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(query)
    }

    async fn check_currency_balance(
        &self, 
        user_id: &i32, 
        wallet_currency: &str
    ) -> Result<Option<f32>, Box<dyn Error>> {
        let query = sqlx::query_as!(Wallet, 
            "SELECT * FROM wallets WHERE user_id = $1 AND currency_code = $2", user_id, wallet_currency)
            .fetch_optional(&self.pool)
            .await?;
        if let Some(wallet) = query {
            if let Some(amount) = wallet.currency_amount {
                Ok(Some(amount))
            } else { 
                Ok(None)
            }
        } else { 
            Ok(None)
        }
    }
}

#[async_trait::async_trait]
impl BuyOrdersRepository for Repository {
    async fn fetch_buy_orders(&self) -> Result<Vec<BuyOrder>, Box<dyn Error>> {
        let query = sqlx::query_as!(BuyOrder, "SELECT * FROM buy_orders;")
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }

    async fn create_buy_order(&self, order: &CreateBuyOrderRequest) -> Result<Option<BuyOrder>, Box<dyn Error>> {
        let buy_volume = order.buy_volume;
        let buy_currency_code = &order.buy_currency_code;
        let sell_currency_code = &order.sell_currency_code;
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
        let query = sqlx::query_as!(CurrencyExchangeRatio,
            "INSERT INTO currency_exchange_ratios(first_currency_code, second_currency_code, first_currency_value, second_currency_value)
            VALUES ($1, $2, $3, $4) RETURNING *", first_currency_code, second_currency_code, first_currency_value, second_currency_value)
        .fetch_optional(&self.pool).await?;
        Ok(query)
    }
}

#[cfg(test)]
mod repository_spec {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
    use crate::database_connector::DatabaseConnector;
    use crate::env_parser::EnvParser;

    #[test]
    fn should_verify_pwds() {
        let password = "qwerty12345";
        let password_bytes = password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate();
        let hashed = argon2.hash_password(password_bytes, &salt)
            .unwrap()
            .to_string();
        println!("Hashed password{}", hashed);

        let parsed_hash = PasswordHash::new(hashed.as_str());
        assert!(parsed_hash.is_ok());

        let x = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash.unwrap())
            .is_ok();
        assert_eq!(x, true);
    }
    
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
    }
}