use async_trait::async_trait;
use uuid::Uuid;
use oxide_domain::error::DomainError;

#[async_trait]
pub trait ProfileProvider {
    async fn get_display_name(&self, user_id: Uuid) -> Result<String, DomainError>;
}