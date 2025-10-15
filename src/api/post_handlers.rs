use actix_web::{post, Responder};
use actix_web::web::{Data, Json};
use sqlx::PgPool;
use crate::datasource::api_models::{CreateUserRequest, CreateUserResponse};
use crate::datasource::repository::repository::Repository;
use crate::datasource::repository::user_repository::UserRepository;
use crate::utils::env_parser::EnvParser;
use crate::utils::jwt::get_token;
use crate::utils::model_mapper::map_user_to_network_model;

#[post("/api/v1/users/create")]
pub async fn create_user(pool: Data<PgPool>, request: Json<CreateUserRequest>) -> impl Responder {
    let repository = Repository::new(pool.get_ref().clone());
    let user = repository.create_user(&request.0)
        .await
        .map_err(|e| { Json(e)})
        .unwrap();
    if user.is_some() {
        let network_user = map_user_to_network_model(&user.unwrap());
        let env_parser = EnvParser::new();
        let token = get_token(&network_user.id, &env_parser).expect("Unable to create token");
        let resp = CreateUserResponse::new(
            None,
            Some(network_user),
            Some(token)
        );
        Json(resp)
    } else {
        let err_message = "Error upon creating user";
        let resp = CreateUserResponse::new(Some(err_message.to_string()), None, None);
        Json(resp)
    }
}