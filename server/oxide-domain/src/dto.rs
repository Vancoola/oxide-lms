use crate::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDbRow {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}


