use crate::datasource::api_models::{AddCurrencyRequest, BalanceRequest, CreateWalletRequest};
use crate::datasource::errors::DataError;
use crate::datasource::models::{CurrencyBalance, Wallet};

#[async_trait::async_trait]
pub trait WalletRepository {
    async fn create_wallet(&self, request: &CreateWalletRequest) -> Result<Wallet, DataError>;

    async fn add_currency(&self, request: &AddCurrencyRequest) -> Result<Option<Wallet>, DataError>;
    
    async fn get_currency_balance(&self, request: &BalanceRequest) -> Result<Option<CurrencyBalance>, DataError>;
}