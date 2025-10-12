use crate::datasource::models::CreateUserRequest;
use crate::datasource::repository::Repository;
use crate::datasource::user_repository::UserRepository;
use actix_web::web::Json;
use actix_web::{get, post, web, Responder};
use sqlx::PgPool;
use web::Data;

#[post("/api/v1/users")]
pub async fn create_user(pool: Data<PgPool>, body: Json<CreateUserRequest>) -> impl Responder {
    let repo = Repository::new(pool.as_ref().clone());
    let create_request = body.into_inner();
    let response = repo.create_user(&create_request).await.expect("Unable to create user");
    Json(response)
}

#[get("/api/v1/users")]
pub async fn get_users(pool: Data<PgPool>) -> impl Responder {
    let repo = Repository::new(pool.as_ref().clone());
    let response = repo.find_users().await.expect("Unable to find user");
    Json(response)
}