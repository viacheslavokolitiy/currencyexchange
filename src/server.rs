use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use crate::api::middleware::{protected, JwtMiddleware};
use crate::api::post_handlers::create_user;
use crate::utils::database_connector::DatabaseConnector;
use crate::utils::env_parser::EnvParser;
use crate::utils::tracing_middleware::NetworkLogSpanBuilder;

pub struct Server {
    env_parser: EnvParser,
}

impl Server {
    pub fn new(env_parser: EnvParser) -> Self {
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
            .wrap(NetworkLogSpanBuilder::new().middleware().clone())
            .service(create_user)
            .service(
                web::resource("/protected")
                    .wrap(JwtMiddleware)
                    .route(web::get().to(protected)),
            ))
            .listen(listener)?
            .run()
            .await
    }
}