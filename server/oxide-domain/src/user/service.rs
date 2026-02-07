use crate::crypto::PasswordHasher;
use crate::error::DomainError;
use crate::user::User;
use crate::user::repository::UserRepository;

pub async fn register_user<R: UserRepository, H: PasswordHasher>(
    repo: &R,
    hasher: &H,
    email: &str,
    password: &str,
) -> Result<User, DomainError> {
    if repo.exists_by_email(&email).await? {
        return Err(DomainError::AlreadyExists);
    }
    let hash = hasher.hash(password)?;
    let user = User::new(email, hash.as_str())?;
    repo.create_user(&user).await?;
    Ok(user)
}

pub async fn register_admin<R: UserRepository, H: PasswordHasher>(
    repo: &R,
    hasher: &H,
    email: &str,
    password: &str,
) -> Result<User, DomainError> {
    if repo.exists_by_email(&email).await? {
        return Err(DomainError::AlreadyExists);
    }
    let hash = hasher.hash(password)?;
    let user = User::new_admin(email, hash.as_str())?;
    repo.create_user(&user).await?;
    Ok(user)
}
