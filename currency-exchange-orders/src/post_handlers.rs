use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use currency_exchange_middleware::jwt::Claims;
use crate::models::CreateOrderRequest;

pub async fn create_order(
    claims: web::ReqData<Claims>,
    req: HttpRequest,
    pool: Data<PgPool>,
    body: Json<CreateOrderRequest>
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

pub mod post_handlers_helper {
    use sqlx::PgPool;
    use crate::errors::ValidationErrors;

    fn validate_offered_currency<T, T1, T2>(
        pool: PgPool,
        uid: T,
        offered_amount: T,
        offered_code: T1
    ) -> Result<(), ValidationErrors>
    where
        T: Into<i32>,
        T1: Into<i32>,
        T2: Into<String> {

        todo!()
    }
}