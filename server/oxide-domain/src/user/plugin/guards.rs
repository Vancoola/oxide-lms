use async_trait::async_trait;
use crate::error::DomainError;
use crate::user::object::Email;

#[async_trait]
pub trait PreRegistrationGuard: Send + Sync {
    async fn execute(&self, email: &Email) -> Result<(), DomainError>;
}