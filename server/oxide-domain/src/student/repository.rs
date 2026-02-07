use async_trait::async_trait;
use uuid::Uuid;
use crate::error::DomainError;
use crate::student::Student;

#[async_trait]
pub trait StudentRepository {
    async fn add_student(&self, student: &Student) -> Result<Student, DomainError>;
    async fn exists_student_by_uid(&self, uid: Uuid) -> Result<bool, DomainError>;
}