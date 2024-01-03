use std::str::FromStr;

use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

#[derive(Deserialize)]
pub struct ExponentiationRequest {
    pub exponent: String,
    pub base: String,
    pub modulus: String,
}

/// Berechnet die Exponentiation.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Exponentiation enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Ergebnis der Exponentiation enthält.
pub(crate) async fn exponentiation(
    req_body: Json<ExponentiationRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /math/exponentiation wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: ExponentiationRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let exponent = &BigInt::from_str(&*req_body.exponent);
    let base = &BigInt::from_str(&*req_body.base);
    let modulus = &BigInt::from_str(&*req_body.modulus);

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    match (exponent, base, modulus) {
        (Ok(exponent), Ok(base), Ok(modulus)) => {
            let result = number_theory_service
                .fast_exponentiation(base, exponent, modulus)
                .to_str_radix(10);

            let response = SingleStringResponse { message: result };

            HttpResponse::Ok().json(response)
        }
        _ => {
            return HttpResponse::BadRequest().json(SingleStringResponse {
                message: "Fehler beim Parsen der Parameter".to_string(),
            })
        }
    }
}
