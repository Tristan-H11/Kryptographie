use std::str::FromStr;

use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use bigdecimal::num_bigint::BigInt;
use log::info;

use crate::api::serializable_models::{ExponentiationRequest, SingleStringResponse};
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;

/**
 * Führt die Exponentiation aus
 */
pub(crate) async fn exponentiation(req_body: Json<ExponentiationRequest>) -> impl Responder {
    info!("Endpunkt /math/exponentiation wurde aufgerufen");
    let req_body: ExponentiationRequest = req_body.into_inner();

    let exponent = &BigInt::from_str(&*req_body.exponent).unwrap();
    let base = &BigInt::from_str(&*req_body.base).unwrap();
    let modulus = &BigInt::from_str(&*req_body.modulus).unwrap();

    let result = FastExponentiation::calculate(base, exponent, modulus, false).to_str_radix(10); //TODO UseFast einbauen


    let response = SingleStringResponse {
        message: result
    };

    HttpResponse::Ok().json(response)
}