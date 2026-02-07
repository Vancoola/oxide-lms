use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("user already exists")]
    AlreadyExists,
    #[error("user not found")]
    NotFound,
    #[error("Internal infrastructure error: {0}")]
    Infrastructure(String),
    #[error("Already activate")]
    AlreadyActivated,
}