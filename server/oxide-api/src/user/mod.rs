pub mod auth;
pub mod admin;

use std::sync::Arc;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::routing::{get, post};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use oxide_infrastructure::jwt::Claims;
use crate::AppState;
use crate::dto::user::UserInfo;
use crate::error::{ApiError};
use crate::user::admin::register_user;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new().route("/me", get(me))
}
pub fn admin_router() -> Router<Arc<AppState>> {
    Router::new().route("/user/register", post(register_user))
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
        first_name: "Firstname".to_string(),
        last_name: "Lastname".to_string(),
        group_code: "Group".to_string(),
    }))
}

pub struct AuthUser(pub Claims);
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);
        let jar = CookieJar::from_request_parts(parts, &app_state)
            .await
            .map_err(|_| ApiError::Unauthorized("Token is invalid".to_string()))?;

        let token = jar.get("access_lms_token").ok_or(ApiError::Unauthorized("Token not found".to_string()))?;

        let token_data = decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(app_state.config.jwt.secret.as_ref()),
            &Validation::default(),
        )
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        Ok(AuthUser(token_data.claims))
    }
}