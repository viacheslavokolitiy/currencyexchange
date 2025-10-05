use std::env;
use std::path::PathBuf;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_MAX_CONNECTIONS: &str = "MAX_CONNECTIONS";

pub struct EnvParser {
    env: PathBuf,
}

impl EnvParser {
    pub fn new() -> Self {
        Self {
            env: dotenvy::dotenv().ok().expect("Unable to find .env file!"),
        }
    }

    pub fn database_url(&self) -> String {
        env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set")
    }

    pub fn max_connections(&self) -> u32 {
        env::var(ENV_MAX_CONNECTIONS).expect("MAX_CONNECTIONS must be set")
            .parse::<u32>().expect("MAX_CONNECTIONS must be a number")
    }
}

#[cfg(test)]
mod env_parser_spec {
    use std::env;
    use crate::migrationmanager::env_parser::EnvParser;

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