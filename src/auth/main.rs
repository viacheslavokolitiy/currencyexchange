use std::net::TcpListener;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use CurrencyExchange::auth::auth_server_env_parser::AuthServerEnvParser;
use CurrencyExchange::auth::endpoints::create_user;
use CurrencyExchange::utils::database_connector::DatabaseConnector;
use CurrencyExchange::utils::env_parser::EnvParser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}

pub async fn start_server() -> std::io::Result<()> {
    let env_parser = EnvParser::new();
    let connector = DatabaseConnector::new(
        env_parser.database_url(),
        env_parser.max_connections()
    );

    let pool = connector.connect().await;
    let listener =
        TcpListener::bind(env_parser.parse_auth_server_address()).expect("Couldn't bind to port");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || App::new()
        .app_data(Data::new(pool.clone()))
        .wrap(Logger::default())
        .service(create_user))
        .listen(listener)?
        .run()
        .await
}