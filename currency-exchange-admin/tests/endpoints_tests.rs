use actix_web::{test, web, App};
use actix_web::web::Data;
use fake::{Dummy, Fake, Faker};
use currency_exchange_admin::post_handlers::{add_currency_to_wallet, create_currency, create_wallet};
use currency_exchange_admin::server::AdminEnv;
use currency_exchange_data::datasource::api_models::{AddCurrencyRequest, CreateCurrencyRequest, CreateWalletRequest};
use currency_exchange_middleware::database_connector::DatabaseConnector;
use currency_exchange_middleware::env_parser::{EnvParser, MiddlewareEnv};
use currency_exchange_middleware::jwt::get_token;
use currency_exchange_middleware::middleware::JwtMiddleware;

#[actix_web::test]
async fn create_currency_must_succeed() {
    let dummy_request: DummyCreateCurrencyRequest = Faker.fake_with_rng(&mut rand::rng());
    let req = CreateCurrencyRequest {
        currency_code: dummy_request.currency_code.chars().take(3).collect(),
    };
    let parser = AdminEnv::new();
    let connector = DatabaseConnector::new(
        parser.database_url(),
        parser.max_connections()
    );
    let pool = connector.connect().await;
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                web::resource("/api/v1/currencies/create")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_currency)),
            )
    ).await;
    let middleware_env = MiddlewareEnv::new();
    let jwt_token = get_token(&1, &middleware_env).unwrap();
    let req = test::TestRequest::post()
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .uri("/api/v1/currencies/create")
        .set_json(&req)
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    sqlx::query("DELETE FROM currencies").execute(&pool).await.unwrap();
}

#[derive(Debug, Dummy, Clone)]
struct DummyCreateCurrencyRequest {
    pub currency_code: String,
}
