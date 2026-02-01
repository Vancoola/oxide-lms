use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {

}