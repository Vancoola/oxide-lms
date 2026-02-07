pub mod repository;
pub mod service;

use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,

    pub is_activate: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Profile {
    pub fn new(user_id: Uuid) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            first_name: None,
            last_name: None,
            middle_name: None,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            is_activate: false,
        })
    }

    pub fn load(
        id: Uuid,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        middle_name: Option<String>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
        is_activate: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            first_name,
            last_name,
            middle_name,
            created_at,
            updated_at,
            is_activate,
        }
    }

    pub fn update(
        &mut self,
        first_name: &str,
        last_name: &str,
        middle_name: Option<String>,
    ) -> Result<(), DomainError> {
        self.first_name = Some(first_name.to_string());
        self.last_name = Some(last_name.to_string());
        if let Some(middle_name) = middle_name {
            self.middle_name = Some(middle_name.to_string());
        } else {
            self.middle_name = None;
        }
        self.updated_at = OffsetDateTime::now_utc();
        self.is_activate = true;
        Ok(())
    }
}
