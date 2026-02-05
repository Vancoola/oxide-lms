use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        User::new(value.email, value.password, value.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDbRow {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl From<User> for UserDbRow {
    fn from(value: User) -> Self {
        Self{
            id: value.id,
            email: value.email,
            password: value.password,
            name: value.name,
        }
    }
}