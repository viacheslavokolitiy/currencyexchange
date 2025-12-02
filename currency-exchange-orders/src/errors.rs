use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationErrors {
    #[error("Unable to set up order. Wallet does not have enough funds")]
    InsufficientProvidedCurrencyError
}