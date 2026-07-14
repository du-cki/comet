use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: i64,
    pub exp: usize,
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn create_jwt(
    user_id: i64,
    role: i64,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = current_timestamp() + (24 * 3600);

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
