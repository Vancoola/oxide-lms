use async_trait::async_trait;
use uuid::Uuid;
use crate::error::DomainError;
use crate::user::User;

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError>;
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    async fn get_password_by_email(&self, email: &str) -> Result<(Uuid, String), DomainError>;
}