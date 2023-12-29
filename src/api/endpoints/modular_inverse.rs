use std::str::FromStr;

use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Query};
use bigdecimal::num_bigint::BigInt;
use log::info;

use crate::api::serializable_models::{ModulInverseRequest, SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

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

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };


    let result = number_theory_service.modulo_inverse(&n, &modul);

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
