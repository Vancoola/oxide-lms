use std::sync::Arc;
use oxide_domain::error::DomainError;
use oxide_domain::event::EventPublisher;

pub mod error;
pub mod user;
mod profile;

#[derive(Clone)]
pub struct PostgresContext {
    pool: sqlx::PgPool,
}

impl PostgresContext {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

pub(crate) fn to_domain_err(err: sqlx::Error) -> DomainError {
    match err {
        sqlx::Error::RowNotFound => DomainError::NotFound,
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            DomainError::AlreadyExists
        }
        _ => DomainError::Infrastructure(err.to_string()),
    }
}
