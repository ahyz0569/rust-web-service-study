use super::state::AppState;
use actix_web::{web, HttpResponse};

// Actix 웹 애플리케이션에 등록된 애플리케이션 상태는 자동으로 모든 핸들러 함수들이 web::Data<T> 라는 추출자 객체(extractor object)를 사용해 접근할 수 있음
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}