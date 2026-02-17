use std::fmt::{Formatter, Debug};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct AuthRequest {
    #[cfg_attr(feature = "utoipa", schema(example="someone@somedomain.com", min_length=6, max_length=255, format="email", required))]
    #[validate(email, length(min=6, max=255))]
    pub email: String,
    #[cfg_attr(feature = "utoipa", schema(example="password123", min_length=8, max_length=100, format="password", required))]
    #[validate(length(min=8, max=100))]
    pub password: String,
}


#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}
impl Debug for JwtToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JwtToken(***)")
    }
}