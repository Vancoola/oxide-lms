use crate::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        User::new(
            value.email.as_str(),
            value.password.as_str(),
        ).unwrap()
    }
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

impl From<User> for UserDbRow {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password: value.password,
        }
    }
}
