use crate::error::DomainError;
use crate::user::object::{Password, RawPassword};

#[cfg_attr(test, mockall::automock)]
pub trait PasswordHasher: Send + Sync  {
    fn hash(&self, password: &RawPassword) -> Result<String, DomainError>;
    fn verify(&self, password: &RawPassword, hash: &Password) -> Result<bool, DomainError>;
}