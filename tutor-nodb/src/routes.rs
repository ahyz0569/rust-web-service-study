use super::handlers::*;
use actix_web::web;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            utoipa_actix_web::scope("/courses")
                .service(new_course)
                .service(get_courses_for_tutor)
                .service(get_course_detail),
        );
    }
}
