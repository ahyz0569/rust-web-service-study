use actix_web::{
    middleware::Logger,
    web, App, HttpServer
};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;
use std::io;
use std::sync::Mutex;
use utoipa::{openapi, OpenApi};

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
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
            .service(utoipa_actix_web::scope("/courses").configure(routes::course_routes(shared_data.clone())))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}