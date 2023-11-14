mod encryption;
mod tests;
mod rest;

use actix_web::{App, HttpServer, web};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();


    HttpServer::new(|| App::new().service(healthcheck).default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
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