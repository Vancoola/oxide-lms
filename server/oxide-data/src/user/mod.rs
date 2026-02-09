use async_trait::async_trait;
use oxide_domain::error::DomainError;
use oxide_domain::profile::Profile;
use oxide_domain::profile::repository::ProfileRepository;
use oxide_domain::user::User;
use sqlx::types::Uuid;
use oxide_domain::user::repository::UserRepository;
use crate::{to_domain_err, PostgresContext};



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
            record.email,
            record.password,
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
            record.email,
            record.password,
            record.is_admin,
        ))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError> {
        todo!()
    }

    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, password, is_admin) VALUES ($1, $2, $3, $4)"#,
            user.id.as_uuid(),
            user.email.as_str(),
            user.password_hash.as_str(),
            user.is_admin
        )
            .execute(&self.pool)
            .await
            .map_err(to_domain_err)?;
        Ok(())
    }

    async fn create_user_and_publish(&self, user: &mut User) -> Result<(), DomainError> {
        todo!()
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
