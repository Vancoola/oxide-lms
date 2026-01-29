use thiserror::Error;

#[derive(Debug, Error)]
pub enum BusinessError {
    #[error("Jwt error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}