use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::InfrastructureError;

pub fn generate_jwt(user_id: &Uuid, secret_key: &str) -> Result<String, InfrastructureError> {
    let header = Header::new(Algorithm::HS256);
    Ok(encode(
        &header,
        &Claims {
            sub: *user_id
        },
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )?)
}

#[derive(Serialize, Deserialize)]
struct Claims {
    pub sub: Uuid,           // идентификатор пользователя
    //pub exp: usize,          // время истечения (Unix timestamp)
}