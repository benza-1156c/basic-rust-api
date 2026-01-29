use std::env;

use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, encode, errors::Error as JwtError};
use sea_orm::sqlx::types::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn createjwt_token(user_id: String, email: String, role: String, duration: i64) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(duration))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        email,
        role,
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("ไม่พบ JWT_SECRET ใน .env");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}
