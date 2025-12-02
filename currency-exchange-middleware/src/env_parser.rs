use std::env;
use std::path::{Path, PathBuf};
use crate::env_parser::env_vars::ENV_DATABASE_URL;
use crate::env_parser::env_vars::ENV_HOST;
use crate::env_parser::env_vars::ENV_JWT_SECRET;
use crate::env_parser::env_vars::ENV_MAX_CONNECTIONS;
use crate::env_parser::env_vars::ENV_PORT;

pub(crate) mod env_vars {
    pub(crate) const ENV_DATABASE_URL: &str = "DATABASE_URL";
    pub(crate) const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";
    pub(crate) const ENV_JWT_SECRET: &str = "JWT_SECRET";
    pub(crate) const ENV_HOST: &str = "SERVER_HOST";
    pub(crate) const ENV_PORT: &str = "SERVER_PORT";
}

pub(crate) struct EnvParser {
    env: PathBuf
}

impl EnvParser {
    pub(crate) fn new() -> Self {
        Self {
            env: dotenvy::dotenv().ok().expect("Failed to find .env file"),
        }
    }
    pub(crate) fn database_url(&self) -> String {
        env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set")
    }

    pub(crate) fn max_connections(&self) -> u32 {
        env::var(ENV_MAX_CONNECTIONS).expect("MAX_CONNECTIONS must be set")
            .parse::<u32>().expect("MAX_CONNECTIONS must be a number")
    }

    pub(crate) fn jwt_secret(&self) -> String {
        env::var(ENV_JWT_SECRET).expect("JWT_SECRET must be set")
    }

    pub(crate) fn host(&self) -> String {
        env::var(ENV_HOST).expect("SERVER_HOST must be set")
    }

    pub(crate) fn port(&self) -> String {
        env::var(ENV_PORT).expect("SERVER_PORT must be set")
    }
}

#[cfg(test)]
mod env_parser_spec {
    use std::env;
    use crate::env_parser::EnvParser;

    #[test]
    fn should_load_env() {
        let parser = EnvParser::new();
        assert!(parser.env.is_file());
        assert!(parser.env.exists());
        let conn = env::var("MAX_CONNECTIONS");
        assert!(conn.is_ok());
        assert_eq!(conn.unwrap(), "10");
    }
}