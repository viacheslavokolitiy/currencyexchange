use time::OffsetDateTime;
use currency_exchange_auth::env_parser::EnvParser;
use currency_exchange_auth::server::Server;

#[actix_web::main]
async fn main() {
    let parser = EnvParser::new();
    let srv = Server::new(parser);
    srv.start_server().await.expect("Unable to start server");
    tracing::info!("Server started at : {}", OffsetDateTime::now_utc());
}