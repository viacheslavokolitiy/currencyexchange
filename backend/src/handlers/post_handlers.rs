use crate::datasource::buy_orders_repository::BuyOrdersRepository;
use crate::datasource::currency_exchange_ratio_repository::CurrencyExchangeRatioRepository;
use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::sell_orders_repository::SellOrdersRepository;
use crate::datasource::user_repository::UserRepository;
use crate::datasource::wallet_repository::WalletRepository;
use crate::env_parser::EnvParser;
use crate::error_responses::CurrencyExchangeRatesCreateFailed;
use crate::middleware::jwt::{get_token, Claims};
use crate::models::auth_responses::error_responses::UserNotFound;
use crate::models::auth_responses::success_responses::LoggedInUser;
use crate::models::{CreateBuyOrderRequest, CreateCurrencyRequest, CreateExchangeRateRequest, CreateSellOrderRequest, CreateUserRequest, CreateWalletRequest, LoginUserRequest};
use crate::repository::Repository;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{post, HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::error::Error;

#[post("/api/v1/user/create")]
pub async fn create_user(pool: Data<PgPool>, request: Json<CreateUserRequest>) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let req = repository.create_user(&request.0)
        .await
        .expect("Error creating user");
    if req.is_some() {
        HttpResponse::Created().json(req.unwrap())
    } else {
        HttpResponse::BadRequest().json(req.expect("Unable to create user"))
    }
}

#[post("/api/v1/user/login")]
pub async fn login_user(pool: Data<PgPool>, json: Json<LoginUserRequest>) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let user_exists = repository.check_if_user_exists(&json.username)
        .await
        .expect("Unable to check if user exists");
    if user_exists.is_none() {
        HttpResponse::NotFound().json(UserNotFound::new(String::from("User not found")))
    } else {
        let uid = user_exists.unwrap().id;
        let pwd = &json.password;
        let passwords_match = repository.verify_password(uid, &pwd)
            .await
            .expect("Unable to verify password");
        if !passwords_match {
            HttpResponse::NotFound().json(UserNotFound::new(String::from("User not found")))
        } else {
            let parser = EnvParser::new();
            let token = get_token(&uid, &parser)
                .expect("Unable to get token");
            let logged_user = LoggedInUser::new(token);
            HttpResponse::Ok().json(logged_user)
        }
    }
}

pub async fn create_currency(
    pool: Data<PgPool>,
    request: Json<CreateCurrencyRequest>,
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let code = &request.currency_code;
    let query = repository.create_new_currency(code)
        .await
        .expect("Error creating currency");
    HttpResponse::Created().json(query)
}

pub async fn create_wallet(
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    req: Json<CreateWalletRequest>,
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let uid = claims.sub.parse::<i32>().unwrap();
    let currency = req.0.currency_code;
    let wallet = repository.create_wallet(&uid, &currency)
        .await;
    if let Ok(wallet) = wallet {
        HttpResponse::Created().json(wallet)
    } else {
        HttpResponse::Conflict().json(wallet.err().unwrap().to_string())
    }
}

pub async fn create_exchange_rate(
    pool: Data<PgPool>,
    req: Json<CreateExchangeRateRequest>,
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let first_currency_code = &req.first_currency_code;
    let second_currency_code = &req.second_currency_code;
    let first_currency_value = req.first_currency_value;
    let second_currency_value = req.second_currency_value;

    let resp = repository.add_exchange_ratio(first_currency_code, second_currency_code, first_currency_value, second_currency_value)
        .await;
    if let Ok(res) = resp {
        if res.is_none() {
            HttpResponse::BadRequest().json(
                CurrencyExchangeRatesCreateFailed::new("Error during exchange rate creation", (first_currency_code, second_currency_code)),
            )
        } else {
            HttpResponse::Created().json(res.unwrap())
        }
    } else {
        HttpResponse::Conflict().json(
            CurrencyExchangeRatesCreateFailed::new(
                resp.err().unwrap().to_string(), 
                (first_currency_code.to_string(), second_currency_code.to_string()))
        )
    }
}

pub async fn post_new_buy_order(
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    req: Json<CreateBuyOrderRequest>,
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let uid = claims.sub.parse::<i32>().unwrap();
    let resp = repository.create_buy_order(
        &uid,
        &req.0
    ).await;
    produce_new_order_response(resp, HttpResponse::Created(), HttpResponse::BadRequest())
}

pub async fn post_new_sale_order(
    claims: ReqData<Claims>,
    pool: Data<PgPool>,
    req: Json<CreateSellOrderRequest>
) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let uid = claims.sub.parse::<i32>().unwrap();
    let resp = repository.create_sell_order(
        &uid,
        &req.0
    ).await;
    produce_new_order_response(resp, HttpResponse::Created(), HttpResponse::BadRequest())
}

fn produce_new_order_response<'a, T>(
    response: Result<Option<T>, Box<dyn Error>>,
    mut success_builder: HttpResponseBuilder,
    mut error_builder: HttpResponseBuilder
) -> HttpResponse
where
    T: Serialize + Deserialize<'a> {

    if let Ok(res) = response {
        success_builder.json(res)
    } else {
        error_builder.json(response.err().unwrap().to_string())
    }
}