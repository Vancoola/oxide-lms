use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "training_status", rename_all = "snake_case")]
// pub enum TrainingStatus {
//     Applicant,     // Абитуриент
//     Enrolled,      // Зачислен (но еще не начал учебу)
//     Studying,      // Обучается
//     AcademicLeave, // В академическом отпуске
//     Graduate,      // Выпускник
//     Expelled,      // Отчислен
//     Transferred,   // Переведен в другой вуз
// }
//
// #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "degree", rename_all = "lowercase")]
// pub enum Degree {
//     Bachelor,     // Бакалавриат
//     Specialist,   // Специалитет
//     Master,       // Магистратура
//     Postgraduate, // Аспирантура
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Speciality {
//     pub id: Uuid,
//     //pub department_id: Option<Uuid>,  // может быть без кафедры
//     pub faculty_id: Uuid,
//     pub code: String,   // "09.03.01"
//     pub name: String,   // "Информатика и вычислительная техника"
//     pub degree: Degree, // бакалавр/магистр и т.д.
//     pub is_active: bool,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct StudentAccount {
//     pub id: Uuid,
//     pub user_id: Uuid,
//
//     pub status: TrainingStatus,
//
//     pub faculty_id: Uuid,
//     pub group_id: Option<Uuid>,               // учебная группа
//     pub enrolled_at: Option<OffsetDateTime>,  // дата зачисления
//     pub graduated_at: Option<OffsetDateTime>, // дата выпуска
//     pub student_id_number: Option<String>,    // номер студенческого
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Staff {
    pub id: Uuid,
    pub user_id: Uuid,

    pub faculty_id: Uuid,
}
