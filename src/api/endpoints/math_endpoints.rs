use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::math_core::babystep_giantstep::{Shanks, ShanksResult};
use crate::math_core::number_theory::extended_euclid_result::ExtendedEuclidResult;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ModulInverseRequest {
    pub n: String,
    pub modul: String,
}

#[derive(Deserialize)]
pub struct ShanksRequest {
    pub base: String,
    pub element: String,
    pub modul: String,
}

#[derive(Serialize)]
pub struct ShanksResponse {
    pub result: String,
    pub giantsteps: Vec<(String, String)>,
}

impl From<ShanksResult> for ShanksResponse {
    fn from(value: ShanksResult) -> Self {
        ShanksResponse {
            result: value.result.to_str_radix(10),
            giantsteps: value
                .map
                .iter()
                .map(|(s, g)| (s.to_str_radix(10), g.to_str_radix(10)))
                .collect(),
        }
    }
}

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

#[derive(Deserialize)]
pub struct ExponentiationRequest {
    pub exponent: String,
    pub base: String,
    pub modulus: String,
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

/// Berechnet das modulare Inverse.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die modulare Inverse enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Ergebnis der modularen Inverse enthält.
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

    call_checked_with_parsed_big_ints(|| {
        let n = BigInt::from_str(&req_body.n)?;
        let modul = BigInt::from_str(&req_body.modul)?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let result = number_theory_service.modulo_inverse(&n, &modul);

        let response = match result {
            Ok(x) => {
                let response = SingleStringResponse {
                    message: x.to_string(),
                };
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::BadRequest().json(SingleStringResponse {
                message: "Kein Ergebnis gefunden".to_string(),
            }),
        };

        Ok(response)
    })
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
            Ok(shanks_result) => {
                let shanks_response: ShanksResponse = shanks_result.into();
                HttpResponse::Ok().json(shanks_response)
            }
            Err(_) => HttpResponse::BadRequest().json(SingleStringResponse {
                message: "Fehler beim Berechnen des diskreten Logarithmus".to_string(),
            }),
        };
        Ok(response)
    })
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

    call_checked_with_parsed_big_ints(|| {
        let exponent = &BigInt::from_str(&*req_body.exponent)?;
        let base = &BigInt::from_str(&*req_body.base)?;
        let modulus = &BigInt::from_str(&*req_body.modulus)?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let result = number_theory_service
            .fast_exponentiation(base, exponent, modulus)
            .to_str_radix(10);

        let response = SingleStringResponse { message: result };

        Ok(HttpResponse::Ok().json(response))
    })
}
