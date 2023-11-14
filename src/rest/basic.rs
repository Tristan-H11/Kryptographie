use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub fn configure_http_server() -> Server {
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
        .bind(("127.0.0.1", 8080)).expect("Can not bind to port 8080")
        .run()
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> HttpResponse {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}