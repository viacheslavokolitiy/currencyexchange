use sqlx::PgPool;
use currency_exchange_data::datasource::errors::DataError;
use currency_exchange_data::datasource::models::{CurrencyAmountQuery, IncomingCurrencyWallet, OutgoingCurrencyWallet, Wallet};
use currency_exchange_data::datasource::repository::currency_amount_repository::CurrencyAmountRepository;
use currency_exchange_data::datasource::repository::repository::Repository;
use currency_exchange_data::datasource::repository::wallet_repository::WalletRepository;

#[async_trait::async_trait]
pub trait BuyTransactionManager {
    async fn process_buy_transaction(&self) -> Result<CurrencyAmountQuery, DataError>;
}

#[async_trait::async_trait]
pub trait SellTransactionManager {
    async fn process_sell_transaction(&self) -> Result<CurrencyAmountQuery, DataError>;
}

pub enum TransactionType {
    Buy,
    Sell
}

pub struct Transaction {
    transaction_type: TransactionType,
    pool: PgPool,
    sum: i32,
    rate: f32,
    order_issuer_id: i32,
    incoming_currency_id: i32,
    outgoing_currency_id: i32,
}

impl Transaction {
    pub fn new<T: Into<i32>, F: Into<f32>>(
        transaction_type: TransactionType,
        pool: PgPool,
        sum: T,
        rate: F,
        order_issuer_id: T,
        incoming_currency_id: T,
        outgoing_currency_id: T
    ) -> Self {
        Self {
            transaction_type,
            pool,
            sum: sum.into(),
            rate: rate.into(),
            order_issuer_id: order_issuer_id.into(),
            incoming_currency_id: incoming_currency_id.into(),
            outgoing_currency_id: outgoing_currency_id.into()
        }
    }

    async fn get_incoming_currency_wallet(&self, repository: &Repository) -> Result<Option<IncomingCurrencyWallet>, DataError> {
        repository.find_wallet_by_incoming_currency(
            &self.order_issuer_id,
            &self.incoming_currency_id
        ).await
    }

    async fn get_outgoing_currency_wallet(&self, repository: &Repository) -> Result<Option<OutgoingCurrencyWallet>, DataError> {
        repository.find_wallet_by_outgoing_currency(
            &self.order_issuer_id,
            &self.outgoing_currency_id
        ).await
    }
    
    async fn has_wallets(&self) -> Result<bool, DataError> {
        let repository = Repository::new(self.pool.clone());

        let incoming_currency_wallet = self.get_incoming_currency_wallet(&repository).await?;
        let outgoing_currency_wallet = self.get_outgoing_currency_wallet(&repository).await?;

        if !incoming_currency_wallet.is_some() || !outgoing_currency_wallet.is_some() {
            return Err(DataError::WalletNotFoundError("Wallet not found".to_string()))
        }

        Ok(true)
    }
}

#[async_trait::async_trait]
impl BuyTransactionManager for Transaction {
    async fn process_buy_transaction(&self) -> Result<CurrencyAmountQuery, DataError> {
        let outgoing_currency_id = &self.outgoing_currency_id;
        let repository = Repository::new(self.pool.clone());

        let has_wallets = self.has_wallets().await?;
        if has_wallets {
            let outgoing_currency_balance = repository.currency_amount(outgoing_currency_id)
                .await?
                .expect("Error finding outgoing balance by id")
                .amount;
            let outgoing_balance = outgoing_currency_balance.unwrap_or(0);
            let sum_to_pay = (self.sum as f32)* self.rate;
            let sum_to_deduct = sum_to_pay as i32;
            if sum_to_deduct <= outgoing_balance {
                let incoming_currency_wallet_id = self.get_incoming_currency_wallet(&repository)
                    .await?
                    .unwrap()
                    .wallet_id
                    .unwrap();
                let outgoing_currency_wallet_id = self.get_outgoing_currency_wallet(&repository)
                    .await?
                    .unwrap()
                    .wallet_id
                    .unwrap();
                let result = repository.exchange_currencies(
                    self.sum,
                    self.incoming_currency_id,
                    self.outgoing_currency_id,
                    self.rate,
                    incoming_currency_wallet_id,
                    outgoing_currency_wallet_id
                ).await?;
                if result.is_some() {
                    Ok(result.unwrap())
                } else {
                    Err(DataError::CurrencyExchangeError("Error during currency exchange".to_string()))
                }
            } else {
                Err(DataError::WalletBalanceError("Insufficient funds".to_string()))
            }
        } else {
            Err(DataError::WalletNotFoundError("Wallet not found".to_string()))
        }
    }
}

#[async_trait::async_trait]
impl SellTransactionManager for Transaction {
    async fn process_sell_transaction(&self) -> Result<CurrencyAmountQuery, DataError> {
        let outgoing_currency_id = &self.outgoing_currency_id;
        let repository = Repository::new(self.pool.clone());

        let has_wallets = self.has_wallets().await?;
        if has_wallets {
            let outgoing_currency_balance = repository.currency_amount(outgoing_currency_id)
                .await?
                .expect("Error finding outgoing balance by id")
                .amount;
            let outgoing_balance = outgoing_currency_balance.unwrap_or(0);
            let sum_to_pay = (self.sum as f32)* self.rate;
            let sum_to_deduct = sum_to_pay as i32;
            if sum_to_deduct <= outgoing_balance {
                let incoming_currency_wallet_id = self.get_incoming_currency_wallet(&repository)
                    .await?
                    .unwrap()
                    .wallet_id
                    .unwrap();
                let outgoing_currency_wallet_id = self.get_outgoing_currency_wallet(&repository)
                    .await?
                    .unwrap()
                    .wallet_id
                    .unwrap();
                let result = repository.exchange_currencies(
                    self.sum,
                    self.incoming_currency_id,
                    self.outgoing_currency_id,
                    self.rate,
                    incoming_currency_wallet_id,
                    outgoing_currency_wallet_id
                ).await?;
                if result.is_some() {
                    Ok(result.unwrap())
                } else {
                    Err(DataError::CurrencyExchangeError("Error during currency exchange".to_string()))
                }
            } else {
                Err(DataError::WalletBalanceError("Insufficient funds".to_string()))
            }
        } else {
            Err(DataError::WalletNotFoundError("Wallet not found".to_string()))
        }
    }
}