use std::os::linux::raw::stat;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use validator::Validate;
use oxide_business::user::service::try_auth;
use oxide_domain::user::object::{Email, RawPassword};
use oxide_shared_types::auth::{AuthRequest, JwtToken};
use crate::AppState;
use crate::error::AppError;
use axum::debug_handler;

pub fn auth_router() -> Router<Arc<AppState>>{
    Router::new().route("/login", post(login))
}

#[debug_handler]
#[utoipa::path(
    context_path="/api/v1/auth",
    post,
    path="/login",
    tag="auth",
    request_body = AuthRequest,
    responses(
        (status = 200, body = JwtToken)
    ),
    summary = "Authenticate user and return JWT token",
    description = "Logs in a user by verifying credentials and returns an access token. \
                 Requires valid email and password."
)]
pub async fn login(State(state): State<Arc<AppState>>, Json(payload): Json<AuthRequest>) -> Result<Response, AppError> {

    payload.validate().map_err(|e| AppError::Internal(e.into()))?;

    let email = Email::new(payload.email.clone())?;
    let password = RawPassword::new(payload.password.clone())?;
    //TODO: make a more precise definition of httpstatuscode for errors
    let user = try_auth(state.user_repo.as_ref(), state.password_hasher.as_ref(), email, password).await?;
    Ok((StatusCode::OK, Json(JwtToken{token: user.email.as_str().to_string()})).into_response())
}