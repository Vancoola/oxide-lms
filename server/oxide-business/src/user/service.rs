use std::sync::Arc;
use oxide_domain::crypto::PasswordHasher;
use oxide_domain::error::DomainError;
use oxide_domain::user::object::{Email, Password, RawPassword};
use oxide_domain::user::plugin::UserExtensionRegistry;
use oxide_domain::user::repository::UserRepository;
use oxide_domain::user::User;

pub async fn register_user<R: UserRepository, H: PasswordHasher>(
    repo: &R,
    hasher: &H,
    plugin_registry: &UserExtensionRegistry,
    email: Email,
    password: RawPassword,
) -> Result<User, DomainError> {
    for guard in &plugin_registry.guards {
        guard.execute(&email).await?;
    }
    if repo.exists_by_email(email.as_str()).await? {
        return Err(DomainError::AlreadyExists);
    }
    let hash = hasher.hash(&password)?;
    let mut user = User::new(email, Password::from_hash(hash));
    for middleware in &plugin_registry.middlewares {
        middleware.transform(&mut user).await?;
    }
    repo.create_user_and_publish(&mut user).await?;
    for hook in &plugin_registry.hooks {
        let u = user.clone();
        let h = Arc::clone(hook);
        tokio::spawn(async move {
            h.execute(&u).await;
        });
    }
    Ok(user)
}

pub async fn register_admin<R: UserRepository, H: PasswordHasher>(
    repo: &R,
    hasher: &H,
    plugin_registry: &UserExtensionRegistry,
    email: Email,
    password: RawPassword,
) -> Result<User, DomainError> {
    for guard in &plugin_registry.guards {
        guard.execute(&email).await?;
    }
    if repo.exists_by_email(email.as_str()).await? {
        return Err(DomainError::AlreadyExists);
    }
    let hash = hasher.hash(&password)?;
    let mut user = User::new_admin(email, Password::from_hash(hash));
    for middleware in &plugin_registry.middlewares {
        middleware.transform(&mut user).await?;
    }
    repo.create_user_and_publish(&mut user).await?;
    for hook in &plugin_registry.hooks {
        let u = user.clone();
        let h = Arc::clone(hook);
        tokio::spawn(async move {
            h.execute(&u).await;
        });
    }
    Ok(user)
}



// #[cfg(test)]
// mod test {
//     use rstest::rstest;
//     use oxide_domain::crypto::MockPasswordHasher;
//     use crate::user::repository::MockUserRepository;
//     use super::*;
// 
//     #[rstest]
//     #[case("true@example.com", "mysecretpassword")]
//     #[tokio::test]
//     async fn register_user_valid_input_successfully(#[case] input_email: String, #[case] input_password: String) {
// 
//         let email = Email::new(input_email.clone()).unwrap();
//         let password = RawPassword::new(input_password.clone()).unwrap();
// 
//         let mut mock_repo = MockUserRepository::new();
//         let email_clone = input_email.clone();
//         let password_clone = input_password.clone();
//         mock_repo.expect_exists_by_email()
//             .withf(move |email| email == input_email)
//             .times(1)
//             .returning(move |_| Ok(false));
//         mock_repo.expect_create_user_and_publish()
//             .withf(move |u| {
//                 (u.email.as_str() == email_clone)
//                     & (u.password_hash.as_str() != password_clone)
//                     & (u.is_admin == false)
//             })
//             .times(1)
//             .returning(|_| Ok(()));
// 
//         let mut mock_hasher = MockPasswordHasher::new();
//         mock_hasher
//             .expect_hash()
//             .withf(move |rp| rp.as_str() == input_password)
//             .returning(|_| Ok("hashed".to_string()));
// 
//         let result = register_user(&mock_repo, &mock_hasher, email, password).await;
//         assert!(result.is_ok());
//     }
// 
//     #[rstest]
//     #[case("true@example.com", "mysecretpassword")]
//     #[tokio::test]
//     async fn register_user_valid_input_error_email_exists(#[case] input_email: String, #[case] input_password: String) {
// 
//         let email = Email::new(input_email.clone()).unwrap();
//         let password = RawPassword::new(input_password.clone()).unwrap();
// 
//         let mut mock_repo = MockUserRepository::new();
//         mock_repo.expect_exists_by_email()
//             .withf(move |email| email == input_email)
//             .times(1)
//             .returning(move |_| Ok(true));
// 
//         let mut mock_hasher = MockPasswordHasher::new();
//         mock_hasher
//             .expect_hash()
//             .withf(move |rp| rp.as_str() == input_password)
//             .returning(|_| Ok("hashed".to_string()));
// 
//         let result = register_user(&mock_repo, &mock_hasher, email, password).await;
//         assert!(result.is_err());
//         assert_eq!(result.err().unwrap(), DomainError::AlreadyExists);
//     }
// 
// }