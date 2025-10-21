use actix_web::{HttpRequest, HttpResponse};

pub async fn orders(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("You are authenticated! This is protected content.")
}