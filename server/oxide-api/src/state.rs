use std::sync::Arc;
use sqlx::PgPool;
use oxide_data::PostgresContext;
use oxide_domain::crypto::PasswordHasher;
use oxide_domain::profile::repository::ProfileRepository;
use oxide_domain::user::plugin::UserExtensionRegistry;
use oxide_domain::user::repository::UserRepository;
use oxide_infrastructure::auth::argon2::Argon2Hasher;
use crate::error::AppError;

pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_extension_registry: Arc<UserExtensionRegistry>,
}

impl AppState {
    pub async fn new(pg_pool: PgPool) -> Result<Self, AppError> {
        let context = Arc::new(PostgresContext::new(pg_pool.clone()));
        Ok(Self{
            user_repo: context.clone(),
            profile_repo: context.clone(),
            password_hasher: Arc::new(Argon2Hasher),
            user_extension_registry: Arc::new(UserExtensionRegistry::new())
        })

    }
}