use actix_web::{web, HttpResponse, Responder};
use bigdecimal::num_bigint::ParseBigIntError;
use log::info;
use serde::Serialize;

use crate::api::endpoints::math::exponentiation::exponentiation;
use crate::api::endpoints::math::extended_euclid::euclid_endpoint;
use crate::api::endpoints::math::modular_inverse::modular_inverse_endpoint;
use crate::api::endpoints::math::shanks::shanks_endpoint;
use crate::api::endpoints::rsa::create_key_pair::create_key_pair;
use crate::api::endpoints::rsa::decrypt::decrypt;
use crate::api::endpoints::rsa::encrypt::encrypt;
use crate::api::endpoints::rsa::multiplication::multiplication;
use crate::api::endpoints::rsa::sign::sign;
use crate::api::endpoints::rsa::verify::verify;
use crate::api::serializable_models::SingleStringResponse;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(healthcheck)))
        .service(
            web::scope("/rsa")
                .route("/createKeyPair", web::post().to(create_key_pair))
                .route("/encrypt", web::post().to(encrypt))
                .route("/decrypt", web::post().to(decrypt))
                .route("/sign", web::post().to(sign))
                .route("/verify", web::post().to(verify))
                .route("/multiplication", web::post().to(multiplication)),
        )
        .service(
            web::scope("/math")
                .route("/exponentiation", web::post().to(exponentiation))
                .route("/extended_euclid", web::post().to(euclid_endpoint))
                .route("/shanks", web::post().to(shanks_endpoint))
                .route("/modular_inverse", web::post().to(modular_inverse_endpoint)),
        )
        .default_service(web::route().to(not_found));
}

async fn healthcheck() -> impl Responder {
    info!("Endpunkt /health wurde aufgerufen");
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> HttpResponse {
    info!("Endpunkt wurde nicht gefunden");
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

/// Nimmt eine Closure entgegen, die eine Antwort zurückgibt und ruft diese Funktion auf.
/// Gibt die Antwort der Funktion zurück.
/// Falls ein Fehler produziert wird, wird ein BadRequest zurückgegeben.
pub fn call_checked_with_parsed_big_ints(func: impl Fn() -> Result<HttpResponse, ParseBigIntError>) -> HttpResponse {
    func().unwrap_or_else(|_| HttpResponse::BadRequest().json(SingleStringResponse {
        message: "Fehler beim Parsen der Parameter".to_string(),
    }))
}