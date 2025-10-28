use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("{0}")]
    EntryAlreadyExists(String),
    #[error("{0}")]
    WalletCreationError(String),
    #[error("{0}")]
    WalletNotFoundError(String),
    #[error("{0}")]
    WalletBalanceError(String),
}