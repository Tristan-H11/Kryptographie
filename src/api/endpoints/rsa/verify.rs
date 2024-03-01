use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, SingleStringResponse, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::Verifier;
use crate::encryption::string_schemes::rsa::keys::RsaWithStringPublicKey;
use crate::encryption::string_schemes::rsa::rsa_with_string_scheme::RsaWithStringScheme;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub plaintext: String,
    pub signature: String,
    pub key_pair: KeyPair,
    pub radix: u32,
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
    let radix = req_body.radix;

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.key_pair.to_public_key()?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let rsa_with_string_key = RsaWithStringPublicKey {
            rsa_public_key: public_key,
            radix,
        };

        let plaintext = RsaWithStringScheme::verify(
            &rsa_with_string_key,
            &plaintext,
            &signature,
            number_theory_service,
        );
        let response = SingleStringResponse {
            message: plaintext.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    })
}
