use std::sync::Arc;
use sqlx::PgPool;
use oxide_config::AppConfig;
use oxide_data::PostgresContext;
use oxide_domain::crypto::PasswordHasher;
use oxide_domain::profile::repository::ProfileRepository;
use oxide_domain::user::plugin::UserExtensionRegistry;
use oxide_domain::user::repository::UserRepository;
use oxide_infrastructure::auth::argon2::Argon2Hasher;
use crate::error::AppError;

#[derive(axum::extract::FromRef)]
pub struct AppState {
    pub pool: PgPool,

    pub user_repo: Arc<dyn UserRepository>,
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_extension_registry: Arc<UserExtensionRegistry>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new(config: Arc<AppConfig>) -> Result<Self, AppError> {
        let pg_pool = PgPool::connect(config.database_url.as_str()).await?;
        let context = Arc::new(PostgresContext::new(pg_pool.clone()));
        Ok(Self{
            pool: pg_pool,
            user_repo: context.clone(),
            profile_repo: context.clone(),
            password_hasher: Arc::new(Argon2Hasher),
            user_extension_registry: Arc::new(UserExtensionRegistry::new()),
            config,
        })

    }
}