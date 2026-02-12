use std::sync::Arc;
use async_trait::async_trait;
use tracing::debug;
use uuid::Uuid;
use oxide_domain::error::DomainError;
use oxide_domain::event::{EventHandler, GlobalEvent};
use oxide_domain::profile::repository::ProfileRepository;
use oxide_domain::user::event::UserEvent;
use crate::profile::service::create_profile;

pub struct ProfileHandler {
    profile_repository: Arc<dyn ProfileRepository + Send + Sync>,
}

impl ProfileHandler {
    pub fn new(profile_repository: Arc<dyn ProfileRepository + Send + Sync>) -> Self {
        Self { profile_repository }
    }
}

#[async_trait]
impl EventHandler for ProfileHandler {
    async fn handle(&self, event: &GlobalEvent) -> Result<(), DomainError> {
        match event {
            GlobalEvent::User(u) => {
                match u {
                    UserEvent::Created { user_id, email } => {
                        create_profile(self.profile_repository.as_ref(), *user_id).await?;
                        debug!("Profile Created: {:?}", user_id);
                        Ok(())
                    }
                }
            }
            _ => Ok(())
        }
    }
}