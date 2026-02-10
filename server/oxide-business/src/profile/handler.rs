use std::sync::Arc;
use uuid::Uuid;
use oxide_domain::error::DomainError;
use oxide_domain::profile::repository::ProfileRepository;
use crate::profile::service::create_profile;

pub struct ProfileHandler {
    profile_repository: Arc<dyn ProfileRepository>,
}

impl ProfileHandler {
    pub fn new(profile_repository: Arc<dyn ProfileRepository>) -> Self {
        Self { profile_repository }
    }

    pub async fn on_user_created(&self, user_id: Uuid, email: String) -> Result<(), DomainError> {
        create_profile(self.profile_repository.as_ref(), user_id).await?;
        Ok(())
    }

}