use crate::model_mapper::map_user_to_network_model;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};
use currency_exchange_data::datasource::api_models::{CreateUserRequest, CreateUserResponse, LoginRequest};
use currency_exchange_data::datasource::repository::repository::Repository;
use currency_exchange_data::datasource::repository::user_repository::UserRepository;
use currency_exchange_middleware::env_parser::MiddlewareEnv;
use currency_exchange_middleware::jwt::get_token;
use sqlx::PgPool;

#[post("/api/v1/users/create")]
pub async fn create_user(pool: Data<PgPool>, request: Json<CreateUserRequest>) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let user = repository.create_user(&request.0)
        .await
        .map_err(|e| { Json(e)})
        .unwrap();
    if user.is_some() {
        let network_user = map_user_to_network_model(&user.unwrap());
        let env_parser = MiddlewareEnv::new();
        let token = get_token(&network_user.id, &env_parser).expect("Unable to create token");
        let resp = CreateUserResponse::new(
            None,
            Some(network_user),
            Some(token)
        );
        HttpResponse::Created().json(Json(resp))
    } else {
        let err_message = "Error upon creating user";
        let resp = CreateUserResponse::new(Some(err_message.to_string()), None, None);
        HttpResponse::BadRequest().json(Json(resp))
    }
}

#[post("/api/v1/login")]
pub async fn login(pool: Data<PgPool>, request: Json<LoginRequest>) -> HttpResponse {
    let repository = Repository::new(pool.get_ref().clone());
    let parser = MiddlewareEnv::new();
    let user_option = repository.find_user_by_username(&request.0.username)
        .await
        .expect("Error finding user");
    if user_option.is_some() {
        let user = user_option.unwrap();
        let user_pwd = user.password;
        if user_pwd == request.0.password {
            let id = user.user_id;
            let token = get_token(&id, &parser).expect("Unable to create token");
            HttpResponse::Created().json(Json(token))
        } else {
            HttpResponse::BadRequest().json(Json("Invalid username/password".to_string()))
        }
    } else {
        HttpResponse::BadRequest().json(Json("Invalid username/password".to_string()))
    }
}