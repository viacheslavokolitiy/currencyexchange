use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use currency_exchange_data::datasource::api_models::ExchangeCurrencyRequest;
use currency_exchange_middleware::jwt::Claims;
use crate::order_transaction_manager::{BuyTransactionManager, SellTransactionManager, Transaction, TransactionType};

pub async fn buy_currency(
    claims: web::ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<ExchangeCurrencyRequest>
) -> HttpResponse {
    let json = body.into_inner();
    let headers = req.headers();
    if let Some(id) = headers.get("Authorization") {
        if claims.sub.parse::<i32>().unwrap() == json.order_issuer_id {
            let tx = Transaction::new(
                TransactionType::Buy,
                pool.get_ref().clone(),
                json.sum,
                json.rate,
                json.order_issuer_id,
                json.incoming_currency_id,
                json.outgoing_currency_id
            );
            let query = tx.process_buy_transaction()
                .await
                .expect("Transaction processing error");
            HttpResponse::Ok().json(query)
        } else {
            HttpResponse::Unauthorized().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
pub async fn sell_currency(
    claims: web::ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<ExchangeCurrencyRequest>
) -> HttpResponse {
    let json = body.into_inner();
    let headers = req.headers();
    if let Some(id) = headers.get("Authorization") {
        if claims.sub.parse::<i32>().unwrap() == json.order_issuer_id {
            let tx = Transaction::new(
                TransactionType::Buy,
                pool.get_ref().clone(),
                json.sum,
                json.rate,
                json.order_issuer_id,
                json.incoming_currency_id,
                json.outgoing_currency_id
            );
            let query = tx.process_sell_transaction()
                .await
                .expect("Transaction processing error");
            HttpResponse::Ok().json(query)
        } else {
            HttpResponse::Unauthorized().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}