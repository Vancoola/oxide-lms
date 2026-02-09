use std::fmt::{Debug, Formatter};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::crypto::PasswordHasher;
use crate::error::DomainError;


#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct UserId(Uuid);
impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn load(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        if email.is_empty() {
            return Err(DomainError::InvalidInputValue("email cannot be empty".to_string()));
        }
        if !EmailAddress::is_valid(&email) {
            return Err(DomainError::InvalidInputValue("invalid email".to_string()));
        }
        Ok(Email(email))
    }

    pub(crate) fn load(email: String) -> Self {
        Email(email)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct RawPassword(String);
impl RawPassword {
    pub fn new(password: String) -> Result<Self, DomainError> {
        if password.is_empty() {
            return Err(DomainError::InvalidInputValue("password cannot be empty".to_string()));
        }
        if password.len() < 8 {
            return Err(DomainError::InvalidInputValue("password must be at least 8 characters".to_string()));
        }
        Ok(Self(password))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl Debug for RawPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RawPassword(***)")
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Password(String);
impl Password {

    pub fn from_hash(password: String) -> Self {
        Self(password)
    }

    pub fn verify<H: PasswordHasher>(&self, plain: &RawPassword, hasher: &H) -> bool {
        hasher.verify(plain, self).unwrap_or(false)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

}


#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case("true@example.com")]
    #[case("user.name@domain.co.uk")]
    fn email_valid_input_successfully(#[case] input: String) {
        let email = Email::new(input);
        assert!(email.is_ok());
    }

    #[rstest]
    #[case("")]
    #[case("plainaddress")]
    #[case("@missing-local.com")]
    #[case("test@")]
    fn email_invalid_input_error(#[case] input: String) {
        let result = Email::new(input);
        assert!(result.is_err());
    }



    #[rstest]
    #[case("mysecretpassword")]
    #[case("mysecretpassword12232")]
    #[case("1234567890")]
    fn raw_password_valid_input_successfully(#[case] input: String){
        let result = RawPassword::new(input);
        assert!(result.is_ok());
    }

    #[rstest]
    #[case("")]
    #[case("1234567")]
    fn raw_password_invalid_input_error(#[case] input: String){
        let result = RawPassword::new(input);
        assert!(result.is_err());
    }
}