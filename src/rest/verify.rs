use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use crate::encryption::rsa::keys::PublicKey;
use crate::rest::serializable_models::{FromToSerializable, SingleStringResponse, VerifyRequest};

///
/// Verifiziert eine Signatur zu einer Nachricht.
///
pub(crate) async fn verify(req_body: Json<VerifyRequest>) -> impl Responder {
    let req_body: VerifyRequest = req_body.into_inner();
    let plaintext = req_body.plaintext;
    let signature = req_body.signature;
    let public_key = PublicKey::from_serializable(req_body.public_key);

    let plaintext = public_key.verify(&signature, &plaintext);
    let response = SingleStringResponse {
        message: plaintext.to_string()
    };

    HttpResponse::Ok().json(response)
}