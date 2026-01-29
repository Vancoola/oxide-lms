mod course_status;

use uuid::Uuid;
use crate::course::course_status::CourseStatus;

pub struct Course {

    pub id: Uuid,
    pub status: CourseStatus,

    pub title: String,
    pub description: String,

}