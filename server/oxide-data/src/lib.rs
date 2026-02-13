use oxide_domain::error::DomainError;
use oxide_domain::user::event::UserEvent;

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
    pub async fn outbox_watch(&self) -> Result<Vec<UserEvent>, DomainError> {
        let records = sqlx::query!("SELECT payload FROM outbox_events").fetch_all(&self.pool).await.map_err(to_domain_err)?;
        let event: Vec<UserEvent> = records
            .into_iter()
            .map(|row| {
                serde_json::from_value::<UserEvent>(row.payload)
                    .expect("Mapping failed")
            })
            .collect();
        Ok(event)
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
