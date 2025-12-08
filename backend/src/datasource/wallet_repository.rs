use crate::datasource::user_repository::UserRepository;
use crate::models::Wallet;
use crate::repository::Repository;
use std::error::Error;

#[async_trait::async_trait]
pub trait WalletRepository {
    async fn check_if_wallet_exists(
        &self, 
        user_id: &i32, 
        wallet_currency: &str
    ) -> Result<Option<Wallet>, Box<dyn Error>>;
    
    async fn create_wallet(
        &self,
        user_id: &i32,
        wallet_currency: &str,
    ) -> Result<Option<Wallet>, Box<dyn Error>>;
    
    async fn check_currency_balance(
        &self,
        user_id: &i32,
        wallet_currency: &str
    ) -> Result<Option<f32>, Box<dyn Error>>;
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