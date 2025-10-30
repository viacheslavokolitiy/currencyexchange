use crate::client_methods::{add_currency_to_wallet, buy_currency, create_buy_order, create_new_currency, create_new_wallet, create_sell_order, create_user, display_buy_orders, display_currencies, display_sell_orders, login_user, sell_currency};
use clap::Parser;
use currency_exchange_client::client::{ApiCommands, CliCommands, UserCommands};

mod api_endpoints {
    pub const LOGIN: &str = "/api/v1/login";
    pub const SIGNUP: &str = "/api/v1/users/create";
    pub const CURRENCY_LIST: &str = "/api/v1/currencies";
    pub const CREATE_CURRENCY: &str = "/api/v1/currencies/create";
    pub const CREATE_WALLET: &str = "/api/v1/wallet/create";
    pub const ADD_CURRENCY: &str = "/api/v1/wallet/currencies/add";
    pub const BUY_ORDERS: &str = "/api/v1/orders/buy";
    pub const SELL_ORDERS: &str = "/api/v1/orders/sell";
    pub const CREATE_BUY_ORDER: &str = "/api/v1/orders/buy/new";
    pub const CREATE_SELL_ORDER: &str = "/api/v1/orders/sell/new";
    pub const BUY_CURRENCY: &str = "/api/v1/orders/buy/execute";
    pub const SELL_CURRENCY: &str = "/api/v1/orders/sell/execute";
}

mod url_builder {
    use currency_exchange_client::client_env_parser::ClientEnvParser;
    pub fn build_login_base_url(parser: &ClientEnvParser) -> String {
        let host = parser.parse_login_host();
        let port = parser.parse_login_host_port();
        format!("{host}:{port}")
    }

    pub fn build_user_api_base_url(parser: &ClientEnvParser) -> String {
        let host = parser.parse_user_host();
        let port = parser.parse_user_port();
        format!("{host}:{port}")
    }

    pub fn build_orders_api_base_url(parser: &ClientEnvParser) -> String {
        let host = parser.parse_orders_host();
        let port = parser.parse_orders_port();
        format!("{host}:{port}")
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

mod client_methods {
    use crate::api_endpoints::{ADD_CURRENCY, BUY_CURRENCY, BUY_ORDERS, CREATE_BUY_ORDER, CREATE_CURRENCY, CREATE_SELL_ORDER, CREATE_WALLET, CURRENCY_LIST, LOGIN, SELL_CURRENCY, SELL_ORDERS, SIGNUP};
    use crate::password_encoder::encode_password;
    use crate::url_builder::{build_user_api_base_url, build_login_base_url, build_orders_api_base_url};
    use currency_exchange_client::client::{AddCurrencyArgs, BuyCurrencyArgs, CreateBuyOrderArgs, CreateCurrencyArgs, CreateSellOrderArgs, CreateUserArgs, CreateWalletArgs, ListCurrenciesArgs, LoginUserArgs, SellCurrencyArgs, ShowBuyOrdersArgs, ShowSellOrdersArgs};
    use currency_exchange_client::client_env_parser::ClientEnvParser;
    use currency_exchange_data::datasource::api_models::{AddCurrencyRequest, CreateBuyOrderRequest, CreateCurrencyRequest, CreateSellOrderRequest, CreateUserRequest, CreateUserResponse, CreateWalletRequest, ExchangeCurrencyRequest, LoginRequest};
    use reqwest::Client;
    use currency_exchange_data::datasource::models::{BuyOrder, Currency, CurrencyAmount, SellOrder, Wallet};

    pub async fn login_user(args: LoginUserArgs) {
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let login_request = LoginRequest::new(
            args.username,
            encode_password(args.password.as_str())
        );
        let res = network_client.post(format!("{}://{}{}", parser.parse_link_host(), build_login_base_url(&parser), LOGIN))
            .json(&login_request)
            .send()
            .await;
        if res.is_ok() {
            let resp = res.unwrap().json::<String>().await;
            println!("Your token {}", resp.unwrap());
        } else {
            println!("Failed to login {}", res.err().unwrap());
        }
    }

    pub async fn create_user(args: CreateUserArgs) {
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let signup_request = CreateUserRequest::new(
            args.username,
            args.email,
            encode_password(args.password.as_str()),
            args.firstname,
            Some(args.middlename),
            args.lastname,
        );
        let res = network_client.post(format!("{}://{}{}", parser.parse_link_host(), build_login_base_url(&parser), SIGNUP))
            .json(&signup_request)
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<CreateUserResponse>().await;
            println!("User created successfully {:?}", json.unwrap());
        } else {
            println!("Failed to create user {:?}", res);
        }
    }

    pub async fn display_currencies(args: ListCurrenciesArgs) {
        let token = args.auth_token;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let res = network_client.get(format!("{}://{}{}", parser.parse_link_host(), build_user_api_base_url(&parser), CURRENCY_LIST))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<Vec<Currency>>().await;
            if json.is_ok() {
                let currencies = json.unwrap();
                println!("{:?}", currencies);
            }
        } else {
            println!("Failed to get currencies {:?}", res);
        }
    }

    pub async fn create_new_currency(args: CreateCurrencyArgs) {
        let token = args.auth_token;
        let currency_code = args.currency_code;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let create_currency_req = CreateCurrencyRequest::new(currency_code);
        let res = network_client.post(format!("{}://{}{}", parser.parse_link_host(), build_user_api_base_url(&parser), CREATE_CURRENCY))
            .header("Authorization", format!("Bearer {}", token))
            .json(&create_currency_req)
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<Currency>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to create currency {:?}", res);
        }
    }

    pub async fn create_new_wallet(args: CreateWalletArgs) {
        let token = args.auth_token;
        let user_id = args.user_id;
        let currency_id = args.currency_id;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let create_wallet_req = CreateWalletRequest::new(
            user_id,
            currency_id,
        );
        let wallet_response = network_client.post(format!("{}://{}{}", parser.parse_link_host(), build_user_api_base_url(&parser), CREATE_WALLET))
            .header("Authorization", format!("Bearer {}", token))
            .json(&create_wallet_req)
            .send()
            .await;
        if wallet_response.is_ok() {
            let json = wallet_response.unwrap().json::<Wallet>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to create wallet {:?}", wallet_response);
        }
    }

    pub async fn add_currency_to_wallet(args: AddCurrencyArgs) {
        let user_id = args.user_id;
        let currency_id = args.currency_id;
        let token = args.auth_token;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let add_currency_req = AddCurrencyRequest::new(
            user_id,
            currency_id,
        );
        let add_currency_res = network_client.put(format!("{}://{}{}", parser.parse_link_host(), build_user_api_base_url(&parser), ADD_CURRENCY))
            .header("Authorization", format!("Bearer {}", token))
            .json(&add_currency_req)
            .send()
            .await;
        if add_currency_res.is_ok() {
            let json = add_currency_res.unwrap().json::<Wallet>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to add currency to {:?}", add_currency_res);
        }
    }

    pub async fn display_buy_orders(args: ShowBuyOrdersArgs) {
        let orders_limit = args.orders;
        let token = args.auth_token;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let params = [("count", orders_limit.to_string())];
        let url = format!("{}://{}{}", parser.parse_link_host(), build_orders_api_base_url(&parser), BUY_ORDERS);
        let url_with_params = reqwest::Url::parse_with_params(&url, &params).unwrap();
        let res = network_client.get(url_with_params)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<Vec<BuyOrder>>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to find buy orders {:?}", res);
        }
    }

    pub async fn display_sell_orders(args: ShowSellOrdersArgs) {
        let orders_limit = args.orders;
        let token = args.auth_token;
        let network_client = Client::new();
        let parser = ClientEnvParser::new();
        let params = [("count", orders_limit.to_string())];
        let url = format!("{}://{}{}", parser.parse_link_host(), build_orders_api_base_url(&parser), SELL_ORDERS);
        let url_with_params = reqwest::Url::parse_with_params(&url, &params).unwrap();
        let res = network_client.get(url_with_params)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<Vec<SellOrder>>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to find buy orders {:?}", res);
        }
    }

    pub async fn create_buy_order(args: CreateBuyOrderArgs) {
        let user_id = args.issuer_id;
        let buy_amount = args.buy_amount;
        let buy_currency_id = args.buy_currency_id;
        let sell_currency_id = args.sell_currency_id;
        let expiry_days = args.expiry_days;
        let token = args.auth_token;
        let parser = ClientEnvParser::new();
        let client = Client::new();

        let create_buy_order_req = CreateBuyOrderRequest::new(
            user_id,
            buy_amount,
            buy_currency_id,
            sell_currency_id,
            expiry_days,
        );
        let url = format!("{}://{}{}", parser.parse_link_host(), build_orders_api_base_url(&parser), CREATE_BUY_ORDER);
        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&create_buy_order_req)
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<BuyOrder>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to create buy order {:?}", res);
        }
    }
    
    pub async fn create_sell_order(args: CreateSellOrderArgs) {
        let user_id = args.issuer_id;
        let sell_amount = args.sell_amount;
        let buy_currency_id = args.buy_currency_id;
        let sell_currency_id = args.sell_currency_id;
        let expiry_days = args.expiry_days;
        let token = args.auth_token;
        let parser = ClientEnvParser::new();
        let client = Client::new();

        let create_buy_order_req = CreateSellOrderRequest::new(
            user_id,
            sell_amount,
            buy_currency_id,
            sell_currency_id,
            expiry_days,
        );
        let url = format!("{}://{}{}", parser.parse_link_host(), build_orders_api_base_url(&parser), CREATE_SELL_ORDER);
        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&create_buy_order_req)
            .send()
            .await;
        if res.is_ok() {
            let json = res.unwrap().json::<SellOrder>().await;
            if json.is_ok() {
                println!("{:?}", json.unwrap());
            }
        } else {
            println!("Failed to create buy order {:?}", res);
        }
    }
    
    pub async fn buy_currency(args: BuyCurrencyArgs) {
        let sum = args.sum;
        let rate = args.rate;
        let order_issuer_id = args.order_issuer_id;
        let incoming_currency_id = args.incoming_currency_id;
        let outgoing_currency_id = args.outgoing_currency_id;
        let token = args.auth_token;
        let env_parser = ClientEnvParser::new();
        let network_client = Client::new();
        let req = ExchangeCurrencyRequest::new(
            sum, rate, order_issuer_id, incoming_currency_id, outgoing_currency_id,
        );
        let url = format!("{}://{}{}", env_parser.parse_link_host(), build_orders_api_base_url(&env_parser), BUY_CURRENCY);
        let res = network_client.put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&req)
            .send()
            .await;
        if res.is_ok() {
            let result = res.unwrap().json::<CurrencyAmount>().await;
            if result.is_ok() {
                println!("{:?}", result.unwrap())
            }
        } else {
            println!("Failed to buy currency {:?}", res);
        }
    }
    
    pub async fn sell_currency(args: SellCurrencyArgs) {
        let sum = args.sum;
        let rate = args.rate;
        let order_issuer_id = args.order_issuer_id;
        let incoming_currency_id = args.incoming_currency_id;
        let outgoing_currency_id = args.outgoing_currency_id;
        let token = args.auth_token;
        let env_parser = ClientEnvParser::new();
        let network_client = Client::new();
        let req = ExchangeCurrencyRequest::new(
            sum, rate, order_issuer_id, incoming_currency_id, outgoing_currency_id,
        );
        let url = format!("{}://{}{}", env_parser.parse_link_host(), build_orders_api_base_url(&env_parser), SELL_CURRENCY);
        let res = network_client.put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&req)
            .send()
            .await;
        if res.is_ok() {
            let result = res.unwrap().json::<CurrencyAmount>().await;
            if result.is_ok() {
                println!("{:?}", result.unwrap())
            }
        } else {
            println!("Failed to buy currency {:?}", res);
        }
    }
}

fn main() {
    let cli = CliCommands::parse();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        match cli {
            CliCommands::Users { command } => match command {
                UserCommands::Create {args} => {
                    create_user(args).await;
                }
                UserCommands::Login {args} => {
                    login_user(args).await;
                }
            }
            CliCommands::Api { command } => match command {
                ApiCommands::ListCurrencies {args} => {
                    display_currencies(args).await;
                }
                ApiCommands::CreateCurrency {args} => {
                    create_new_currency(args).await;
                }
                ApiCommands::CreateWallet {args} => {
                    create_new_wallet(args).await;
                }
                ApiCommands::AddCurrencyForWallet {args} => {
                    add_currency_to_wallet(args).await;
                }
                ApiCommands::BuyOrders {args} => {
                    display_buy_orders(args).await;
                }
                ApiCommands::SellOrders {args} => {
                    display_sell_orders(args).await;
                }
                ApiCommands::CreateBuyOrder {args} => {
                    create_buy_order(args).await;
                }
                ApiCommands::CreateSellOrder {args} => {
                    create_sell_order(args).await;
                }
                ApiCommands::BuyCurrency {args} => {
                    buy_currency(args).await;
                }
                ApiCommands::SellCurrency {args} => {
                    sell_currency(args).await;
                }
            }
        }
    })
}
