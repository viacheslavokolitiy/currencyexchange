use std::error::Error;
use crate::models::{BuyOrder, CreateBuyOrderRequest};

#[async_trait::async_trait]
pub trait BuyOrdersRepository {
    async fn fetch_buy_orders(&self) -> Result<Vec<BuyOrder>, Box<dyn Error>>;

    async fn create_buy_order(&self, order: &CreateBuyOrderRequest) -> Result<Option<BuyOrder>, Box<dyn Error>>;
}