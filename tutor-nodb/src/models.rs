use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone, Validate)]
pub struct Course {
    #[validate(range(min = 1, max = 100, message = "100 이하의 값을 입력하세요"))]
    #[schema(example = 1)]
    pub tutor_id: i32,

    #[validate(range(min = 1, max = 100))]
    #[schema(example = 1)]
    pub course_id: Option<i32>,

    #[validate(length(min = 3, message = "3글자 이상 입력하세요"))]
    #[schema(example = "first course!")]
    pub course_name: String,

    #[schema(value_type = String, format = DateTime)]
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}
