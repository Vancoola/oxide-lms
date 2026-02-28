use crate::error::ApiError;
use crate::state::AppState;
use crate::user::AuthUser;
use axum::{debug_handler, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use oxide_business::user::service::register_user as register_user_service;
use oxide_domain::user::object::{Email, RawPassword};
use oxide_shared_types::admin::RegisterRequest;
use std::sync::Arc;

#[debug_handler]
#[utoipa::path(
    context_path="/api/v1/admin",
    post,
    path="/user/register",
    tag="admin"
)]
pub async fn register_user(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, ApiError> {
    if !auth_user.0.is_admin {
        return Err(ApiError::Forbidden);
    }

    let email =
        Email::new(payload.email.clone()).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    let password = RawPassword::new(payload.password.clone())
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let user = register_user_service(
        state.user_repo.as_ref(),
        state.password_hasher.as_ref(),
        state.user_extension_registry.as_ref(),
        email,
        password,
    )
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json({
            "name";
            "name"
        }),
    )
        .into_response())
}
