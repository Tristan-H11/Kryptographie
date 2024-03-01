use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use keys::RsaWithStringPrivateKey;
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, SingleStringResponse, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::Signer;
use crate::encryption::string_schemes::rsa::keys;
use crate::encryption::string_schemes::rsa::rsa_with_string_scheme::RsaWithStringScheme;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

#[derive(Deserialize)]
pub struct SignRequest {
    pub plaintext: String,
    pub key_pair: KeyPair,
    pub radix: u32,
}

/// Endpunkt zum Signieren einer Nachricht mit RSA.
///
/// # Argumente
/// * `req_body` - Die Anfrage, die den zu signierenden Text und den privaten Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die die Signatur enthält.
pub(crate) async fn sign(
    req_body: Json<SignRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/sign wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: SignRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.plaintext;
    let radix = req_body.radix;

    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.key_pair.to_private_key()?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let rsa_with_string_key = RsaWithStringPrivateKey {
            rsa_private_key: private_key,
            radix,
        };

        let signature =
            RsaWithStringScheme::sign(&rsa_with_string_key, &plaintext, number_theory_service);
        let response = SingleStringResponse { message: signature };

        Ok(HttpResponse::Ok().json(response))
    })
}
