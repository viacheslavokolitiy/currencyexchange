use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CurrencyExchangeRatesCreateFailed {
    pub message: String,
    pub pair: (String, String),
}

impl CurrencyExchangeRatesCreateFailed {
    pub fn new<S: Into<String>>(message: S, pair: (S, S)) -> Self {
        Self {
            message: message.into(),
            pair: (pair.0.into(), pair.1.into())
        }
    }
}