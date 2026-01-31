use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::dto::CreateUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,

    pub email: String,
    pub(crate) password: String,

    pub name: String,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Self {
        Self { id: Uuid::new_v4(), email, password, name, }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faculty {
    pub id: Uuid,
    pub name: String,
    pub short_name: String,  // "ИТИ", "ФЭМ" и т.д.
    pub dean_id: Uuid,
    pub is_active: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AcademicGroup {
    pub id: Uuid,
    pub faculty_id: Uuid,
    pub code: String,           // например "ИТ-21-1"
    pub name: Option<String>,   // полное название (опционально)
    pub course: i16,            // лучше i16 для совместимости с SQL
    pub speciality_id: Uuid,    // ссылка на специальность
    pub curator_id: Option<Uuid>, // куратор (преподаватель)
    pub created_at: OffsetDateTime,
    pub is_active: bool,        // архивная или действующая группа
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "training_status", rename_all = "lowercase")]
pub enum TrainingStatus {
    Applicant,          // Абитуриент
    Enrolled,           // Зачислен (но еще не начал учебу)
    Studying,           // Обучается
    AcademicLeave,      // В академическом отпуске
    Graduate,           // Выпускник
    Expelled,           // Отчислен
    Transferred,        // Переведен в другой вуз
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "degree", rename_all = "lowercase")]
pub enum Degree {
    Bachelor,      // Бакалавриат
    Specialist,    // Специалитет
    Master,        // Магистратура
    Postgraduate,  // Аспирантура
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speciality {
    pub id: Uuid,
    //pub department_id: Option<Uuid>,  // может быть без кафедры
    pub faculty_id: Uuid,
    pub code: String,      // "09.03.01"
    pub name: String,      // "Информатика и вычислительная техника"
    pub degree: Degree,    // бакалавр/магистр и т.д.
    pub is_active: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,

    pub status: TrainingStatus,

    pub speciality_id: Uuid,
    pub faculty_id: Uuid,
    pub group_id: Option<Uuid>,               // учебная группа

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub enrolled_at: Option<OffsetDateTime>,  // дата зачисления
    pub graduated_at: Option<OffsetDateTime>, // дата выпуска
    pub student_id_number: Option<String>,    // номер студенческого
}


