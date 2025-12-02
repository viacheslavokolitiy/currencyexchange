use actix_cors::Cors;
use actix_web::{App, HttpServer};
use currency_exchange_middleware::env_parser::EnvParser;
use currency_exchange_swagger::swagger;
use currency_exchange_swagger::swagger_env::SwaggerEnv;
use swagger::__path_buy_orders;
use swagger::__path_sell_orders;
use swagger::__path_register;
use swagger::__path_login;
use swagger::__path_currencies;
use swagger::__path_create_currency;
use swagger::__path_add_currency_to_wallet;
use swagger::__path_sell_currency;
use swagger::__path_create_sell_order;
use swagger::__path_create_buy_order;
use swagger::__path_create_new_wallet;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Default, OpenApi)]
#[openapi(
    paths(buy_orders, sell_orders, register, login, currencies, create_currency, add_currency_to_wallet, sell_currency, create_sell_order, create_buy_order, create_new_wallet),
    servers(
        (url="http://localhost:8081", description="Order server"),
        (url="http://localhost:8080", description="Auth server"),
        (url="http://localhost:8082", description="User related features server")
    ),
    modifiers(&BearerAuthModifier),
    security(
        ("bearer_auth" = [])
    )
)]
struct ApiDoc;

struct BearerAuthModifier;

impl Modify for BearerAuthModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let parser = SwaggerEnv::new();
    let port = parser.port();
    let host = parser.host();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
        })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
