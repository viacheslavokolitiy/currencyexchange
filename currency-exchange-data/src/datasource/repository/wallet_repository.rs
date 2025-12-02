use crate::datasource::api_models::{AddCurrencyRequest, BalanceRequest, CreateWalletRequest};
use crate::datasource::errors::DataError;
use crate::datasource::models::{CurrencyBalance, IncomingCurrencyWallet, OutgoingCurrencyWallet, Wallet};

#[async_trait::async_trait]
pub trait WalletRepository {
    async fn create_wallet(
        &self, 
        request: &CreateWalletRequest
    ) -> Result<Wallet, DataError>;

    async fn add_currency(
        &self, request: 
        &AddCurrencyRequest
    ) -> Result<Option<Wallet>, DataError>;
    
    async fn get_currency_balance(
        &self, 
        request: &BalanceRequest
    ) -> Result<Option<CurrencyBalance>, DataError>;
    
    async fn find_wallet_by_incoming_currency(
        &self, 
        issuer_id: &i32, 
        incoming_currency_id: &i32
    ) -> Result<Option<IncomingCurrencyWallet>, DataError>;
    
    async fn find_wallet_by_outgoing_currency(
        &self,
        issuer_id: &i32,
        outgoing_currency_id: &i32
    ) -> Result<Option<OutgoingCurrencyWallet>, DataError>;
}