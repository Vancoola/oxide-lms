use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserEvent {
    Created {
        user_id: Uuid,
        email: String,
    }
}