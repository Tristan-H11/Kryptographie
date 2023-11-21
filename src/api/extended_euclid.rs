use std::str::FromStr;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use bigdecimal::num_bigint::BigInt;
use log::info;
use crate::api::serializable_models::{ExtendedEuclidRequest, ExtendedEuclidResponse, SingleStringResponse};
use crate::encryption::math_functions::number_theory::extended_euclid;

/**
* Führt den erweiterten Euklidischen Algorithmus aus
*/
pub(crate) async fn euclid_endpoint(req_body: Json<ExtendedEuclidRequest>) -> impl Responder {
    info!("Endpunkt /math/extended_euclid wurde aufgerufen");
    let req_body: ExtendedEuclidRequest = req_body.into_inner();

    let a = &BigInt::from_str(&*req_body.a).unwrap();
    let b = &BigInt::from_str(&*req_body.b).unwrap();

    let (ggt, x, y) = extended_euclid(a, b);

    let response = ExtendedEuclidResponse {
        x: x.to_str_radix(10),
        y: y.to_str_radix(10),
        ggt: ggt.to_str_radix(10),
    };

    HttpResponse::Ok().json(response)
}