use crate::datasource::currency_repository::CurrencyRepository;
use crate::datasource::user_repository::UserRepository;
use crate::env_parser::EnvParser;
use crate::middleware::jwt::{get_token, Claims};
use crate::models::auth_responses::error_responses::UserNotFound;
use crate::models::auth_responses::success_responses::LoggedInUser;
use crate::models::{CreateCurrencyRequest, CreateUserRequest, CreateWalletRequest, LoginUserRequest};
use crate::repository::Repository;
use actix_web::web::{Data, Json, ReqData};
use actix_web::{post, HttpResponse};
use sqlx::PgPool;
use crate::datasource::wallet_repository::WalletRepository;

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
        .await
        .expect("Error creating wallet");
    HttpResponse::Created().json(wallet)
}