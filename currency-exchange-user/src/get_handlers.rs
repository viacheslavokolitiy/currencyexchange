use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::{Data};
use sqlx::PgPool;
use currency_exchange_data::datasource::repository::currency_repository::CurrencyRepository;
use currency_exchange_data::datasource::repository::repository::Repository;

pub async fn currencies(
    req: HttpRequest,
    pool: Data<PgPool>
) -> HttpResponse {
    let headers = req.headers();
    if let Some(_) = headers.get("Authorization") {
        let repository = Repository::new(pool.get_ref().clone());
        let currencies = repository.all_currencies()
            .await
            .expect("Error getting all_currencies");
        HttpResponse::Ok().json(currencies)
    } else {
        HttpResponse::Unauthorized().body("Authorization header missing")
    }
}