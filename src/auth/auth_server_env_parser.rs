use std::env;
use crate::utils::env_parser::EnvParser;

pub trait AuthServerEnvParser {
    const AUTH_SERVER_ADDRESS: &'static str = "AUTH_SERVER_ADDRESS";
    fn parse_auth_server_address(&self) -> String;
}

impl AuthServerEnvParser for EnvParser {
    fn parse_auth_server_address(&self) -> String {
        env::var(Self::AUTH_SERVER_ADDRESS).expect("AUTH_SERVER_ADDRESS not set")
    }
}