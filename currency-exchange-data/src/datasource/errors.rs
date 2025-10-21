use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("{0}")]
    EntryAlreadyExists(String),
}