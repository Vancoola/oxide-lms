use async_trait::async_trait;
use crate::error::DomainError;
use crate::user::User;

#[async_trait]
pub trait UserMiddleware: Send + Sync {
    async fn transform(&self, user: &mut User) -> Result<User, DomainError>;
}