use actix_web::HttpResponse;
use actix_web::web::{Data, Json, ReqData};
use sqlx::PgPool;
use crate::datasource::wallet_repository::WalletRepository;
use crate::middleware::jwt::Claims;
use crate::models::{ReplenishBalanceRequest, ReplenishBalanceResponse};
use crate::repository::Repository;

pub async fn replenish_balance(
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    req: Json<ReplenishBalanceRequest>
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let uid = claims.sub.parse::<i32>().unwrap();
    let currency = req.0.currency_code;
    let amount = req.0.amount;
    let resp = repository.replenish_wallet_balance(&uid, &amount, &currency)
        .await;
    if let Ok(_) = resp {
        HttpResponse::Ok().json(
            ReplenishBalanceResponse::new(
                format!("Your balance in {} was replenished for {} {}", &currency, &amount, &currency),
            )
        )
    } else {
        HttpResponse::BadRequest().json(
            ReplenishBalanceResponse::new(resp.err().unwrap().to_string())
        )
    }
}