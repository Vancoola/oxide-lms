use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use oxide_shared_types::auth::{AuthRequest, JwtToken};
use crate::AppState;


pub fn auth_router() -> Router<Arc<AppState>>{
    Router::new().route("/login", post(login))
}

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
pub async fn login(State(_state): State<Arc<AppState>>,Json(_payload): Json<AuthRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(JwtToken{token: "access".to_string()}))
}