use actix_web::{
    middleware::Logger,
    web, App, HttpServer
};
use routes::general_routes;
use std::io;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use state::AppState;

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
            .configure(
                routes::course_routes(),
            )
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}