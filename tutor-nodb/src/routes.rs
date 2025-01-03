use crate::state::AppState;

use super::handlers::*;
use actix_web::web;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(shared_data: web::Data<AppState>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(shared_data)
            .service(new_course)
            .service(get_courses_for_tutor)
            .service(get_course_detail);
    }
}
