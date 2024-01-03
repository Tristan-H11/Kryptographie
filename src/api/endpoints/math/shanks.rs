use std::str::FromStr;

use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use serde::Deserialize;

use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::babystep_giantstep::Shanks;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryService;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

#[derive(Deserialize)]
pub struct ShanksRequest {
    pub base: String,
    pub element: String,
    pub modul: String,
}

/// Berechnet den diskreten Logarithmus.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Berechnung des diskreten Logarithmus enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Ergebnis des diskreten Logarithmus enthält.
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

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    let shanks_service = Shanks::new(number_theory_service);

    call_checked_with_parsed_big_ints(|| {
        let base = BigInt::from_str(&req_body.base)?;
        let element = BigInt::from_str(&req_body.element)?;
        let modul = BigInt::from_str(&req_body.modul)?;

        let result = shanks_service.calculate(&base, &element, &modul);
        let response = match result {
            Ok(x) => {
                let response = SingleStringResponse {
                    message: x.to_string(),
                };
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::BadRequest().json(SingleStringResponse {
                message: "Fehler beim Berechnen des diskreten Logarithmus".to_string(),
            }),
        };
        Ok(response)
    })
}
