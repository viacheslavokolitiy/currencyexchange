use actix_web::web::Data;
use actix_web::{test, App};
use fake::{Dummy, Fake, Faker};
use currency_exchange_middleware::env_parser::{EnvParser, JwtEnvParser, MiddlewareEnv};
use currency_exchange_auth::post_handlers::{create_user, login};
use currency_exchange_data::datasource::api_models::{CreateUserRequest, LoginRequest};
use currency_exchange_middleware::database_connector::DatabaseConnector;

#[actix_web::test]
async fn create_user_should_succeed() {
    let dummy_user: DummyCreateUser = Faker.fake();
    let user = CreateUserRequest {
        username: dummy_user.username.chars().take(8).collect(),
        email: dummy_user.email,
        password: dummy_user.password,
        firstname: dummy_user.firstname,
        middlename: Some(dummy_user.middlename),
        lastname: dummy_user.lastname,
    };

    let parser = MiddlewareEnv::new();
    let connector = DatabaseConnector::new(
        parser.database_url(),
        parser.max_connections()
    );

    let pool = connector.connect().await;
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(create_user)
    ).await;
    let req = test::TestRequest::post().uri("/api/v1/users/create")
        .set_json(&user)
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    sqlx::query("DELETE FROM users").execute(&pool).await.unwrap();
}

#[actix_web::test]
async fn login_should_succeed() {
    let dummy_user: DummyCreateUser = Faker.fake();
    let user = CreateUserRequest {
        username: dummy_user.username.chars().take(8).collect(),
        email: dummy_user.email,
        password: dummy_user.password,
        firstname: dummy_user.firstname,
        middlename: Some(dummy_user.middlename),
        lastname: dummy_user.lastname,
    };

    let parser = MiddlewareEnv::new();
    let connector = DatabaseConnector::new(
        parser.database_url(),
        parser.max_connections()
    );

    let pool = connector.connect().await;
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(create_user)
            .service(login)
    ).await;
    let req = test::TestRequest::post().uri("/api/v1/users/create")
        .set_json(&user)
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let login_req = test::TestRequest::post().uri("/api/v1/login")
        .set_json(LoginRequest {
            username: user.username,
            password: user.password,
        }).to_request();
    let login_res = test::call_service(&app, login_req).await;
    assert!(login_res.status().is_success());

    sqlx::query("DELETE FROM users").execute(&pool).await.unwrap();
}

#[derive(Debug, Dummy, Clone)]
pub struct DummyCreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
}