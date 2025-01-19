use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// route 구성
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // '/health' 경로로 유입되는 HTTP GET 요청을 health_check_handler()로 전달
    cfg.route("/health", web::get().to(health_check_handler));
}

// handler 구성
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Helllo. Ezytutors is alive and kicking")
}

// http 서버를 인스턴스화하고 실행
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // app을 만들고 라우트를 구성
    let app = move || App::new().configure(general_routes);

    // http 서버를 시작
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
