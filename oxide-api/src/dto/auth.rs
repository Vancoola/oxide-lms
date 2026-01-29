use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Debug, ToSchema, Validate)]
pub struct LoginRequest {
    #[schema(example="example@axion-tech.ru", min_length=6, max_length=255, format="email", required)]
    #[validate(email, length(min=6, max=255))]
    pub email: String,
    #[schema(
        example = "password123",
        min_length = 8,
        max_length = 100,
        format="password",
        required
    )]
    #[validate(
        length(min = 8, max = 100),
    )]
    pub password: Option<String>
}