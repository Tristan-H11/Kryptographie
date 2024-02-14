use std::str::FromStr;

use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use serde::{Deserialize, Serialize};

use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::UseFastQuery;
use crate::math_core::number_theory::extended_euclid_result::ExtendedEuclidResult;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};

#[derive(Deserialize)]
pub struct ExtendedEuclidRequest {
    pub a: String,
    pub b: String,
}

#[derive(Serialize)]
pub struct ExtendedEuclidResponse {
    pub x: String,
    pub y: String,
    pub ggt: String,
}

impl ExtendedEuclidResponse {
    /// Erstellt eine neue Instanz der ExtendedEuclidResponse anhand eines ExtendedEuclidResult.
    fn from(result: ExtendedEuclidResult) -> ExtendedEuclidResponse {
        ExtendedEuclidResponse {
            x: result.x.to_string(),
            y: result.y.to_string(),
            ggt: result.ggt.to_string(),
        }
    }
}

/// Berechnet den erweiterten Euklidischen Algorithmus.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Berechnung des erweiterten Euklidischen Algorithmus enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die die Ergebnisse des erweiterten Euklidischen Algorithmus enthält.
pub(crate) async fn euclid_endpoint(
    req_body: Json<ExtendedEuclidRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /math/extended_euclid wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: ExtendedEuclidRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    call_checked_with_parsed_big_ints(|| {
        let a = &BigInt::from_str(&*req_body.a)?;
        let b = &BigInt::from_str(&*req_body.b)?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let extended_euclid_result = number_theory_service.extended_euclid(a, b);

        let response = ExtendedEuclidResponse::from(extended_euclid_result);
        Ok(HttpResponse::Ok().json(response))
    })
}
