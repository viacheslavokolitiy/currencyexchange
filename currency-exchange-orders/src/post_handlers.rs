use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use currency_exchange_data::datasource::api_models::{CreateBuyOrderRequest, CreateSellOrderRequest};
use currency_exchange_data::datasource::error_responses::{CreateBuyOrderResponse, CreateSellOrderResponse};
use currency_exchange_data::datasource::repository::order_repository::OrderRepository;
use currency_exchange_data::datasource::repository::repository::Repository;
use currency_exchange_middleware::jwt::Claims;

pub async fn create_buy_order(
    claims: web::ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<CreateBuyOrderRequest>,
) -> HttpResponse {
    let headers = req.headers();
    if let Some(header) = headers.get("Authorization") {
        let uid = claims.sub.parse::<i32>().expect("Unable to parse user id");
        let json = body.into_inner();
        if uid == json.issuer_id {
            let repo = Repository::new(pool.get_ref().clone());
            let resp = repo.create_buy_order(&json)
                .await
                .map_err(|_e| HttpResponse::InternalServerError().finish());
            if let Ok(data) = resp {
                HttpResponse::Created().json(data)
            } else {
                HttpResponse::BadRequest().json(CreateBuyOrderResponse::new("Failed to create order"))
            }
        } else { 
            HttpResponse::BadRequest().json(CreateBuyOrderResponse::new("Invalid user id"))
        }
    } else {
        HttpResponse::BadRequest().body("No Authorization Header")
    }
}

pub async fn create_sell_order(
    claims: web::ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<CreateSellOrderRequest>,
) -> HttpResponse {
    let headers = req.headers();
    if let Some(header) = headers.get("Authorization") {
        let uid = claims.sub.parse::<i32>().expect("Unable to parse user id");
        let json = body.into_inner();
        if uid == json.issuer_id {
            let repo = Repository::new(pool.get_ref().clone());
            let resp = repo.create_sell_order(&json)
                .await
                .map_err(|_e| HttpResponse::InternalServerError().finish());
            if let Ok(data) = resp {
                HttpResponse::Created().json(data)
            } else {
                HttpResponse::BadRequest().json(CreateSellOrderResponse::new("Failed to create order"))
            }
        } else {
            HttpResponse::BadRequest().json(CreateSellOrderResponse::new("Invalid user id"))
        }
    } else {
        HttpResponse::BadRequest().body("No Authorization Header")
    }
}