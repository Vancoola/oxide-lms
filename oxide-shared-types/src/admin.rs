use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Student,
    Faculty,
    Admin
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[cfg_attr(feature = "utoipa", schema(example="SomeFirstname", min_length=2, max_length=255, required))]
    #[validate(email, length(min=2, max=255))]
    pub firstname: String,
    #[cfg_attr(feature = "utoipa", schema(example="SomeLastname", min_length=2, max_length=255, required))]
    #[validate(email, length(min=2, max=255))]
    pub lastname: String,
    #[cfg_attr(feature = "utoipa", schema(example="someone@somedomain.com", min_length=6, max_length=255, format="email", required))]
    #[validate(email, length(min=6, max=255))]
    pub email: String,
    #[cfg_attr(feature = "utoipa", schema(example="password123", min_length=8, max_length=100, format="password", required))]
    #[validate(length(min=8, max=100))]
    pub password: String,
    pub role: Role
}