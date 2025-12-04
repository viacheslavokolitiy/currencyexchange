use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::user_repository::UserRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::models::{CreateUserRequest, CreateUserResponse, Currency, DatabaseUser, UserId, Wallet};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use std::error::Error;
use time::OffsetDateTime;

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

        let wallet_exists = self.check_if_wallet_exists(user_id, wallet_currency)
            .await?;
        if wallet_exists.is_none() {
            let default_currency_amount:f32 = 0.0;
            let query = sqlx::query_as!(Wallet,
                "INSERT INTO wallets(currency_amount, currency_code, user_id)
                 VALUES ($1, $2, $3) RETURNING * ", default_currency_amount, wallet_currency, user_id)
                .fetch_optional(&self.pool)
                .await?;
            Ok(query)
        } else {
            Err("Wallet with currency code already exists".into())
        }
    }
}

#[cfg(test)]
mod user_repository_spec {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

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
}