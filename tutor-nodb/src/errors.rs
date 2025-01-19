use std::fmt::Display;

use actix_web::{body, http, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CustomErrorResponse {
    pub custom_message: String,
    pub errors: Vec<String>,
}

impl Display for CustomErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl ResponseError for CustomErrorResponse {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
