use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use backend::database_connector::DatabaseConnector;
use backend::env_parser::EnvParser;
use backend::handlers::get_handlers::{find_buy_orders, find_exchange_rates, find_wallet, get_all_currencies, get_currency_by_code, is_username_taken};
use backend::handlers::post_handlers::{create_currency, create_exchange_rate, create_user, create_wallet, login_user, post_new_buy_order};
use backend::middleware::middleware::JwtMiddleware;
use backend::middleware::tracing_middleware::NetworkLogSpanBuilder;
use std::io;
use std::net::TcpListener;
use backend::handlers::put_handlers::replenish_balance;

pub const FETCH_ALL_CURRENCIES: &str = "/api/v1/currencies";
pub const FETCH_CURRENCY: &str = "/api/v1/currency";
pub const CREATE_CURRENCY: &str = "/api/v1/currencies/new";
pub const FIND_WALLET: &str = "/api/v1/me/wallet";
pub const CREATE_WALLET: &str = "/api/v1/me/wallet/create";
pub const EXCHANGE_RATES: &str = "/api/v1/currencies/rates";
pub const EXCHANGE_RATES_NEW: &str = "/api/v1/currencies/rates/create";
pub const REPLENISH_WALLET: &str = "/api/v1/me/wallet/replenish";
pub const POST_BUY_ORDER: &str = "/api/v1/orders/buy/new";

pub const FETCH_BUY_ORDERS: &str = "/api/v1/orders/buy/all";

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
        .service(is_username_taken)
        .service(
            web::resource(FETCH_ALL_CURRENCIES)
                .wrap(JwtMiddleware)
                .route(web::get().to(get_all_currencies))
        )
        .service(
            web::resource(FETCH_CURRENCY)
                .wrap(JwtMiddleware)
                .route(web::get().to(get_currency_by_code))
        )
        .service(
            web::resource(CREATE_CURRENCY)
                .wrap(JwtMiddleware)
                .route(web::post().to(create_currency))
        ).service(
            web::resource(FIND_WALLET)
                .wrap(JwtMiddleware)
                .route(web::get().to(find_wallet))
        ).service(
            web::resource(CREATE_WALLET)
                .wrap(JwtMiddleware)
                .route(web::post().to(create_wallet))
        ).service(
            web::resource(EXCHANGE_RATES)
                .wrap(JwtMiddleware)
                .route(web::get().to(find_exchange_rates))
         ).service(
            web::resource(EXCHANGE_RATES_NEW)
                .wrap(JwtMiddleware)
                .route(web::post().to(create_exchange_rate))
         ).service(
            web::resource(REPLENISH_WALLET)
                .wrap(JwtMiddleware)
                .route(web::put().to(replenish_balance))
         ).service(
             web::resource(POST_BUY_ORDER)
                 .wrap(JwtMiddleware)
                 .route(web::post().to(post_new_buy_order))
         ).service(
            web::resource(FETCH_BUY_ORDERS)
                .wrap(JwtMiddleware)
               .route(web::get().to(find_buy_orders))
         ))
        .listen(listener)?
        .run()
        .await
}
