use crate::datasource::errors::DataError;
use crate::datasource::models::{BuyOrder, SellOrder};

#[async_trait::async_trait]
pub trait OrderRepository {
    async fn find_buy_orders<S : Into<i64> + Send>(&self, limit: S) -> Result<Vec<BuyOrder>, DataError>;

    async fn find_sell_orders<S: Into<i64> + Send>(&self, limit: S) -> Result<Vec<SellOrder>, DataError>;
}