use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use crate::rest::key_pair::create_key_pair;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/health")
                .route("", web::get().to(healthcheck))
        )
        .service(
            web::scope("/rsa")
                .route("/createKeyPair", web::post().to(create_key_pair))
                // .route("/encrypt", web::post().to(encrypt))
                // .route("/decrypt", web::post().to(decrypt))
                // .route("/sign", web::post().to(signieren))
                // .route("/verify", web::post().to(verify))
        )
        .default_service(web::route().to(not_found));
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