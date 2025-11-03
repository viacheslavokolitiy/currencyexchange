use crate::swagger::password_encoder::encode_password;
use crate::swagger::swagger_models::{BuyOrderBadRequest, CreateWalletRequest};
use crate::swagger::swagger_models::BuyOrderNotFound;
use crate::swagger::swagger_models::CreateCurrencyRequest;
use crate::swagger::swagger_models::CurrencyResponse;
use crate::swagger::swagger_models::LoginRequest;
use crate::swagger::swagger_models::SellCurrencyRequest;
use crate::swagger::swagger_models::SellOrder;
use crate::swagger::swagger_models::SellOrderBadRequest;
use crate::swagger::swagger_models::SellOrderNotFound;
use crate::swagger::swagger_models::SignupRequest;
use crate::swagger::swagger_models::SignupResponse;
use crate::swagger::swagger_models::{AddCurrencyToWalletRequest, BuyCurrencyRequest, BuyOrder, CreateBuyOrderRequest, CreateSellOrderRequest, CurrencyAmount, Wallet};
use crate::swagger::utoipa_endpoints::GET_CURRENCY_LIST;
use crate::swagger::utoipa_endpoints::GET_SELL_ORDERS;
use crate::swagger::utoipa_endpoints::POST_CREATE_BUY_ORDER;
use crate::swagger::utoipa_endpoints::POST_CREATE_CURRENCY;
use crate::swagger::utoipa_endpoints::POST_CREATE_SELL_ORDER;
use crate::swagger::utoipa_endpoints::POST_CREATE_USER;
use crate::swagger::utoipa_endpoints::POST_LOGIN;
use crate::swagger::utoipa_endpoints::PUT_BUY_CURRENCY;
use crate::swagger::utoipa_endpoints::PUT_SELL_CURRENCY;
use crate::swagger::utoipa_endpoints::{GET_BUY_ORDERS, POST_CREATE_WALLET, PUT_ADD_WALLET_CURRENCY};
use crate::swagger_env::SwaggerEnv;
use actix_web::{get, post, HttpRequest, HttpResponse};
use currency_exchange_data::datasource::api_models::CreateUserRequest;
use log::info;
use reqwest::Client;

mod utoipa_endpoints {
    pub const GET_BUY_ORDERS: &str = "/api/v1/orders/buy";
    pub const GET_SELL_ORDERS: &str = "/api/v1/orders/sell";
    pub const GET_CURRENCY_LIST: &str = "/api/v1/currencies";
    pub const POST_CREATE_USER: &str = "/api/v1/users/create";
    pub const POST_LOGIN: &str = "/api/v1/login";
    pub const POST_CREATE_CURRENCY: &str = "/api/v1/currencies/create";
    pub const PUT_SELL_CURRENCY: &str = "/api/v1/orders/sell/execute";
    pub const PUT_BUY_CURRENCY: &str = "/api/v1/orders/buy/execute";

    pub const POST_CREATE_SELL_ORDER: &str = "/api/v1/orders/sell/new";
    pub const POST_CREATE_BUY_ORDER: &str = "/api/v1/orders/buy/new";
    
    pub const PUT_ADD_WALLET_CURRENCY: &str = "/api/v1/wallet/currencies/add";

    pub const POST_CREATE_WALLET: &str = "/api/v1/wallet/create";
}

#[utoipa::path(
    get,
    path = "/api/v1/orders/buy",
    responses(
        (status = 200, body = BuyOrder),
        (status = 404, body = BuyOrderNotFound),
        (status = 401, body = BuyOrderBadRequest)
    ),
    params(
        ("count" = String, Query, description = "Displays number of orders"),
    )
)]
#[get("/api/v1/orders/buy")]
async fn buy_orders(req: HttpRequest) -> HttpResponse {
    tracing::info!("Incoming request : {:?}", req);
    info!("GET /api/v1/orders");
    let client = reqwest::Client::new();
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };

    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let res = client.get(GET_BUY_ORDERS)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<Vec<currency_exchange_data::datasource::models::BuyOrder>>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else {
            HttpResponse::NotFound().json("Failed to get buy orders")
        }
    } else {
        HttpResponse::NotFound().json("Failed to get buy orders")
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/orders/sell",
    responses(
        (status = 200, body = SellOrder),
        (status = 404, body = SellOrderNotFound),
        (status = 401, body = SellOrderBadRequest)
    ),
    params(
        ("count" = String, Query, description = "Displays number of orders"),
    )
)]
#[get("/api/v1/orders/sell")]
async fn sell_orders(req: HttpRequest) -> HttpResponse {
    tracing::info!("Incoming request : {:?}", req);
    info!("GET /api/v1/sell");
    let client = reqwest::Client::new();
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let res = client.get(GET_SELL_ORDERS)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<Vec<currency_exchange_data::datasource::models::SellOrder>>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else {
            HttpResponse::NotFound().json("Failed to get sell orders")
        }
    } else {
        HttpResponse::NotFound().json("Failed to get sell orders")
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/currencies",
    responses(
        (status = 200, body = CurrencyResponse),
        (status = 401, body = String),
        (status = 404, body = String),
    )
)]
async fn currencies(req: HttpRequest) -> HttpResponse {
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let network_client = Client::new();
    let res = network_client.get(GET_CURRENCY_LIST)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<Vec<CurrencyResponse>>().await;
        if json.is_ok() {
            let currencies = json.unwrap();
            HttpResponse::Ok().json(currencies)
        } else {
            HttpResponse::NotFound().json("Failed to get currencies")
        }
    } else {
        HttpResponse::NotFound().json("Failed to get currencies")
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/users/create",
    request_body = SignupRequest,
    responses(
        (status = 201, body = SignupResponse),
        (status = 400, body = SignupResponse)
    )
)]
async fn register(body: SignupRequest) -> HttpResponse {
    let network_client = Client::new();
    let signup_request = CreateUserRequest::new(
        body.username,
        body.email,
        encode_password(body.password.as_str()),
        body.firstname,
        body.middlename,
        body.lastname,
    );
    let res = network_client.post(POST_CREATE_USER)
        .json(&signup_request)
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<SignupResponse>().await;
        HttpResponse::Created().json(json.unwrap())
    } else {
        let json = res.unwrap().json::<SignupResponse>().await;
        HttpResponse::BadRequest().json(json.unwrap())
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, body = String),
        (status = 400, body = String)
    )
)]
async fn login(body: LoginRequest) -> HttpResponse {
    let network_client = Client::new();
    let login_request = LoginRequest {
        username: body.username,
        password: body.password,
    };
    let res = network_client.post(POST_LOGIN)
        .json(&login_request)
        .send()
        .await;
    if res.is_ok() {
        let resp = res.unwrap().json::<String>().await;
        HttpResponse::Ok().json(resp.unwrap())
    } else {
        HttpResponse::BadRequest().json("Login failed")
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/currencies/create",
    request_body = CreateCurrencyRequest,
    responses(
        (status = 201, body = CurrencyResponse),
        (status = 400, body = String)
    )
)]
async fn create_currency(req: HttpRequest, body: CreateCurrencyRequest) -> HttpResponse {
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let currency_code = body.currency_code;
    let network_client = Client::new();
    let create_currency_req = CreateCurrencyRequest {
        currency_code,
    };
    let res = network_client.post(POST_CREATE_CURRENCY)
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_currency_req)
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<CurrencyResponse>().await;
        if json.is_ok() {
            HttpResponse::Created().json(json.unwrap())
        } else {
            HttpResponse::BadRequest().json("Failed to create currency")
        }
    } else {
        HttpResponse::BadRequest().json("Failed to create currency")
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/wallet/create",
    request_body = CreateWalletRequest,
    responses(
        (status = 200, body = Wallet),
        (status = 400, body = String),
    )
)]
pub async fn create_new_wallet(req: HttpRequest, args: CreateWalletRequest) -> HttpResponse {
    let network_client = Client::new();
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let wallet_response = network_client.post(POST_CREATE_WALLET)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await;
    if wallet_response.is_ok() {
        let json = wallet_response.unwrap().json::<Wallet>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else { 
            HttpResponse::BadRequest().json("Failed to create wallet")
        }
    } else {
        HttpResponse::BadRequest().json("Failed to create wallet")
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/wallet/currencies/add",
    request_body = AddCurrencyToWalletRequest,
    responses(
        (status = 200, body = Wallet),
        (status = 400, body = String),
    )
)]
pub async fn add_currency_to_wallet(req: HttpRequest, args: AddCurrencyToWalletRequest) -> HttpResponse {
    let client = Client::new();
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let add_currency_res = client.put(PUT_ADD_WALLET_CURRENCY)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await;
    if add_currency_res.is_ok() {
        let json = add_currency_res.unwrap().json::<Wallet>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else { 
            HttpResponse::BadRequest().json("Failed to add currency to") 
        }
    } else {
        HttpResponse::BadRequest().json("Failed to add currency to")
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/orders/buy/new",
    request_body = CreateBuyOrderRequest,
    responses(
        (status = 200, body = BuyOrder),
        (status = 400, body = String),
    )
)]
pub async fn create_buy_order(req: HttpRequest, args: CreateBuyOrderRequest) -> HttpResponse {
    let client = Client::new();
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let res = client.post(POST_CREATE_BUY_ORDER)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<BuyOrder>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else {
            HttpResponse::BadRequest().json("Failed to create buy order")
        }
    } else {
        HttpResponse::BadRequest().json("Failed to create buy order")
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/orders/sell/new",
    request_body = CreateSellOrderRequest,
    responses(
        (status = 200, body = SellOrder),
        (status = 400, body = String),
    )
)]
pub async fn create_sell_order(req: HttpRequest, args: CreateSellOrderRequest) -> HttpResponse {
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let client = Client::new();;
    let res = client.post(POST_CREATE_SELL_ORDER)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await;
    if res.is_ok() {
        let json = res.unwrap().json::<SellOrder>().await;
        if json.is_ok() {
            HttpResponse::Ok().json(json.unwrap())
        } else {
            HttpResponse::BadRequest().json("Failed to create sell order")
        }
    } else {
        HttpResponse::BadRequest().json("Failed to create sell order")
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/orders/buy/execute",
    request_body = BuyCurrencyRequest,
    responses(
        (status = 200, body = CurrencyAmount),
        (status = 400, body = String),
    )
)]
pub async fn buy_currency(req: HttpRequest, args: BuyCurrencyRequest) -> HttpResponse {
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let network_client = Client::new();
    let res = network_client.put(PUT_BUY_CURRENCY)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await;
    if res.is_ok() {
        let result = res.unwrap().json::<CurrencyAmount>().await;
        if result.is_ok() {
            HttpResponse::Ok().json(result.unwrap())
        } else {
            HttpResponse::BadRequest().json("Failed to buy currency")
        }
    } else {
        HttpResponse::BadRequest().json("Failed to put currency")
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/orders/sell/execute",
    request_body = SellCurrencyRequest,
    responses(
        (status = 200, body = CurrencyAmount),
        (status = 400, body = String),
    )
)]
pub async fn sell_currency(req: HttpRequest, body: SellCurrencyRequest) -> HttpResponse {
    let token = match req.headers().get("Authorization") {
        Some(h) => h.to_str().ok(),
        None => None,
    };
    let token = match token {
        Some(t) if t.starts_with("Bearer ") => t.trim_start_matches("Bearer ").to_string(),
        _ => return HttpResponse::BadRequest().json("Invalid Bearer Token"),
    };
    let network_client = Client::new();
    let res = network_client.put(PUT_SELL_CURRENCY)
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await;
    if res.is_ok() {
        let result = res.unwrap().json::<CurrencyAmount>().await;
        if result.is_ok() {
            HttpResponse::Ok().json(result.unwrap())
        } else {
            HttpResponse::BadRequest().json("Error")
        }
    } else {
        HttpResponse::BadRequest().json("Error")
    }
}

mod password_encoder {
    use argon2::Config;

    pub fn encode_password(pwd: &str) -> String {
        let config = Config::default();
        let salt = b"saltsaltsalt";
        let hash = argon2::hash_encoded(pwd.as_bytes(), salt, &config).unwrap();
        hash
    }
}


mod swagger_models {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, Debug, ToSchema, Default)]
    pub struct BuyOrder {
        pub buy_order_id: Option<i32>,
        pub issuer_id: Option<i32>,
        pub buy_currency_amount: Option<i32>,
        pub created_at: String,
        pub updated_at: String,
        pub expires_at: String,
        pub buy_currency_id: Option<i32>,
        pub sell_currency_id: Option<i32>,
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct SellOrder {
        pub sell_order_id: Option<i32>,
        pub issuer_id: Option<i32>,
        pub sell_currency_amount: Option<i32>,
        pub created_at: String,
        pub updated_at: String,
        pub expires_at: String,
        pub sell_currency_id: Option<i32>,
        pub buy_currency_id: Option<i32>
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct SignupRequest {
        pub username: String,
        pub email: String,
        pub password: String,
        pub firstname: String,
        pub middlename: Option<String>,
        pub lastname: String,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct CreateCurrencyRequest {
        pub currency_code: String,
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct SignupResponse {
        error_message: String,
        user: Option<CreatedUserResponse>,
        token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct CreatedUserResponse {
        pub id: i32,
        username: String,
        email: String,
        firstname: String,
        middlename: Option<String>,
        lastname: String,
        created_at: String,
        updated_at: String,
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct BuyOrderNotFound {
        pub message: String
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct SellOrderNotFound {
        pub message: String
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct BuyOrderBadRequest {
        pub message: String
    }

    #[derive(Serialize, Deserialize, Debug, ToSchema)]
    pub struct SellOrderBadRequest {
        pub message: String
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct CurrencyResponse {
        pub currency_id: Option<i32>,
        pub currency_code: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct SellCurrencyRequest {
        pub sum: i32,
        pub rate: f32,
        pub order_issuer_id: i32,
        pub incoming_currency_id: i32,
        pub outgoing_currency_id: i32
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct BuyCurrencyRequest {
        pub sum: i32,
        pub rate: f32,
        pub order_issuer_id: i32,
        pub incoming_currency_id: i32,
        pub outgoing_currency_id: i32
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct CurrencyAmount {
        pub amount: Option<i32>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct CreateSellOrderRequest {
        pub issuer_id: i32,
        pub sell_amount: i32,
        pub sell_currency_id: i32,
        pub buy_currency_id: i32,
        pub expiry_days: i32,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct AddCurrencyToWalletRequest {
        pub user_id: i32,
        pub currency_id: i32
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct Wallet {
        pub wallet_id: i32,
        pub user_id: i32,
        pub currency_id: i32
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct CreateBuyOrderRequest {
        pub issuer_id: i32,
        pub buy_amount: i32,
        pub buy_currency_id: i32,
        pub sell_currency_id: i32,
        pub expiry_days: i32,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
    pub struct CreateWalletRequest {
        pub user_id: i32,
        pub currency_id: i32,
    }
}