use oxide_domain::error::DomainError;
use oxide_domain::profile::Profile;
use oxide_domain::profile::repository::ProfileRepository;
use uuid::Uuid;

pub async fn create_profile(
    repo: &dyn ProfileRepository,
    user_id: Uuid,
) -> Result<Profile, DomainError> {
    if repo.exists_profile_by_uid(user_id).await? {
        return Err(DomainError::AlreadyExists);
    }
    let profile = Profile::new(user_id)?;
    repo.create_profile(&profile).await?;
    Ok(profile)
}

pub async fn update_profile(
    repo: &dyn ProfileRepository,
    user_id: Uuid,
    first_name: &str,
    last_name: &str,
    middle_name: Option<String>,
) -> Result<Profile, DomainError> {
    if !repo.exists_profile_by_uid(user_id).await? {
        return Err(DomainError::NotFound);
    }
    let mut profile = repo.get_profile_by_uid(user_id).await?;
    profile.update(first_name, last_name, middle_name)?;
    repo.update_profile(&profile).await?;
    Ok(profile)
}
