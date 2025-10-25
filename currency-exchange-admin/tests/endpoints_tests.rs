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
}

#[actix_web::test]
async fn add_currency_to_wallet_must_succeed() {
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
                web::resource("/api/v1/wallet/currencies/add")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(add_currency_to_wallet)),
            )
            .service(
                web::resource("/api/v1/wallet/create")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_wallet)),
        )
    ).await;
    let middleware_env = MiddlewareEnv::new();
    let jwt_token = get_token(&1, &middleware_env).unwrap();
    let curr_req = AddCurrencyRequest {
        user_id: 1,
        currency_id: 1
    };
    let curr_req_two = AddCurrencyRequest {
        user_id: 1,
        currency_id: 2
    };
    let curr_req_three = AddCurrencyRequest {
        user_id: 1,
        currency_id: 3
    };
    let create_wallet_req = CreateWalletRequest {
        user_id: 1,
        currency_id: 1
    };
    let wallet_req = test::TestRequest::post()
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .uri("/api/v1/wallet/create")
        .set_json(&create_wallet_req)
        .to_request();
    let wallet_resp = test::call_service(&app, wallet_req).await;
    assert!(wallet_resp.status().is_success());

    let req = test::TestRequest::post()
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .uri("/api/v1/wallet/currencies/add")
        .set_json(&curr_req)
        .to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let req_two = test::TestRequest::post()
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .uri("/api/v1/wallet/currencies/add")
        .set_json(&curr_req_two)
        .to_request();
    let res = test::call_service(&app, req_two).await;
    assert!(res.status().is_success());

    let req_three = test::TestRequest::post()
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)))
        .uri("/api/v1/wallet/currencies/add")
        .set_json(&curr_req_three)
        .to_request();
    let res = test::call_service(&app, req_three).await;
    assert!(res.status().is_success());
}

#[derive(Debug, Dummy, Clone)]
struct DummyCreateCurrencyRequest {
    pub currency_code: String,
}
