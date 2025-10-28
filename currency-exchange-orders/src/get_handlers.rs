use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::Json;
use sqlx::{PgPool};
use web::{Data, ReqData};
use currency_exchange_data::datasource::api_models::BalanceRequest;
use currency_exchange_data::datasource::repository::repository::Repository;
use currency_exchange_data::datasource::repository::wallet_repository::WalletRepository;
use currency_exchange_data::datasource::responses::{BalanceNotFoundResponse, CurrencyNotFoundResponse, WalletNotFoundResponse};
use currency_exchange_middleware::jwt::Claims;

pub async fn orders(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("You are authenticated! This is protected content.")
}

pub async fn currency_balance(
    req: HttpRequest,
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    body: Json<BalanceRequest>
) -> HttpResponse {
    let headers = req.headers();
    if let Some(_) = headers.get("Authorization") {
        let uid = claims.sub.parse::<i32>().unwrap();
        let repo = Repository::new(pool.as_ref().clone());
        let request = body.into_inner();
        let wallets = repo.find_wallet(&uid)
            .await;
        if wallets.is_none() {
            HttpResponse::NotFound().json(WalletNotFoundResponse::new(format!("Wallet for user {} not found", uid)))
        } else {
            let currency_code = &request.currency_code;
            let currency = repo.find_currency(&currency_code)
                .await;
            if currency.is_none() {
                HttpResponse::NotFound().json(CurrencyNotFoundResponse::new(format!("Currency with code {} not found", currency_code)))
            } else {
                let balance = repo.get_currency_balance(&request)
                    .await
                    .expect("Currency balance error");
                if balance.is_none() {
                    HttpResponse::NotFound().json(BalanceNotFoundResponse::new(format!("Balance for currency with code {} not found", currency_code)))
                } else {
                    HttpResponse::Ok().json(balance.unwrap())
                }
            }
        }
    } else {
        HttpResponse::BadRequest().json("You don't have authorization.")
    }
}