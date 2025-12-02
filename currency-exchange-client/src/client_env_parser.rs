use std::env;
use std::path::PathBuf;
use crate::client_env_parser::env_vars::{ENV_LINK, ENV_LOGIN_SERVER_HOST, ENV_LOGIN_SERVER_PORT, ENV_ORDERS_HOST, ENV_ORDERS_PORT, ENV_USER_HOST, ENV_USER_PORT};

mod env_vars {
    pub const ENV_LOGIN_SERVER_HOST: &str = "LOGIN_SERVER_HOST";
    pub const ENV_LOGIN_SERVER_PORT: &str = "LOGIN_SERVER_PORT";
    pub const ENV_LINK: &str = "LINK";
    pub const ENV_USER_PORT: &str = "USER_PORT";
    pub const ENV_USER_HOST: &str = "USER_HOST";
    pub const ENV_ORDERS_PORT: &str = "ORDERS_PORT";
    pub const ENV_ORDERS_HOST: &str = "ORDERS_HOST";
}

pub struct ClientEnvParser {
    env: PathBuf
}

impl ClientEnvParser {
    pub fn new() -> Self {
        Self {
            env: dotenvy::from_filename("./currency-exchange-client/.env").expect("Failed to load .env file!")
        }
    }
    pub fn parse_login_host(&self) -> String {
        env::var(ENV_LOGIN_SERVER_HOST).expect("LOGIN_SERVER_HOST must be set")
    }

    pub fn parse_login_host_port(&self) -> u16 {
        env::var(ENV_LOGIN_SERVER_PORT).expect("LOGIN_SERVER_PORT must be set")
            .parse::<u16>()
            .expect("LOGIN_SERVER_PORT must be an integer")
    }
    
    pub fn parse_user_host(&self) -> String {
        env::var(ENV_USER_HOST).expect("USER_HOST must be set")
    }
    
    pub fn parse_user_port(&self) -> u16 {
        env::var(ENV_USER_PORT)
            .expect("USER_PORT must be set")
            .parse::<u16>()
            .expect("USER_PORT must be an integer")
    }
    
    pub fn parse_orders_host(&self) -> String {
        env::var(ENV_ORDERS_HOST).expect("ORDERS_HOST must be set")
    }
    
    pub fn parse_orders_port(&self) -> u16 {
        env::var(ENV_ORDERS_PORT)
            .expect("ORDERS_PORT must be set")
            .parse::<u16>()
            .expect("ORDERS_PORT must be an integer")
    }

    pub fn parse_link_host(&self) -> String {
        env::var(ENV_LINK).expect("LINK must be set")
    }
}