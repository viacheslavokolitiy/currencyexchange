use std::error::Error;
use crate::models::Wallet;

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
}