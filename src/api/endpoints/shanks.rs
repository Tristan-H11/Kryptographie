use std::str::FromStr;

use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;

use crate::api::serializable_models::{ShanksRequest, SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::babystep_giantstep::shanks;

/**
 * Führt den erweiterten Euklidischen Algorithmus aus
 */
pub(crate) async fn shanks_endpoint(
    req_body: Json<ShanksRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /math/shanks wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: ShanksRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let base = BigInt::from_str(&req_body.base).unwrap();
    let element = BigInt::from_str(&req_body.element).unwrap();
    let modul = BigInt::from_str(&req_body.modul).unwrap();
    let result = shanks(&base, &element, &modul, use_fast);
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
