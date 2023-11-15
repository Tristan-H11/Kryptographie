use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;

use crate::rest::serializable_models::{EncryptDecryptRequest, SingleStringResponse};

///
/// Entschlüsselt eine Nachricht.
///
pub(crate) async fn decrypt(req_body: Json<EncryptDecryptRequest>) -> impl Responder {
    let req_body: EncryptDecryptRequest = req_body.into_inner();
    let ciphertext = req_body.message;
    let private_key = req_body.key_pair.to_private_key();
    let number_system_base = req_body.number_system_base;

    let plaintext = private_key.decrypt(&ciphertext, number_system_base);
    let response = SingleStringResponse {
        message: plaintext
    };

    HttpResponse::Ok().json(response)
}