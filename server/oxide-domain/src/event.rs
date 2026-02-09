use async_trait::async_trait;
use crate::error::DomainError;
use crate::profile::event::ProfileEvent;
use crate::student::event::StudentEvent;
use crate::user::event::UserEvent;

pub enum GlobalEvent{
    User(UserEvent),
    Profile(ProfileEvent),
    Student(StudentEvent)
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: GlobalEvent) -> Result<(), DomainError>;
}