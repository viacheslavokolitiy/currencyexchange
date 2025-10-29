use time::OffsetDateTime;
use currency_exchange_middleware::env_parser::MiddlewareEnv;
use currency_exchange_auth::server::Server;

#[actix_web::main]
async fn main() {
    let parser = MiddlewareEnv::new();
    let srv = Server::new(parser);
    srv.start_server().await.expect("Unable to start server");
    tracing::info!("Server started at : {}", OffsetDateTime::now_utc());
}