use sqlx::PgPool;
use oxide_business::jwt::generate_jwt;
use oxide_domain::dto::CreateUser;
use oxide_domain::model::User;
use crate::dto::JwtToken;
use crate::error::DataError;
use crate::repository::PostgresContext;

#[derive(Clone)]
pub struct UserManager {
    context: PostgresContext,
}

impl UserManager{
    pub fn new(pool: PgPool) -> Self {
        Self { context: PostgresContext::new(pool) }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<JwtToken, DataError> {
        let user_model = User::from(user);
        let secret = "your-super-secret-key";
        let token = generate_jwt(&user_model.id, secret)?;
        self.context.create_user(user_model).await?;
        Ok(JwtToken { token })
    }

    pub async fn login_user(&self, email: &str, password: &str) -> Result<JwtToken, DataError> {
        let (id, password) = self.context.get_password_by_email(email).await?;
        if password != password {
            return Err(DataError::InvalidPassword)
        }
        let secret = "your-super-secret-key";
        let token = generate_jwt(&id, secret)?;
        Ok(JwtToken { token })
    }


}