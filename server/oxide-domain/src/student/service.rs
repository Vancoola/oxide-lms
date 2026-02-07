use uuid::Uuid;
use crate::error::DomainError;
use crate::student::repository::StudentRepository;
use crate::student::{Student, TrainingStatus};

pub async fn add_student<R: StudentRepository>(
    repo: &R,
    user_id: Uuid,
    student_id: Option<String>,
    status: TrainingStatus,
    organizational_unit_id: Uuid
) -> Result<Student, DomainError> {
    if repo.exists_student_by_uid(user_id).await? {
        return Err(DomainError::AlreadyExists);
    }
    let student = Student::new(user_id, student_id, status, organizational_unit_id)?;
    repo.add_student(&student).await?;
    Ok(student)
}