use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::paths::ExternalDocumentation;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::tag::Tag;
use apistos::web::{get, post, resource, scope};
use apistos::SwaggerUIConfig;
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

// use routes::*;
use handlers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 애플리케이션 상태 초기화
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've alread asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        let spec = Spec {
            default_tags: vec!["api".to_owned()],

            tags: vec![Tag {
                name: "api".to_string(),
                description: Some("~0~0~0~0~".to_string()),
                ..Default::default()
            }],

            info: Info {
                title: "A well documented API".to_string(),
                description: Some("This is an API documented using Apistos.".to_string()),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/v0".to_string(),
                ..Default::default()
            }],
            external_docs: Some(ExternalDocumentation {
                description: Some("Find out more about Swagger".to_string()),
                url: "http://swagger.io".to_string(),
                ..Default::default()
            }),
            default_parameters: vec![],
        };

        App::new()
            .document(spec)
            .wrap(Logger::default())
            .app_data(shared_data.clone()) // 웹 애플리케이션에 애플리케이션 상태 등록
            // .configure(general_routes)
            // .configure(course_routes)
            .service(
                scope("/courses")
                    .service(resource("/").route(post().to(new_course)))
                    .service(resource("/{tutor_id}").route(get().to(get_courses_for_tutor)))
                    .service(
                        resource("/{tutor_id}/{course_id}").route(get().to(get_course_detail)),
                    ),
                // .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
                // .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail))
            )
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(SwaggerUIConfig::new(&"/swagger")),
            )
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
