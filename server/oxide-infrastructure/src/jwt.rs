use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use oxide_domain::user::object::UserId;
use crate::error::InfrastructureError;

pub fn generate_jwt(user_id: UserId, is_admin: bool, secret_key: &str) -> Result<String, InfrastructureError> {
    let header = Header::new(Algorithm::HS256);
    Ok(encode(
        &header,
        &Claims {
            sub: *user_id.as_ref(),
            exp: (OffsetDateTime::now_utc() + Duration::days(5)).unix_timestamp(),
            is_admin
        },
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )?)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub is_admin: bool
}