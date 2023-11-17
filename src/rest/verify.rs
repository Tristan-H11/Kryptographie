use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use log::info;

use crate::rest::serializable_models::{SingleStringResponse, VerifyRequest};

///
/// Verifiziert eine Signatur zu einer Nachricht.
///
pub(crate) async fn verify(req_body: Json<VerifyRequest>) -> impl Responder {
    info!("Endpunkt /rsa/verify wurde aufgerufen");
    let req_body: VerifyRequest = req_body.into_inner();
    let plaintext = req_body.plaintext;
    let signature = req_body.signature;
    let public_key = req_body.key_pair.to_public_key();

    let plaintext = public_key.verify(&signature, &plaintext);
    let response = SingleStringResponse {
        message: plaintext.to_string()
    };

    HttpResponse::Ok().json(response)
}