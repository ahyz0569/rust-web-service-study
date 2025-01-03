use super::models::Course;
use super::state::AppState;
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;

const COURSE: &str = "course";

// Actix 웹 애플리케이션에 등록된 애플리케이션 상태는 자동으로 모든 핸들러 함수들이 web::Data<T> 라는 추출자 객체(extractor object)를 사용해 접근할 수 있음
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

/// Create new course to in-memory storage.
#[utoipa::path(
    tag = COURSE,
    responses(
        (status = 200, description = "course added successfully"),
    )
)]
#[post("/")]
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");

    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1).try_into().unwrap()),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

/// get list courses of tutor by given tutor id.
#[utoipa::path(
    tag = COURSE,
    responses(
        (status = 200, description = "courses found from storage", body = [Course]) 
    ),
    params(
        ("tutor_id", description = "Unique id of tutor")
    )
)]
#[get("/{tutor_id}")]
pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

/// get course by given tutor id and course id
#[utoipa::path(
    tag = COURSE,
    responses(
        (status = 200, description = "course found from storage", body = [Course])
    ),
    params(
        ("tutor_id", description = "Unique id of tutor"),
        ("course_id", description = "Unique id of course of tutor"),
    )
)]
#[get("/{tutor_id}/{course_id}")]
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();

    let seleted_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.tutor_id == tutor_id && course.course_id == Some(course_id))
        .ok_or("Course not found");

    if let Ok(course) = seleted_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(new_course)
        ).await;

        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(course)
            .to_request();

        let resp = test::call_service(&app, req).await;
        let full_url = String::from(resp.request().full_url());
        println!("post url: {}", full_url);

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(get_courses_for_tutor),
        ).await;

        // let tutor_id: web::Path<i32> = web::Path::from(1);
        let req = test::TestRequest::get()
            .uri("/1")
            .param("tutor_id", "1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        let full_url = String::from(resp.request().full_url());
        println!("get url {}", full_url);

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(get_course_detail)
        ).await;

        // let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let req = test::TestRequest::get().uri("/1/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}