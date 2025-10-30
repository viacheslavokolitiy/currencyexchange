use time::OffsetDateTime;
use currency_exchange_user::server::{Server, UserEnv};

#[actix_web::main]
async fn main() {
    let env = UserEnv::new();
    let srv = Server::new(env);
    srv.start_server().await.expect("Can't start server");
    tracing::info!("Server started at : {}", OffsetDateTime::now_utc());
}
