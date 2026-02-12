use crate::{PostgresContext, to_domain_err};
use async_trait::async_trait;
use oxide_domain::error::DomainError;
use oxide_domain::profile::Profile;
use oxide_domain::profile::repository::ProfileRepository;
use sqlx::types::Uuid;

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

    async fn get_profile_by_uid(&self, _uid: Uuid) -> Result<Profile, DomainError> {
        todo!()
    }

    async fn exists_profile_by_uid(&self, uid: Uuid) -> Result<bool, DomainError> {
        let record = sqlx::query!("SELECT EXISTS(SELECT * FROM profiles WHERE id = $1)", uid)
            .fetch_one(&self.pool)
            .await.map_err(to_domain_err)?;
        match record.exists {
            None => {Ok(false)},
            Some(b) => {Ok(b)}
        }
    }

    async fn create_profile(&self, profile: &Profile) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT
            INTO profiles(
                          id,
                          user_id,
                          first_name,
                          last_name,
                          middle_name,
                          is_active,
                          created_at,
                          updated_at
                          )
            VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8
            )"#,
            profile.id.as_ref(),
            profile.user_id,
            profile.first_name.as_deref(),
            profile.last_name.as_deref(),
            profile.middle_name.as_deref(),
            profile.is_activate,
            profile.created_at,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(to_domain_err)?;
        Ok(())
    }

    async fn update_profile(&self, _profile: &Profile) -> Result<(), DomainError> {
        todo!()
    }
}
