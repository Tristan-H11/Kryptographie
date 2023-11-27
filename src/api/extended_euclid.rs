use std::str::FromStr;

use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Query};
use bigdecimal::num_bigint::BigInt;
use log::info;

use crate::api::serializable_models::{ExtendedEuclidRequest, ExtendedEuclidResponse, UseFastQuery};
use crate::encryption::math_functions::number_theory::extended_euclid::ExtendedEuclid;

/**
 * Führt den erweiterten Euklidischen Algorithmus aus
 */
pub(crate) async fn euclid_endpoint(req_body: Json<ExtendedEuclidRequest>, query: Query<UseFastQuery>) -> impl Responder {
    info!("Endpunkt /math/extended_euclid wurde aufgerufen, use_fast: {}", query.use_fast);
    let req_body: ExtendedEuclidRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let a = &BigInt::from_str(&*req_body.a).unwrap();
    let b = &BigInt::from_str(&*req_body.b).unwrap();

    let (ggt, x, y) = ExtendedEuclid::calculate(a, b, use_fast);

    let response = ExtendedEuclidResponse {
        x: x.to_str_radix(10),
        y: y.to_str_radix(10),
        ggt: ggt.to_str_radix(10),
    };

    HttpResponse::Ok().json(response)
}