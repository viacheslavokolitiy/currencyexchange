use crate::auth::jwt_secret_env_parser::JwtSecretEnvParser;
use crate::utils::env_parser::EnvParser;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

pub fn get_token(user_id: i32, parser: &EnvParser) -> Result<String, Error> {
    let jwt_secret = parser.parse_jwt_secret();
    let jwt_secret_bytes = jwt_secret.as_bytes();
    generate_jwt_token(&user_id, jwt_secret_bytes)
}
fn generate_jwt_token(user_id: &i32, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let now = OffsetDateTime::now_utc();
    let expiration = now + Duration::minutes(60);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.unix_timestamp(),
        iat: now.unix_timestamp(),
    };

    let header = Header::new(Algorithm::HS256); // Using HMAC-SHA256

    let encoding_key = EncodingKey::from_secret(secret);

    encode(&header, &claims, &encoding_key)
}