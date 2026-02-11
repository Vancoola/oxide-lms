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
}