use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc;
use oxide_domain::error::DomainError;
use oxide_domain::event::{EventPublisher, GlobalEvent};
use oxide_domain::user::event::UserEvent;
use crate::profile::handler::ProfileHandler;

pub struct TokyoEventBus {
    sender: mpsc::UnboundedSender<GlobalEvent>
}

impl TokyoEventBus {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<GlobalEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }
}

#[async_trait]
impl EventPublisher for TokyoEventBus {
    async fn publish(&self, event: GlobalEvent) -> Result<(), DomainError> {
        self.sender.send(event).map_err(|e| DomainError::PublishError(e.to_string()))
    }
}

pub struct EventDispatcher {
    profile_handler: Arc<ProfileHandler>,
}

impl EventDispatcher {
    pub fn new(profile_handler: Arc<ProfileHandler>) -> Self {
        Self { profile_handler }
    }
    pub async fn run(self, mut recv:  mpsc::UnboundedReceiver<GlobalEvent>) {
        tracing::info!("Event Dispatcher started");

        while let Some(event) = recv.recv().await {
            let handler = Arc::clone(&self.profile_handler);
            tokio::spawn(async move {
                if let Err(e) = dispatch_event(event, handler).await {
                    tracing::error!("Event dispatch error: {:?}", e);
                }
            });
        }
    }
}

async fn dispatch_event(
    event: GlobalEvent,
    profile_handler: Arc<ProfileHandler>
) -> Result<(), DomainError> {
    match event {
        GlobalEvent::User(user_event) => {
            match user_event {
                UserEvent::Created { user_id, email } => profile_handler.on_user_created(user_id, email).await?,
            }
        }
        _ => {}
    }
    Ok(())
}

