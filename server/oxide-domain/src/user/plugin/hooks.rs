use async_trait::async_trait;
use crate::user::User;

#[async_trait]
pub trait PostRegistrationHook: Send + Sync {
    async fn execute(&self, user: &User);
}