use serde::{Deserialize, Serialize};
use crate::datasource::models::Currency;

#[derive(Serialize, Deserialize)]
pub struct WalletNotFoundResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyNotFoundResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceNotFoundResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBuyOrderResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSellOrderResponse {
    message: String,
}

impl WalletNotFoundResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into()}
    }
}

impl CurrencyNotFoundResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into()}
    }
}

impl BalanceNotFoundResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into()}
    }
}

impl CreateBuyOrderResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into()}
    }
}

impl CreateSellOrderResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into()}
    }
}