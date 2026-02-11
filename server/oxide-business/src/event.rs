use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc;
use oxide_domain::error::DomainError;
use oxide_domain::event::{EventHandler, EventPublisher, GlobalEvent};

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


#[cfg(test)]
mod test {
    use mockall::mock;
    use rstest::rstest;
    use uuid::Uuid;
    use oxide_domain::user::event::UserEvent;
    use super::*;

    #[rstest]
    #[tokio::test]
    async fn publish_event_success(){
        let (bus, mut recv) = TokyoEventBus::new();
        bus.publish(GlobalEvent::User(UserEvent::Created {user_id: Uuid::new_v4(), email: "test@email.com".to_string()})).await.unwrap();
        let one_ev = recv.recv().await;
        assert!(one_ev.is_some());
        one_ev.unwrap();
    }

    mock! {
        pub Handler {}
        #[async_trait]
        impl EventHandler for Handler {
            async fn handle(&self, event: &GlobalEvent) -> Result<(), DomainError>;
        }
    }

    #[rstest]
    #[tokio::test]
    async fn dispatcher_should_call_subscribers() {
        let (bus, recv) = TokyoEventBus::new();
        let mut mock_handler = MockHandler::new();

        mock_handler.expect_handle()
            .times(1)
            .returning(|_| Ok(()));

        let dispatcher = EventDispatcher::new(vec![Arc::new(mock_handler)]);

        tokio::spawn(async move {
            dispatcher.run(recv).await;
        });

        bus.publish(GlobalEvent::User(UserEvent::Created { user_id: Uuid::new_v4(), email: "test@email.com".to_string() })).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}