pub mod event;
pub mod object;
pub mod plugin;
pub mod repository;

use crate::crypto::PasswordHasher;
use crate::user::event::UserEvent;
use crate::user::event::UserEvent::Created;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::user::object::{Email, Password, RawPassword, UserId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,

    pub email: Email,
    #[serde(skip_serializing)]
    pub password_hash: Password,

    pub is_admin: bool,

    events: Vec<UserEvent>,
}

impl User {
    pub fn new(email: Email, password_hash: Password) -> Self {
        let mut user = Self {
            id: UserId::new(),
            email,
            password_hash,
            is_admin: false,
            events: Vec::new(),
        };
        user.events.push(Created {
            user_id: *user.id.as_uuid(),
            email: user.email.as_str().into(),
        });
        user
    }

    pub fn new_admin(email: Email, password_hash: Password) -> Self {
        let mut user = Self {
            id: UserId::new(),
            email,
            password_hash,
            is_admin: true,
            events: Vec::new(),
        };
        user.events.push(Created {
            user_id: *user.id.as_uuid(),
            email: user.email.as_str().into(),
        });
        user
    }

    pub fn load(id: Uuid, email: String, password_hash: String, is_admin: bool) -> Self {
        Self {
            id: UserId::load(id),
            email: Email::load(email),
            password_hash: Password::from_hash(password_hash),
            is_admin,
            events: Vec::new(),
        }
    }

    pub fn pull_events(&mut self) -> Vec<UserEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn check_password<H: PasswordHasher>(&self, plain: &RawPassword, hasher: &H) -> bool {
        self.password_hash.verify(plain, hasher)
    }
}

// #[cfg(test)]
// mod test {
//     use rstest::rstest;
//     use super::*;
//
//     // #[rstest]
//     // #[case("true@example.com", "secretpassword")]
//     // fn create_user(#[case] email: String, #[case] password: String) {
//     //     let user = User::new(email.clone(), password.clone());
//     //     assert!(user.is_ok());
//     //     let mut user = user.unwrap();
//     //     assert_eq!(user.email.0, email);
//     //     assert!(&!user.is_admin);
//     //
//     //     let has_created_event = user.pull_events().iter().any(|e| matches!(e, Created {..}));
//     //     assert!(has_created_event);
//     //
//     // }
//     //
//     // #[rstest]
//     // #[case("true@example.com", "secretpassword")]
//     // fn crate_admin(#[case] email: String, #[case] password: String) {
//     //     let user = User::new_admin(email.clone(), password.clone());
//     //     assert!(user.is_ok());
//     //     let user = user.unwrap();
//     //     assert_eq!(user.email.0, email);
//     //     assert!(user.is_admin);
//     // }
//
//
// }
