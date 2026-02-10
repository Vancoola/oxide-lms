pub mod repository;
pub mod event;
pub mod object;

use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::profile::event::ProfileEvent;
use crate::profile::object::{Name, ProfileId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: ProfileId,
    pub user_id: Uuid,

    pub first_name: Name,
    pub last_name: Name,
    pub middle_name: Name,

    pub is_activate: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,

    events: Vec<ProfileEvent>,
}

impl Profile {
    pub fn new(user_id: Uuid) -> Result<Self, DomainError> {
        let profile = Self {
            id: ProfileId::new(),
            user_id,
            first_name: Name::none(),
            last_name: Name::none(),
            middle_name: Name::none(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            is_activate: false,
            events: Vec::new(),
        };
        Ok(profile)
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
            id: ProfileId::load(id),
            user_id,
            first_name: Name::some(first_name),
            last_name: Name::some(last_name),
            middle_name: Name::some(middle_name),
            created_at,
            updated_at,
            is_activate,
            events: Vec::new(),
        }
    }

    pub fn update(
        &mut self,
        first_name: Name,
        last_name: Name,
        middle_name: Name,
    ) -> Result<(), DomainError> {
        self.first_name = first_name;
        self.last_name = last_name;
        self.middle_name = middle_name;
        self.updated_at = OffsetDateTime::now_utc();
        self.is_activate = true;
        Ok(())
    }

    pub fn pull_events(&mut self) -> Vec<ProfileEvent> {
        std::mem::take(&mut self.events)
    }

}
