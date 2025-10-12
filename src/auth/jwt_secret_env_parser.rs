use std::env;
use crate::utils::env_parser::EnvParser;

pub trait JwtSecretEnvParser {
    const JWT_SECRET: &'static str = "JWT_SECRET";

    fn parse_jwt_secret(&self) -> String;
}

impl JwtSecretEnvParser for EnvParser {
    fn parse_jwt_secret(&self) -> String {
        env::var(Self::JWT_SECRET).expect("JWT_SECRET not set")
    }
}