use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use oxide_domain::error::DomainError;
use oxide_infrastructure::error::InfrastructureError;
use crate::user::me;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Str error")]
    StrError(#[from] std::str::Utf8Error),
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
    #[error("Domain error")]
    Domain(#[from] DomainError),
    #[error("Infrastructure error")]
    Infrastructure(#[from] InfrastructureError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::StrError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::Domain(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)),
            AppError::Infrastructure(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
        };
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}

#[derive(Error, Debug)]
pub enum ApiError{
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal Server Error")]
    InternalServerError(String),
}


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(e) => (StatusCode::BAD_REQUEST, e),
            ApiError::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, String::from("Forbidden")),
            ApiError::Conflict(e) => (StatusCode::CONFLICT, e), 
            ApiError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}