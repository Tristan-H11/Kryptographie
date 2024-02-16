use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, SingleStringResponse, UseFastQuery};
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub plaintext: String,
    pub signature: String,
    pub key_pair: KeyPair,
    pub g_base: u32,
}

/// Endpunkt zum Verifizieren einer Nachricht mit RSA.
///
/// # Argumente
/// * `req_body` - Die Anfrage, die den zu verifizierenden Text, die Signatur und den öffentlichen Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die den verifizierten Text enthält.
pub(crate) async fn verify(
    req_body: Json<VerifyRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/verify wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: VerifyRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.plaintext;
    let signature = req_body.signature;
    let g_base = req_body.g_base;

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.key_pair.to_public_key()?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let rsa_service =
            crate::encryption::rsa::rsa_with_string_service::RsaWithStringService::new(number_theory_service);

        let plaintext = rsa_service.verify(&signature, &plaintext, &public_key, g_base);
        let response = SingleStringResponse {
            message: plaintext.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    })
}
