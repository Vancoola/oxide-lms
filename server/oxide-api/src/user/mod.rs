pub mod auth;

use std::sync::Arc;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::routing::get;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use oxide_infrastructure::jwt::Claims;
use crate::AppState;
use crate::dto::user::UserInfo;
use crate::error::AppError;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new().route("/me", get(me))
}

#[utoipa::path(
    context_path="/api/v1/users",
    get,
    path="/me",
    tag="user",
    security(
        ("api_jwt_token" = [])
    )
)]
pub async fn me(auth_user: AuthUser) -> impl IntoResponse {
    (StatusCode::OK, Json(UserInfo{
        id: Uuid::new_v4(),
        first_name: "Денис".to_string(),
        last_name: "Суздальцев".to_string(),
        group_code: "ИТ-21-1".to_string(),
    }))
}

pub struct AuthUser(pub Claims);
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::MissingToken)?;

        if !auth_header.starts_with("Bearer") {
            return Err(AppError::MissingToken)
        }

        let token = &auth_header[7..];

        //Todo: Make a secret transfer
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
            .map_err(|_| AppError::MissingToken)?;

        Ok(AuthUser(token_data.claims))
    }
}