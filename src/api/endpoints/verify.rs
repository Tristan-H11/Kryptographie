use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;

use crate::api::serializable_models::{SingleStringResponse, UseFastQuery, VerifyRequest};

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

    let plaintext = public_key.verify(&signature, &plaintext, use_fast);
    let response = SingleStringResponse {
        message: plaintext.to_string(),
    };

    HttpResponse::Ok().json(response)
}
