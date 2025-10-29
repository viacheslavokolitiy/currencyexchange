use std::env;
use std::net::TcpListener;
use std::path::PathBuf;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use currency_exchange_middleware::database_connector::DatabaseConnector;
use currency_exchange_middleware::env_parser::EnvParser;
use currency_exchange_middleware::middleware::{JwtMiddleware};
use currency_exchange_middleware::tracing_middleware::NetworkLogSpanBuilder;
use crate::get_handlers::{buy_orders, currency_balance, sell_orders};
use crate::order_endpoints::{GET_BUY_ORDERS, GET_MY_BALANCE, GET_SELL_ORDERS, POST_NEW_BUY_ORDER, POST_NEW_SELL_ORDER, PUT_BUY_CURRENCY, PUT_SELL_CURRENCY};
use crate::post_handlers::{create_buy_order, create_sell_order};
use crate::put_handlers::{buy_currency, sell_currency};

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";
const ENV_HOST: &str = "SERVER_HOST";

const ENV_PORT: &str = "SERVER_PORT";

pub struct OrdersEnv {
    env: PathBuf,
}

impl OrdersEnv {
    pub fn new() -> Self {
        Self {
            env: dotenvy::from_filename("./currency-exchange-orders/.env").expect("Cannot load env file"),
        }
    }
}

impl EnvParser for OrdersEnv {
    fn database_url(&self) -> String {
        env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set")
    }

    fn max_connections(&self) -> u32 {
        env::var(ENV_MAX_CONNECTIONS).expect("MAX_CONNECTIONS must be set")
            .parse::<u32>().expect("MAX_CONNECTIONS must be a number")
    }

    fn host(&self) -> String {
        env::var(ENV_HOST).expect("SERVER_HOST must be set")
    }

    fn port(&self) -> String {
        env::var(ENV_PORT).expect("SERVER_PORT must be set")
    }
}

pub struct Server {
    env_parser: OrdersEnv,
}

impl Server {
    pub fn new(env_parser: OrdersEnv) -> Self {
        Self { env_parser }
    }

    pub async fn start_server(&self) -> std::io::Result<()> {
        let connector = DatabaseConnector::new(
            self.env_parser.database_url(),
            self.env_parser.max_connections()
        );

        let pool = connector.connect().await;
        let host = self.env_parser.host();
        let port = self.env_parser.port();
        println!("Listening on {}:{}", host, port);
        let listener =
            TcpListener::bind(format!("{}:{}", host, port)).expect("Couldn't bind to port");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        HttpServer::new(move || App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(NetworkLogSpanBuilder::new().middleware().clone())
            .service(
                web::resource(GET_BUY_ORDERS)
                    .wrap(JwtMiddleware)
                    .route(web::get().to(buy_orders)),
            )
            .service(
                web::resource(GET_SELL_ORDERS)
                    .wrap(JwtMiddleware)
                    .route(web::get().to(sell_orders))
            )
            .service(
                web::resource(GET_MY_BALANCE)
                    .wrap(JwtMiddleware)
                    .route(web::get().to(currency_balance))
            )
            .service(
                web::resource(POST_NEW_BUY_ORDER)
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_buy_order)),
            )
            .service(
                web::resource(POST_NEW_SELL_ORDER)
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_sell_order)),
            )
            .service(
                web::resource(PUT_BUY_CURRENCY)
                    .wrap(JwtMiddleware)
                    .route(web::put().to(buy_currency))
            )
            .service(
                web::resource(PUT_SELL_CURRENCY)
                    .wrap(JwtMiddleware)
                    .route(web::put().to(sell_currency))
            ))
            .listen(listener)?
            .run()
            .await
    }
}