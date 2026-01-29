use sqlx::types::Uuid;
use oxide_domain::dto::{CreateUser, PublicUser, UserDbRow};
use oxide_domain::model::User;
use crate::error::DataError;

pub struct PostgresContext {
    pool: sqlx::PgPool,
}

impl PostgresContext {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<PublicUser, DataError>{
        let record = sqlx::query!("SELECT id, email, name FROM users WHERE id = $1", &id)
            .fetch_one(&self.pool)
            .await?;
        Ok(PublicUser{
            id: record.id,
            email: record.email,
            name: record.name
        })
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<PublicUser, DataError> {
        let record = sqlx::query!("SELECT id, email, name FROM users WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await?;
        Ok(PublicUser{
            id: record.id,
            email: record.email,
            name: record.name
        })
    }

    pub async fn create_user(&self, user: User) -> Result<(), DataError> {
        let row = UserDbRow::from(user);
        sqlx::query!("INSERT INTO users (id, email, password, name) VALUES ($1, $2, $3, $4)", row.id, row.email, row.password, row.name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

}