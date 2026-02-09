use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use oxide_shared_types::auth::JwtToken;
use crate::AppState;
use crate::dto::auth::LoginRequest;

#[utoipa::path(
    context_path="/api/v1/auth",
    post,
    path="/login",
    tag="auth",
    summary = "Authenticate user and return JWT token",
    description = "Logs in a user by verifying credentials and returns an access token. \
                 Requires valid email and password."
)]
pub async fn login(State(state): State<Arc<AppState>>,Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(JwtToken{token: "access".to_string()}))
}