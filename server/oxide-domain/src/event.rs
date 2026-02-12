use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::DomainError;
use crate::profile::event::ProfileEvent;
use crate::student::event::StudentEvent;
use crate::user::event::UserEvent;

#[derive(Serialize, Deserialize, Debug)]
pub enum GlobalEvent{
    User(UserEvent),
    Profile(ProfileEvent),
    Student(StudentEvent)
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: GlobalEvent) -> Result<(), DomainError>;
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &GlobalEvent) -> Result<(), DomainError>;
}