use actix_web::web::{Data, Json, ReqData};
use actix_web::{HttpRequest, Responder};
use currency_exchange_data::datasource::api_models::{AddCurrencyRequest, CreateCurrencyRequest, CreateWalletRequest};
use currency_exchange_data::datasource::repository::currency_repository::CurrencyRepository;
use currency_exchange_data::datasource::repository::repository::Repository;
use currency_exchange_middleware::jwt::Claims;
use sqlx::PgPool;
use currency_exchange_data::datasource::repository::wallet_repository::WalletRepository;

pub async fn create_currency(
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<CreateCurrencyRequest>
) -> impl Responder {
    let headers = req.headers();
    if let Some(_) = headers.get("Authorization") {
        let json = body.into_inner();
        let repository = Repository::new(pool.get_ref().clone());
        let curr_response = repository.create_currency(&json)
            .await
            .expect("Unable to create currency");
        Json(curr_response)
    } else {
        Json(None)
    }
}

pub async fn create_wallet(
    claims: ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<CreateWalletRequest>
) -> impl Responder {
    let headers = req.headers();
    if let Some(_) = headers.get("Authorization") {
        let uid = claims.sub.parse::<i32>().expect("Invalid sub id");
        let json = body.into_inner();
        if uid == json.user_id {
            let repository = Repository::new(pool.get_ref().clone());
            let wallet = repository.create_wallet(&json).await.expect("Unable to create wallet");
            Json(Some(wallet))
        } else {
            Json(None)
        }
    } else {
        Json(None)
    }
}

pub async fn add_currency_to_wallet(
    claims: ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<AddCurrencyRequest>,
) -> impl Responder {
    let headers = req.headers();
    if let Some(_) = headers.get("Authorization") {
        let uid = claims.sub.parse::<i32>().expect("Invalid sub id");
        let json = body.into_inner();
        if uid == json.user_id {
            let repository = Repository::new(pool.get_ref().clone());
            let resp = repository.add_currency(&json)
                .await
                .expect("Unable to add currency");
            Json(resp)
        } else {
            Json(None)
        }
    } else {
        Json(None)
    }
}