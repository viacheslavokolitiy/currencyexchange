use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("JWT generation error")]
    JwtError,
}