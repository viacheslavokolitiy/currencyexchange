use std::error::Error;
use crate::models::BuyOrder;

#[async_trait::async_trait]
pub trait BuyOrdersRepository {
    async fn fetch_buy_orders(&self) -> Result<Vec<BuyOrder>, Box<dyn Error>>;
}