use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;

use crate::api::serializable_models::{EncryptDecryptRequest, SingleStringResponse, UseFastQuery};
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

///
/// Verschlüsselt eine Nachricht.
///
pub(crate) async fn encrypt(
    req_body: Json<EncryptDecryptRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/encrypt wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: EncryptDecryptRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.message;
    let number_system_base = req_body.number_system_base;

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.key_pair.to_public_key()?;

        let number_theory_service = match use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let rsa_service =
            crate::encryption::string_schemes::rsa::rsa_with_string_service::RsaWithStringService::new(
                number_theory_service,
            );

        let ciphertext = rsa_service.encrypt(&plaintext, number_system_base, &public_key);
        let response = SingleStringResponse {
            message: ciphertext,
        };

        Ok(HttpResponse::Ok().json(response))
    })
}
