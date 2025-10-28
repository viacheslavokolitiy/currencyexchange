use crate::datasource::api_models::{AddCurrencyRequest, BalanceRequest, CreateBuyOrderRequest, CreateCurrencyRequest, CreateSellOrderRequest, CreateUserRequest, CreateWalletRequest};
use crate::datasource::errors::DataError;
use crate::datasource::models::{BuyOrder, Currency, CurrencyBalance, SellOrder, User, Wallet};
use crate::datasource::repository::currency_repository::CurrencyRepository;
use crate::datasource::repository::user_repository::UserRepository;
use crate::datasource::repository::wallet_repository::WalletRepository;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use crate::datasource::repository::order_repository::OrderRepository;

pub struct Repository {
    pool: PgPool
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_wallet(&self, user_id: &i32) -> Option<Wallet> {
        sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1", user_id)
            .fetch_optional(&self.pool)
            .await
            .expect("Error wallet querying")
    }
    
    pub async fn find_all_wallets(&self, user_id: &i32) -> Vec<Wallet> {
        sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1", user_id)
            .fetch_all(&self.pool)
            .await
            .expect("Error wallet querying")
    }

    pub async fn find_currency(&self, code: &str) -> Option<Currency> {
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

    async fn get_currency_balance(&self, request: &BalanceRequest) -> Result<Option<CurrencyBalance>, DataError> {
        let uid = request.user_id;
        let result = sqlx::query_as!(CurrencyBalance,
            "SELECT w.wallet_id, w.user_id, ca.amount, ca.currency_id FROM wallets as w
            JOIN currency_amount as ca
            ON w.wallet_id = ca.wallet_id
            WHERE w.user_id = $1", uid)
            .fetch_optional(&self.pool)
            .await;
        if result.is_ok() {
            Ok(result.unwrap())
        } else {
            Err(DataError::WalletBalanceError(format!("Wallet balance query for user with id={} failed", &uid)))
        }
    }
}

#[async_trait::async_trait]
impl CurrencyRepository for Repository {
    async fn create_currency(&self, request: &CreateCurrencyRequest) -> Result<Option<Currency>, DataError> {
        let code = &request.currency_code;
        let currency = self.find_currency(code)
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
#[async_trait::async_trait]
impl OrderRepository for Repository {
    async fn find_buy_orders<S: Into<i64> + Send>(
        &self,
        limit: S
    ) -> Result<Vec<BuyOrder>, DataError> {
        let limit = limit.into();
        let vec = sqlx::query_as!(BuyOrder, "SELECT * FROM buy_orders LIMIT $1", limit)
            .fetch_all(&self.pool)
            .await
            .expect("Error loading orders");
        Ok(vec)
    }

    async fn find_sell_orders<S: Into<i64> + Send>(
        &self,
        limit: S
    ) -> Result<Vec<SellOrder>, DataError> {
        let limit = limit.into();
        let vec = sqlx::query_as!(SellOrder, "SELECT * FROM sell_orders LIMIT $1", limit)
            .fetch_all(&self.pool)
            .await
            .expect("Error loading orders");
        Ok(vec)
    }

    async fn create_buy_order(&self, req: &CreateBuyOrderRequest) -> Result<BuyOrder, DataError> {
        let issuer_id = req.issuer_id;
        let amount = req.buy_amount;
        let buy_id = req.buy_currency_id;
        let sell_id = req.sell_currency_id;
        let expiry_total = OffsetDateTime::now_utc() + Duration::days(req.expiry_days as i64);
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let result = sqlx::query_as!(BuyOrder, 
            "INSERT INTO buy_orders(issuer_id, buy_currency_amount, buy_currency_id, sell_currency_id, created_at, updated_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING buy_order_id, issuer_id, buy_currency_amount, buy_currency_id, sell_currency_id, created_at, updated_at, expires_at", 
            issuer_id, amount, buy_id, sell_id, created_at, updated_at, expiry_total)
            .fetch_one(&self.pool)
            .await
            .expect("Error creating buy order");
        Ok(result)   
    }

    async fn create_sell_order(&self, req: &CreateSellOrderRequest) -> Result<SellOrder, DataError> {
        let issuer_id = req.issuer_id;
        let amount = req.sell_amount;
        let buy_id = &req.buy_currency_id;
        let sell_id = &req.sell_currency_id;
        let expiry_total = OffsetDateTime::now_utc() + Duration::days(req.expiry_days as i64);
        let created_at = OffsetDateTime::now_utc();
        let updated_at = OffsetDateTime::now_utc();
        let result = sqlx::query_as!(SellOrder, 
            "INSERT INTO sell_orders(issuer_id, sell_currency_amount, buy_currency_id, sell_currency_id, created_at, updated_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING sell_order_id, issuer_id, sell_currency_amount, buy_currency_id, sell_currency_id, created_at, updated_at, expires_at", 
            issuer_id, amount, buy_id, sell_id, created_at, updated_at, expiry_total)
            .fetch_one(&self.pool)
            .await
            .expect("Error creating sell order");
        Ok(result)
    }
}