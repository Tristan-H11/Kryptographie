use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .service(
                web::resource("")
                    .route(web::get().to(healthcheck))
            )
    ).default_service(web::route().to(not_found));
}

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