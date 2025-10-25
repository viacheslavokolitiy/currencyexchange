use crate::datasource::api_models::{AddCurrencyRequest, CreateCurrencyRequest, CreateUserRequest, CreateWalletRequest};
use crate::datasource::errors::DataError;
use crate::datasource::models::{Currency, User, Wallet};
use crate::datasource::repository::currency_repository::CurrencyRepository;
use crate::datasource::repository::user_repository::UserRepository;
use crate::datasource::repository::wallet_repository::WalletRepository;
use sqlx::PgPool;
use time::OffsetDateTime;

pub struct Repository {
    pool: PgPool
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn find_wallet(&self, user_id: &i32) -> Option<Wallet> {
        sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1", user_id)
            .fetch_optional(&self.pool)
            .await
            .expect("Error wallet querying")
    }

    async fn find_currency(&self, code: &str) -> Option<Currency> {
        sqlx::query_as!(Currency, "SELECT * FROM currencies WHERE currency_code = $1", code)
            .fetch_optional(&self.pool)
            .await
            .expect("Error currency querying")
    }
}

#[async_trait::async_trait]
impl UserRepository for Repository {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await
            .expect("Error loading user");
        Ok(result)
    }

    async fn create_user(&self, user: &CreateUserRequest) -> Result<Option<User>, DataError> {
        let find_user_result = self.find_user_by_username(&user.username).await.expect("Error loading user");
        if find_user_result.is_none() {
            let username = &user.username;
            let email = &user.email;
            let password = &user.password;
            let firstname = &user.firstname;
            let middlename = &user.middlename;
            let lastname = &user.lastname;
            let created_at = OffsetDateTime::now_utc();
            let updated_at = OffsetDateTime::now_utc();
            let create_result = sqlx::query_as!(User,
                "INSERT INTO users(username, email, password, firstname, middlename, lastname, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
                username, email, password, firstname, middlename.clone().unwrap(), lastname, created_at, updated_at
            ).fetch_optional(&self.pool)
                .await
                .expect("Error creating user");
            Ok(create_result)
        } else {
            Err(DataError::EntryAlreadyExists(format!("User with name={} already exists", &user.username)))
        }
    }
}

#[async_trait::async_trait]
impl WalletRepository for Repository {
    async fn create_wallet(&self, request: &CreateWalletRequest) -> Result<Wallet, DataError> {
        let uid = request.user_id;
        let currency_id = request.currency_id;
        let create_wallet = sqlx::query_as!(Wallet, "INSERT INTO wallets(user_id, currency_id) VALUES ($1, $2) RETURNING *", uid, currency_id)
            .fetch_optional(&self.pool)
            .await
            .expect("Error wallet querying");
        if create_wallet.is_some() {
            Ok(create_wallet.unwrap())
        } else {
            Err(DataError::WalletCreationError(format!("Unable to create wallet for user with id={}", &uid)))
        }
    }

    async fn add_currency(&self, request: &AddCurrencyRequest) -> Result<Option<Wallet>, DataError> {
        let uid = request.user_id;
        let curr_id = request.currency_id;
        let wallet = self.find_wallet(&uid).await;
        if wallet.is_some() {
            let add_currency = sqlx::query_as!(Wallet, "INSERT INTO wallets(user_id, currency_id) VALUES ($1, $2) RETURNING *", uid, curr_id)
                .fetch_optional(&self.pool)
                .await
                .expect("Error adding another currency");
            Ok(add_currency)
        } else {
            Err(DataError::WalletNotFoundError(format!("Wallet for user with id={} not found", &uid)))
        }
    }
}

#[async_trait::async_trait]
impl CurrencyRepository for Repository {
    async fn create_currency(&self, request: &CreateCurrencyRequest) -> Result<Option<Currency>, DataError> {
        let code = &request.currency_code;
        let currency = self.find_currency(&code)
            .await;
        if currency.is_some() {
            Err(DataError::EntryAlreadyExists(format!("Currency with code={} already exists", &code)))
        } else {
            let currency_opt = sqlx::query_as!(Currency, "INSERT INTO currencies(currency_code) VALUES ($1) RETURNING *", code)
                .fetch_optional(&self.pool)
                .await
                .expect("Error creating currency");
            Ok(currency_opt)
        }
    }
}