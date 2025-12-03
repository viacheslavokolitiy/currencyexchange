use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use crate::env_parser::EnvParser;
use crate::middleware::jwt::get_token;
use crate::models::{CreateUserRequest, LoginUserRequest};
use crate::models::auth_responses::error_responses::UserNotFound;
use crate::models::auth_responses::success_responses::LoggedInUser;
use crate::repository::{Repository, UserRepository};

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
    let user_exists = repository.check_if_exists(&json.username)
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