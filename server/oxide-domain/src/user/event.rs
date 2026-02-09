use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::user::object::{Email, UserId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserEvent {
    Created {
        user_id: UserId,
        email: Email,
    }
}