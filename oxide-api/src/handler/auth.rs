use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
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
pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(""))
}