use std::str::FromStr;

use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;

use crate::api::serializable_models::{
    ModulInverseRequest, SingleStringResponse, UseFastQuery,
};
use crate::encryption::math_functions::number_theory::modulo_inverse::ModuloInverse;

/**
 *
 */
pub(crate) async fn modular_inverse_endpoint(
    req_body: Json<ModulInverseRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /math/modular_inverse wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: ModulInverseRequest = req_body.into_inner();
    let use_fast = query.use_fast;
    let n = BigInt::from_str(&req_body.n).unwrap();
    let modul = BigInt::from_str(&req_body.modul).unwrap();

    let result = ModuloInverse::calculate(&n, &modul, use_fast);

    match result {
        Ok(x) => {
            let response = SingleStringResponse {
                message: x.to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::BadRequest().json(SingleStringResponse {
            message: "Kein Ergebnis gefunden".to_string(),
        }),
    }
}
