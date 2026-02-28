use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StudentEvent {
    Created {
        user_id: Uuid,
    }
}