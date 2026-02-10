use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Jwt error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}