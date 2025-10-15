use CurrencyExchange::server::Server;
use CurrencyExchange::utils::env_parser::EnvParser;

#[actix_web::main]
async fn main() {
    let parser = EnvParser::new();
    let srv = Server::new(parser);
    srv.start_server().await.expect("Failed to start server");
}
