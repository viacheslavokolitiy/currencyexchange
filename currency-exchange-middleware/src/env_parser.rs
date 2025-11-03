use std::env;
use std::path::PathBuf;
use std::str::FromStr;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";
const ENV_JWT_SECRET: &str = "JWT_SECRET";
const ENV_HOST: &str = "SERVER_HOST";

const ENV_PORT: &str = "SERVER_PORT";

pub trait EnvParser {
    fn database_url(&self) -> String;
    fn max_connections(&self) -> u32;
    fn host(&self) -> String;
    fn port(&self) -> String;
}

pub trait JwtEnvParser {
    fn database_url(&self) -> String;
    fn max_connections(&self) -> u32;
    fn jwt_secret(&self) -> String;
    fn host(&self) -> String;
    fn port(&self) -> String;
}

pub struct MiddlewareEnv {
    env: PathBuf
}

impl MiddlewareEnv {
    pub fn new() -> Self {
        Self {
            env: dotenvy::dotenv().ok().expect("Failed to find .env file"),
        }
    }
}

impl JwtEnvParser for MiddlewareEnv {
    fn database_url(&self) -> String {
        env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set")
    }

    fn max_connections(&self) -> u32 {
        env::var(ENV_MAX_CONNECTIONS).expect("MAX_CONNECTIONS must be set")
            .parse::<u32>().expect("MAX_CONNECTIONS must be a number")
    }

    fn jwt_secret(&self) -> String {
        env::var(ENV_JWT_SECRET).expect("JWT_SECRET must be set")
    }

    fn host(&self) -> String {
        env::var(ENV_HOST).expect("SERVER_HOST must be set")
    }

    fn port(&self) -> String {
        env::var(ENV_PORT).expect("SERVER_PORT must be set")
    }
}

#[cfg(test)]
mod env_parser_spec {
    use std::env;
    use crate::env_parser::MiddlewareEnv;

    #[test]
    fn should_load_env() {
        let parser = MiddlewareEnv::new();
        assert!(parser.env.is_file());
        assert!(parser.env.exists());
        let conn = env::var("MAX_CONNECTIONS");
        assert!(conn.is_ok());
        assert_eq!(conn.unwrap(), "10");
    }
}