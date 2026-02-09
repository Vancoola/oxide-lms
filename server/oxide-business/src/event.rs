use async_trait::async_trait;
use tokio::sync::mpsc;
use oxide_domain::error::DomainError;
use oxide_domain::event::{EventPublisher, GlobalEvent};

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
