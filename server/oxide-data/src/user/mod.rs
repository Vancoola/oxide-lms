use crate::{PostgresContext, to_domain_err};
use async_trait::async_trait;
use oxide_domain::error::DomainError;
use oxide_domain::user::User;
use oxide_domain::user::object::Email;
use oxide_domain::user::repository::UserRepository;
use sqlx::types::Uuid;
use oxide_domain::event::GlobalEvent;

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

    async fn get_user_by_email(&self, email: &Email) -> Result<User, DomainError> {
        let record = sqlx::query!(
            r#"
            SELECT u.id, u.email, u.password, u.is_admin
            FROM users u
            WHERE u.email = $1"#,
            email.as_str()
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

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        let record = sqlx::query!(
            "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
            email.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_domain_err)?;
        match record.exists {
            None => Ok(false),
            Some(b) => Ok(b),
        }
    }

    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, password, is_admin) VALUES ($1, $2, $3, $4)"#,
            user.id.as_ref(),
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
        //TODO: The Outbox pattern should be here
        let mut tx = self.pool.begin().await.map_err(to_domain_err)?;
        sqlx::query!(
            r#"INSERT INTO users (id, email, password, is_admin) VALUES ($1, $2, $3, $4)"#,
            user.id.as_ref(),
            user.email.as_str(),
            user.password_hash.as_str(),
            user.is_admin
        )
        .execute(&mut *tx)
        .await
        .map_err(to_domain_err)?;

        for event in user.pull_events() {
            let payload = serde_json::to_value(GlobalEvent::User(event))
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
            let event_type = "UserEvent";
            sqlx::query!(
                r#"INSERT INTO outbox_events (event_type, payload) VALUES ($1, $2)"#,
                event_type,
                payload
            )
            .execute(&mut *tx)
            .await
            .map_err(to_domain_err)?;
        }
        tx.commit().await.map_err(to_domain_err)?;
        Ok(())
    }

    async fn get_password_by_email(&self, email: &Email) -> Result<(Uuid, String), DomainError> {
        let record = sqlx::query!(
            r#"SELECT id, password FROM users WHERE email = $1"#,
            email.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok((record.id, record.password))
    }
}
