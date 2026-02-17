use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use oxide_domain::error::DomainError;

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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::StrError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")),
            AppError::Domain(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)),
        };
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}