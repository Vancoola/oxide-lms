use async_trait::async_trait;
use uuid::Uuid;
use crate::user::User;
use crate::error::DomainError;
use crate::user::object::Email;

#[cfg_attr(any(test, feature = "mockall"), mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn get_user_by_email(&self, email: &Email) -> Result<User, DomainError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    async fn create_user_and_publish(&self, user: &mut User) -> Result<(), DomainError>;
    async fn get_password_by_email(&self, email: &Email) -> Result<(Uuid, String), DomainError>;
}