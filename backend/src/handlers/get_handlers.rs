use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::user_repository::UserRepository;
use crate::models::auth_responses::success_responses::{UsernameAlreadyTaken, UsernameAvailable};
use crate::models::{CurrencyByCodeParams, UsernameCheckParams};
use crate::repository::Repository;
use actix_web::web::{Data, Query};
use actix_web::{get, HttpResponse};
use sqlx::PgPool;

#[get("/api/v1/users")]
pub async fn is_username_taken(pool: Data<PgPool>, query: Query<UsernameCheckParams>) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let is_taken = repo.check_if_user_exists(query.name.as_str())
        .await
        .expect("Error checking if user already taken");
    if is_taken.is_some() { 
        HttpResponse::Found().json(UsernameAlreadyTaken::new("Username already exists"))
    } else {
        HttpResponse::Ok().json(UsernameAvailable::new("Username available"))
    }
}

pub async fn get_currency_by_code(
    pool: Data<PgPool>,
    query: Query<CurrencyByCodeParams>,
) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let code = &query.code;
    let currency_by_code = repo.find_currency_by_code(code)
        .await
        .expect("Error finding currency by code");
    HttpResponse::Ok().json(currency_by_code)
}

pub async fn get_all_currencies(
    pool: Data<PgPool>
) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let currencies = repo.find_all_currencies()
        .await
        .expect("Error fetching currencies");
    HttpResponse::Ok().json(currencies)
}