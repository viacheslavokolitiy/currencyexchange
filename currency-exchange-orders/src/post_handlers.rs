use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use currency_exchange_middleware::jwt::Claims;
use crate::models::CreateBuyOrderRequest;

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

        HttpResponse::Created().body("You are authenticated!")
    } else {
        HttpResponse::BadRequest().body("No Authorization Header")
    }
}