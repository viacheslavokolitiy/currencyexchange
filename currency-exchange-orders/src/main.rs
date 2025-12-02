use std::path::PathBuf;
use std::str::FromStr;
use time::OffsetDateTime;
use currency_exchange_middleware::env_parser::EnvParser;
use currency_exchange_orders::server::{OrdersEnv, Server};

#[actix_web::main]
async fn main() {
    let parser = OrdersEnv::new();
    let srv = Server::new(parser);
    srv.start_server().await.expect("Unable to start server");
    tracing::info!("Server started at : {}", OffsetDateTime::now_utc());
}
