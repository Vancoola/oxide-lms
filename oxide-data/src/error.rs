use thiserror::Error;
use oxide_business::error::BusinessError;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("sqlx error: ")]
    Sqlx(#[from] sqlx::Error),
    #[error("business error: {0}")]
    Business(#[from] BusinessError),
    #[error("invalid password")]
    InvalidPassword,
}