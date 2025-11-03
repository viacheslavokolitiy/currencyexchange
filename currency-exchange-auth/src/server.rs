use crate::post_handlers::{create_user, login};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use currency_exchange_middleware::database_connector::DatabaseConnector;
use currency_exchange_middleware::env_parser::{JwtEnvParser, MiddlewareEnv};
use currency_exchange_middleware::tracing_middleware::NetworkLogSpanBuilder;
use std::net::TcpListener;
use actix_cors::Cors;

pub struct Server {
    env_parser: MiddlewareEnv,
}

impl Server {
    pub fn new(env_parser: MiddlewareEnv) -> Self {
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
        let listener =
            TcpListener::bind(format!("{}:{}", host, port)).expect("Couldn't bind to port");
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        HttpServer::new(move || App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .wrap(NetworkLogSpanBuilder::new().middleware().clone())
            .service(create_user)
            .service(login))
            .listen(listener)?
            .run()
            .await
    }
}