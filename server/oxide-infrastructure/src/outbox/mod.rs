pub mod postgres;

use async_trait::async_trait;

#[async_trait]
pub trait OutboxWatcher: Send + Sync {
    async fn watch(self);
}