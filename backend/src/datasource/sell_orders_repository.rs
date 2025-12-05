use crate::models::SellOrder;
use std::error::Error;

#[async_trait::async_trait]
pub trait SellOrdersRepository {

    async fn fetch_sell_orders(&self) -> Result<Vec<SellOrder>, Box<dyn Error>>;
}