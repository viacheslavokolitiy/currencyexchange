use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query};
use sqlx::PgPool;
use crate::models::auth_responses::success_responses::{UsernameAlreadyTaken, UsernameAvailable};
use crate::models::UsernameCheckParams;
use crate::repository::{Repository, UserRepository};

#[get("/api/v1/users")]
pub async fn is_username_taken(pool: Data<PgPool>, query: Query<UsernameCheckParams>) -> HttpResponse {
    let repo = Repository::new(pool.as_ref().clone());
    let is_taken = repo.check_if_exists(query.name.as_str())
        .await
        .expect("Error checking if user already taken");
    if is_taken.is_some() { 
        HttpResponse::Found().json(UsernameAlreadyTaken::new("Username already exists".to_string()))
    } else {
        HttpResponse::Ok().json(UsernameAvailable::new("Username available".to_string()))
    }
}