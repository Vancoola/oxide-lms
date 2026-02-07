pub mod repository;
pub mod service;
pub mod event;

use crate::crypto::PasswordHasher;
use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,

    pub email: String,
    pub password: String,

    pub is_admin: bool,
}

impl User {
    pub fn new(email: &str, password: &str) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password: password.to_string(),
            is_admin: false,
        })
    }

    pub fn new_admin(email: &str, password: &str) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password: password.to_string(),
            is_admin: true,
        })
    }

    pub fn load(id: Uuid, email: &str, password: &str, is_admin: bool) -> Self {
        Self {
            id,
            email: email.to_string(),
            password: password.to_string(),
            is_admin,
        }
    }

    pub fn check_password<H: PasswordHasher>(&self, plain: &str, hasher: &H) -> bool {
        hasher.verify(plain, &self.password).unwrap_or(false)
    }
}
