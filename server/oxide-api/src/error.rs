use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Str error")]
    StrError(#[from] std::str::Utf8Error),
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}