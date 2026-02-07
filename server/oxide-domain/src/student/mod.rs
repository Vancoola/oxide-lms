pub mod repository;
pub mod service;

use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum TrainingStatus {
    Applicant,     // Абитуриент
    Enrolled,      // Зачислен (но еще не начал учебу)
    Studying,      // Обучается
    AcademicLeave, // В академическом отпуске
    Graduate,      // Выпускник
    Expelled,      // Отчислен
    Transferred,   // Переведен в другой вуз
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub user_id: Uuid,

    pub student_id: Option<String>,
    pub status: TrainingStatus,

    pub organizational_unit_id: Uuid,

    pub enrolled_at: Option<OffsetDateTime>,
    pub graduated_at: Option<OffsetDateTime>,
}

impl Student {
    pub fn new(
        user_id: Uuid,
        student_id: Option<String>,
        status: TrainingStatus,
        organizational_unit_id: Uuid,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            student_id,
            status,
            organizational_unit_id,
            enrolled_at: None,
            graduated_at: None,
        })
    }

    pub fn load(
        id: Uuid,
        user_id: Uuid,
        student_id: Option<String>,
        status: TrainingStatus,
        organizational_unit_id: Uuid,
        enrolled_at: Option<OffsetDateTime>,
        graduated_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            user_id,
            student_id,
            status,
            organizational_unit_id,
            enrolled_at,
            graduated_at,
        }
    }
}
