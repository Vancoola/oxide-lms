use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct ProfileId(Uuid);
impl ProfileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn load(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_ref(&self) -> &Uuid {
        &self.0
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(Option<String>);
impl Name {
    pub fn none() -> Self {
        Self(None)
    }
    pub fn some(name: Option<String>) -> Self {
        Self(name)
    }
    pub fn as_deref(&self) -> Option<&str> {
        self.0.as_deref()
    }
}