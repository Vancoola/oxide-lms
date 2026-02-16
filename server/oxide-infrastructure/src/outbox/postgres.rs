use std::sync::Arc;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tracing::{debug, error, info};
use oxide_domain::event::{EventPublisher, GlobalEvent};
use crate::error::InfrastructureError;
use crate::outbox::OutboxWatcher;

pub struct PostgresOutboxWatcher {
    pool: PgPool,
    event_bus: Arc<dyn EventPublisher>
}
impl PostgresOutboxWatcher {
    pub fn new(pool: PgPool, event_bus: Arc<dyn EventPublisher>) -> Self {
        Self { pool, event_bus }
    }
}
#[async_trait]
impl OutboxWatcher for PostgresOutboxWatcher {
    async fn watch(self) {
        info!("PostgresOutbox Watcher started");

        let event_bus = self.event_bus.clone();

        tokio::spawn(async move {
            if let Err(e) = listen(event_bus, self.pool.clone()).await {
                error!("Error listening on event listener: {:?}", e);
            }
        });
    }
}

async fn listen(event_bus: Arc<dyn EventPublisher>, pool: PgPool) -> Result<(), InfrastructureError> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("outbox_event").await?;

    while let Some(notification) = listener.try_recv().await? {
        //TODO:Remove payload from notification. use notification as a ping signal for SELECT
        match serde_json::from_str::<GlobalEvent>(notification.payload()) {
            Ok(event) => {
                debug!("Received event: {:?}", event);
                if event_bus.publish(event).await.is_err() {
                    error!("Published event because we couldn't publish it");
                }
            }
            Err(e) => {
                error!("Serialization error: {}", e);
            }
        }

    }

    Ok(())
}