use std::env;
use std::net::TcpListener;
use std::path::PathBuf;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use currency_exchange_middleware::database_connector::DatabaseConnector;
use currency_exchange_middleware::env_parser::EnvParser;
use currency_exchange_middleware::middleware::JwtMiddleware;
use currency_exchange_middleware::tracing_middleware::NetworkLogSpanBuilder;
use crate::post_handlers::{add_currency_to_wallet, create_currency, create_wallet};

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";
const ENV_HOST: &str = "SERVER_HOST";

const ENV_PORT: &str = "SERVER_PORT";

pub struct AdminEnv {
    env: PathBuf
}

impl AdminEnv {
    pub fn new() -> Self {
        Self {
            env: dotenvy::from_filename("./currency-exchange-admin/.env").expect("Failed to load .env file!")
        }
    }
}

impl EnvParser for AdminEnv {
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
    env_parser: AdminEnv,
}

impl Server {
    pub fn new(env_parser: AdminEnv) -> Self {
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
                web::resource("/api/v1/currencies/create")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_currency)),
            )
            .service(
                web::resource("/api/v1/wallet/currencies/add")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(add_currency_to_wallet))
            )
            .service(
                web::resource("/api/v1/wallet/create")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(create_wallet))
            ))
            .listen(listener)?
            .run()
            .await
    }
}