use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::dto::CreateUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,

    pub email: String,
    pub(crate) password: String,

    pub name: String,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Self {
        Self { id: Uuid::new_v4(), email, password, name, }
    }
}

