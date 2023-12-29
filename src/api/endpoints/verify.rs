use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Query};
use log::info;

use crate::api::serializable_models::{SingleStringResponse, UseFastQuery, VerifyRequest};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryService;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

///
/// Verifiziert eine Signatur zu einer Nachricht.
///
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
    let public_key = req_body.key_pair.to_public_key();

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    let rsa_service = crate::encryption::rsa::rsa_service::RsaService::new(number_theory_service);

    let plaintext = rsa_service.verify(&signature, &plaintext, public_key);
    let response = SingleStringResponse {
        message: plaintext.to_string(),
    };

    HttpResponse::Ok().json(response)
}
