use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum DomainError {
    #[error("Invalid input value: {0}")]
    InvalidInputValue(String),
    #[error("user already exists")]
    AlreadyExists,
    #[error("user not found")]
    NotFound,
    #[error("Internal infrastructure error: {0}")]
    Infrastructure(String),
    #[error("Already activate")]
    AlreadyActivated,
    #[error("Publish error: {0}")]
    PublishError(String)
}