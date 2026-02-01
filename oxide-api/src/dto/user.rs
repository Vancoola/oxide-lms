use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct UserInfo{
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub group_code: String,
}