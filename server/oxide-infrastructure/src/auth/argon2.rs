use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use oxide_domain::crypto::PasswordHasher as OxidePasswordHasher;
use oxide_domain::error::DomainError;
use oxide_domain::user::object::{Password, RawPassword};

pub struct Argon2Hasher;

impl OxidePasswordHasher for Argon2Hasher {
    fn hash(&self, password: &RawPassword) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .to_string())
    }

    fn verify(&self, password: &RawPassword, hash: &Password) -> Result<bool, DomainError> {
        let parsed_hash = match PasswordHash::new(hash.as_str()) {
            Ok(h) => h,
            Err(_) => return Ok(false),
        };

        Ok(Argon2::default()
            .verify_password(password.as_str().as_bytes(), &parsed_hash)
            .is_ok())
    }
}
