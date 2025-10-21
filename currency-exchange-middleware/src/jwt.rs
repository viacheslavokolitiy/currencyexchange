use actix_jwt_auth_middleware::FromRequest;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use crate::env_parser::{JwtEnvParser, MiddlewareEnv};

#[derive(Debug, Serialize, Deserialize, FromRequest, Clone)]
pub struct Claims {
    pub sub: String,
    exp: i64,
    iat: i64,
}

pub fn get_token(user_id: &i32, parser: &MiddlewareEnv) -> Result<String, Error> {
    let jwt_secret = parser.jwt_secret();
    let jwt_secret_bytes = jwt_secret.as_bytes();
    generate_jwt_token(&user_id, jwt_secret_bytes)
}
fn generate_jwt_token(user_id: &i32, secret: &[u8]) -> Result<String, Error> {
    let now = OffsetDateTime::now_utc();
    let expiration = now + Duration::minutes(60);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.unix_timestamp(),
        iat: now.unix_timestamp(),
    };

    let header = Header::new(Algorithm::HS256);

    let encoding_key = EncodingKey::from_secret(secret);

    encode(&header, &claims, &encoding_key)
}