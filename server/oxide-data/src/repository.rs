use async_trait::async_trait;
use oxide_domain::error::DomainError;
use oxide_domain::profile::Profile;
use oxide_domain::profile::repository::ProfileRepository;
use oxide_domain::user::User;
use oxide_domain::user::repository::UserRepository;
use sqlx::types::Uuid;

#[derive(Clone)]
pub struct PostgresContext {
    pool: sqlx::PgPool,
}

impl PostgresContext {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

fn to_domain_err(err: sqlx::Error) -> DomainError {
    match err {
        sqlx::Error::RowNotFound => DomainError::NotFound,
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            DomainError::AlreadyExists
        }
        _ => DomainError::Infrastructure(err.to_string()),
    }
}

#[async_trait]
impl UserRepository for PostgresContext {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT u.id, u.email, u.password, u.is_admin
            FROM users u
            WHERE u.id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok(User::load(
            record.id,
            record.email.as_str(),
            record.password.as_str(),
            record.is_admin,
        ))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT u.id, u.email, u.password, u.is_admin
            FROM users u
            WHERE u.email = $1"#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok(User::load(
            record.id,
            record.email.as_str(),
            record.password.as_str(),
            record.is_admin,
        ))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError> {
        todo!()
    }

    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, password, is_admin) VALUES ($1, $2, $3, $4)"#,
            user.id,
            user.email,
            user.password,
            user.is_admin
        )
        .execute(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok(())
    }

    async fn get_password_by_email(&self, email: &str) -> Result<(Uuid, String), DomainError> {
        let record = sqlx::query!(r#"SELECT id, password FROM users WHERE email = $1"#, &email)
            .fetch_one(&self.pool)
            .await
            .map_err(to_domain_err)?;
        Ok((record.id, record.password))
    }
}

#[async_trait]
impl ProfileRepository for PostgresContext {
    async fn get_profile_by_id(&self, id: Uuid) -> Result<Profile, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT *
            FROM profiles
            WHERE id=$1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok(Profile::load(
            record.id,
            record.user_id,
            record.first_name,
            record.last_name,
            record.middle_name,
            record.created_at,
            record.updated_at,
            record.is_active,
        ))
    }

    async fn get_profile_by_uid(&self, uid: Uuid) -> Result<Profile, DomainError> {
        todo!()
    }

    async fn exists_profile_by_uid(&self, uid: Uuid) -> Result<bool, DomainError> {
        todo!()
    }

    async fn create_profile(&self, profile: &Profile) -> Result<Uuid, DomainError> {
        todo!()
    }

    async fn update_profile(&self, profile: &Profile) -> Result<Uuid, DomainError> {
        todo!()
    }
}
