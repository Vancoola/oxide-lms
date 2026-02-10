use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::organizational_unit::Unit;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnitTypeId(Uuid);


impl UnitTypeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn load(id: &Uuid) -> Self {
        Self(id.to_owned())
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnitId(Uuid);

impl UnitId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn load(id: &Uuid) -> Self {
        Self(id.to_owned())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitCode(String);
impl UnitCode {
    pub fn new(code: String) -> Self {
        let code = code.trim().to_uppercase();
        Self(code)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitName(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortName(String);
impl ShortName {
    pub fn new(name: String) -> Self {
        let name = name.trim().to_uppercase();
        Self(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitPath(Vec<UnitId>);

impl UnitPath {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn load(path: Vec<UnitId>) -> Self {
        Self(path)
    }
    pub fn from_parent(parent: &Unit) -> Self {
        let mut new_path = parent.path.clone();
        new_path.push(&parent.id);
        new_path
    }

    pub fn push(&mut self, id: &UnitId) {
        self.0.push(id.clone());
    }
}