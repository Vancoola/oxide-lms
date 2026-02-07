pub mod repository;
mod service;

use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitType {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub can_have_students: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: Uuid,
    pub name: String,
    pub short_name: String,

    pub parent_id: Option<Uuid>,

    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub type_id: Uuid,
}

impl UnitType {
    pub fn new(code: &str, name: &str, can_have_students: bool) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            code: code.to_string(),
            name: name.to_string(),
            can_have_students,
        })
    }

    pub fn load(id: Uuid, code: &str, name: &str, can_have_students: bool) -> Self {
        Self {
            id,
            code: code.to_string(),
            name: name.to_string(),
            can_have_students,
        }
    }
}

impl Unit {
    pub fn new(
        name: &str,
        short_name: &str,
        parent_id: Option<Uuid>,
        is_active: bool,
        type_id: Uuid,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            short_name: short_name.to_string(),
            parent_id,
            is_active,
            type_id,
            created_at: OffsetDateTime::now_utc(),
        })
    }

    pub fn load(
        id: Uuid,
        name: &str,
        short_name: &str,
        parent_id: Option<Uuid>,
        is_active: bool,
        type_id: Uuid,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            short_name: short_name.to_string(),
            parent_id,
            is_active,
            type_id,
            created_at,
        }
    }
}
