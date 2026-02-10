use crate::error::DomainError;
use crate::profile::Profile;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn get_profile_by_id(&self, id: Uuid) -> Result<Profile, DomainError>;
    async fn get_profile_by_uid(&self, uid: Uuid) -> Result<Profile, DomainError>;
    async fn exists_profile_by_uid(&self, uid: Uuid) -> Result<bool, DomainError>;
    async fn create_profile(&self, profile: &Profile) -> Result<(), DomainError>;
    async fn update_profile(&self, profile: &Profile) -> Result<(), DomainError>;
}
