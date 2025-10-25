use time::OffsetDateTime;
use currency_exchange_admin::server::{AdminEnv, Server};

#[actix_web::main]
async fn main() {
    let env = AdminEnv::new();
    let srv = Server::new(env);
    srv.start_server().await.expect("Can't start server");
    tracing::info!("Server started at : {}", OffsetDateTime::now_utc());
}
