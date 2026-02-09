pub mod repository;
pub mod service;
pub mod object;

use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::organizational_unit::object::{ShortName, UnitCode, UnitTypeId, UnitName, UnitId, UnitPath};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitType {
    pub id: UnitTypeId,
    pub code: UnitCode,
    pub name: UnitName,
    pub can_have_students: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub name: UnitName,
    pub short_name: ShortName,

    pub parent_id: Option<UnitId>,

    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub type_id: UnitTypeId,
    pub path: UnitPath
}

impl UnitType {
    pub fn new(code: UnitCode, name: UnitName, can_have_students: bool) -> Result<Self, DomainError> {
        Ok(Self {
            id: UnitTypeId::new(),
            code,
            name,
            can_have_students,
        })
    }

    pub fn load(id: &Uuid, code: UnitCode, name: UnitName, can_have_students: bool) -> Self {
        Self {
            id: UnitTypeId::load(id),
            code,
            name,
            can_have_students,
        }
    }
}

impl Unit {
    pub fn new(
        name: UnitName,
        short_name: ShortName,
        parent: Option<&Unit>,
        is_active: bool,
        type_id: UnitTypeId,
    ) -> Result<Self, DomainError> {

        let path = match parent {
            Some(parent) => {
                UnitPath::from_parent(parent)
            }
            None => UnitPath::new()
        };


        Ok(Self {
            id: UnitId::new(),
            name,
            short_name,
            parent_id: parent.map(|parent| parent.id),
            is_active,
            type_id,
            created_at: OffsetDateTime::now_utc(),
            path
        })
    }

    pub fn load(
        id: &Uuid,
        name: UnitName,
        short_name: ShortName,
        parent_id: Option<Uuid>,
        is_active: bool,
        type_id: &Uuid,
        created_at: OffsetDateTime,
        path: Vec<Uuid>
    ) -> Self {
        let parent_id = match parent_id {
            Some(parent_id) => Some(UnitId::load(&parent_id)),
            None => None
        };
        Self {
            id: UnitId::load(id),
            name,
            short_name,
            parent_id,
            is_active,
            type_id: UnitTypeId::load(type_id),
            created_at,
            path: UnitPath::load(path.iter().map(|u| UnitId::load(u)).collect()),
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
}