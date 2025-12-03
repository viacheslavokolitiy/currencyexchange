use std::io;
use std::net::TcpListener;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use currency_exchange_auth::database_connector::DatabaseConnector;
use currency_exchange_auth::env_parser::EnvParser;
use currency_exchange_auth::handlers::get_handlers::is_username_taken;
use currency_exchange_auth::handlers::post_handlers::{create_user, login_user};
use currency_exchange_auth::middleware::tracing_middleware::NetworkLogSpanBuilder;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let parser = EnvParser::new();
    let connector = DatabaseConnector::new(
        parser.database_url(),
        parser.max_connections()
    );

    let pool = connector.connect().await;
    let host = parser.host();
    let port = parser.port();
    let listener =
        TcpListener::bind(format!("{}:{}", host, port)).expect("Couldn't bind to port");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || App::new()
        .app_data(Data::new(pool.clone()))
        .wrap(NetworkLogSpanBuilder::new().middleware().clone())
        .service(create_user)
        .service(login_user)
        .service(is_username_taken))
        .listen(listener)?
        .run()
        .await
}
