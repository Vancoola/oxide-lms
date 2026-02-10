use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc;
use oxide_domain::error::DomainError;
use oxide_domain::event::{EventHandler, EventPublisher, GlobalEvent};
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
    subscribers: Vec<Arc<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new(subscribers: Vec<Arc<dyn EventHandler>>) -> Self {
        Self { subscribers }
    }
    pub async fn run(self, mut recv:  mpsc::UnboundedReceiver<GlobalEvent>) {
        tracing::info!("Event Dispatcher started");
        while let Some(event) = recv.recv().await {
            let subscribers = self.subscribers.clone();
            //let event_arc = Arc::new(event);
            tokio::spawn(async move {
                for sub in subscribers {
                    if let Err(e) = sub.handle(&event).await {
                        tracing::error!("Error handling event: {:?}", e);
                    }
                }
            });
        }
    }
}

