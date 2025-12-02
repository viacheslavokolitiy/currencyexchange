use std::env;
use std::path::PathBuf;
use currency_exchange_middleware::env_parser::EnvParser;
use crate::swagger_env::env_vars::{ENV_DATABASE_URL, ENV_HOST, ENV_MAX_CONNECTIONS, ENV_ORDERS_HOST, ENV_ORDERS_PORT, ENV_PORT};

pub struct SwaggerEnv {
    env: PathBuf,
}

mod env_vars {
    pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
    pub const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";
    pub const ENV_HOST: &str = "SERVER_HOST";

    pub const ENV_PORT: &str = "SERVER_PORT";
    
    pub const ENV_ORDERS_PORT: &str = "ORDERS_PORT";
    
    pub const ENV_ORDERS_HOST: &str = "ORDERS_HOST";
}

impl SwaggerEnv {
    pub fn new() -> Self {
        Self {
            env: dotenvy::from_filename("./currency-exchange-swagger/.env").expect("Cannot load env file"),
        }
    }
    
    pub fn orders_host(&self) -> String {
        env::var(ENV_ORDERS_HOST).expect("ORDERS_HOST must be set")
    }
    
    pub fn orders_port(&self) -> String {
        env::var(ENV_ORDERS_PORT).expect("ORDERS_PORT must be set")
    }
}

impl EnvParser for SwaggerEnv {
    fn database_url(&self) -> String {
        env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set")
    }

    fn max_connections(&self) -> u32 {
        env::var(ENV_MAX_CONNECTIONS).expect("MAX_CONNECTIONS must be set")
            .parse::<u32>().expect("MAX_CONNECTIONS must be a number")
    }

    fn host(&self) -> String {
        env::var(ENV_HOST).expect("SERVER_HOST must be set")
    }

    fn port(&self) -> String {
        env::var(ENV_PORT).expect("SERVER_PORT must be set")
    }
}