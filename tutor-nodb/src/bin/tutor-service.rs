use actix_web::HttpRequest;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_validation::validator::ValidatorErrorHandlerExt;
use routes::general_routes;
use std::borrow::Cow;
use std::io;
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::errors::CustomErrorResponse;

#[path = "../errors.rs"]
mod errors;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use state::AppState;

fn flatten_errors(
    errors: &ValidationErrors,
    path: Option<String>,
    indent: Option<u16>,
) -> Vec<(u16, String, &ValidationError)> {
    errors
        .errors()
        .iter()
        .flat_map(|(&field, err)| {
            let indent = indent.unwrap_or(0);
            let actual_path = path
                .as_ref()
                .map(|path| [path.as_str(), field].join("."))
                .unwrap_or_else(|| field.to_owned());
            match err {
                ValidationErrorsKind::Field(field_errors) => field_errors
                    .iter()
                    .map(|error| (indent, actual_path.clone(), error))
                    .collect::<Vec<_>>(),
                ValidationErrorsKind::List(list_error) => list_error
                    .iter()
                    .flat_map(|(index, errors)| {
                        let actual_path = format!("{}[{}]", actual_path.as_str(), index);
                        flatten_errors(errors, Some(actual_path), Some(indent + 1))
                    })
                    .collect::<Vec<_>>(),
                ValidationErrorsKind::Struct(struct_errors) => {
                    flatten_errors(struct_errors, Some(actual_path), Some(indent + 1))
                }
            }
        })
        .collect::<Vec<_>>()
}

fn error_handler(errors: validator::ValidationErrors, _: &HttpRequest) -> actix_web::Error {
    CustomErrorResponse {
        custom_message: "Validation error".to_string(),
        errors: flatten_errors(&errors, None, None)
            .iter()
            .map(|(_, field, err)| {
                let code = err.code.as_ref();

                format!(
                    "{}: {}",
                    field,
                    err.message.as_ref().unwrap_or(&Cow::Borrowed(code))
                )
            })
            .collect::<Vec<_>>(),
    }
    .into()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "course", description = "course management")
        )
    )]
    struct ApiDoc;

    // 애플리케이션 상태 초기화
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've alread asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .app_data(shared_data.clone())
            .configure(routes::course_routes())
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
            .validator_error_handler(Arc::new(error_handler))
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
