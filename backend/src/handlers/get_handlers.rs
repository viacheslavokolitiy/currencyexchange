use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::user_repository::UserRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::middleware::jwt::Claims;
use crate::models::auth_responses::success_responses::{UsernameAlreadyTaken, UsernameAvailable};
use crate::models::{CurrencyByCodeParams, CurrencyExchangeRatesParams, UsernameCheckParams, WalletByCurrencyCodeParams};
use crate::repository::Repository;
use actix_web::web::{Data, Query, ReqData};
use actix_web::{get, HttpResponse};
use sqlx::PgPool;
use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
use crate::models::error_responses::CurrencyExchangeRatesNotFound;

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

pub async fn find_wallet(
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    query: Query<WalletByCurrencyCodeParams>
) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let code = &query.code;
    let uid = claims.sub.parse::<i32>().unwrap();
    let check_wallet = repo.check_if_wallet_exists(&uid, code)
        .await
        .expect("Error checking if wallet exists");
    HttpResponse::Ok().json(check_wallet)
}

pub async fn find_exchange_rates(
    pool: Data<PgPool>,
    query: Query<CurrencyExchangeRatesParams>,
) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let first_currency_code = &query.first;
    let second_currency_code = &query.second;
    let resp = repo.find_exchange_ratio_by_codes(
        first_currency_code, 
        second_currency_code
    ).await.expect("Error finding exchange rates");
    if resp.is_none() { 
        HttpResponse::NotFound().json(
            CurrencyExchangeRatesNotFound::new("Exchange rate not found", (first_currency_code, second_currency_code))
        )
    } else { 
        HttpResponse::Ok().json(resp.unwrap())
    }
}