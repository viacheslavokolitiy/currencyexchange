use std::error::Error;
use crate::models::Fee;
use crate::repository::Repository;

#[async_trait::async_trait]
pub trait FeesRepository {
    async fn get_fees(&self) -> Result<Option<Fee>, Box<dyn Error>>;

    async fn add_fee(
        &self,
        exchange_comission: f32,
        state_tax: f32,
        sales_tax: f32
    ) -> Result<(), Box<dyn Error>>;

    async fn update_comission(
        &self,
        new_comission: f32,
    ) -> Result<(), Box<dyn Error>>;

    async fn update_state_tax(
        &self,
        new_state_tax: f32,
    ) -> Result<(), Box<dyn Error>>;

    async fn update_sales_tax(
        &self,
        new_sales_tax: f32,
    ) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl FeesRepository for Repository {
    async fn get_fees(&self) -> Result<Option<Fee>, Box<dyn Error>> {
        let query = sqlx::query_as!(Fee, "SELECT * FROM fees")
            .fetch_optional(&self.pool)
            .await?;
        Ok(query)
    }

    async fn add_fee(
        &self,
        exchange_comission: f32,
        state_tax: f32,
        sales_tax: f32
    ) -> Result<(), Box<dyn Error>> {
        let result = self.get_fees().await?;
        if result.is_none() {
            let query = sqlx::query_as!(Fee,
                    "INSERT INTO fees(exchange_comission, state_tax, sales_tax) VALUES ($1, $2, $3)", exchange_comission, state_tax, sales_tax)
                .execute(&self.pool)
                .await?;
            if query.rows_affected() > 0 {
                Ok(())
            } else {
                Err("Unable to add fee to fees".into())
            }
        } else {
            Err("Fee already exists".into())
        }
    }

    async fn update_comission(&self, new_comission: f32) -> Result<(), Box<dyn Error>> {
        let result = self.get_fees().await?;
        if result.is_some() {
            let fee = result.unwrap();
            let fee_id = fee.id.unwrap();
            let query = sqlx::query_as!(Fee,
            "UPDATE fees SET exchange_comission = $1 WHERE id = $2", new_comission, fee_id)
                .execute(&self.pool).await?;
            if query.rows_affected() > 0 {
                Ok(())
            } else {
                Err("Unable to update commission".into())
            }
        } else {
            Err("Fee does not exist".into())
        }
    }

    async fn update_state_tax(&self, new_state_tax: f32) -> Result<(), Box<dyn Error>> {
        let result = self.get_fees().await?;
        if result.is_some() {
            let fee = result.unwrap();
            let fee_id = fee.id.unwrap();
            let query = sqlx::query_as!(Fee,
            "UPDATE fees SET state_tax = $1 WHERE id = $2", new_state_tax, fee_id)
                .execute(&self.pool).await?;
            if query.rows_affected() > 0 {
                Ok(())
            } else {
                Err("Unable to update state tax".into())
            }
        } else {
            Err("Fee does not exist".into())
        }
    }

    async fn update_sales_tax(&self, new_sales_tax: f32) -> Result<(), Box<dyn Error>> {
        let result = self.get_fees().await?;
        if result.is_some() {
            let fee = result.unwrap();
            let fee_id = fee.id.unwrap();
            let query = sqlx::query_as!(Fee,
            "UPDATE fees SET sales_tax = $1 WHERE id = $2", new_sales_tax, fee_id)
                .execute(&self.pool).await?;
            if query.rows_affected() > 0 {
                Ok(())
            } else {
                Err("Unable to update sales tax".into())
            }
        } else {
            Err("Fee does not exist".into())
        }
    }
}