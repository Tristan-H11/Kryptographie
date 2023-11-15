use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;

use crate::rest::serializable_models::{EncryptDecryptRequest, SingleStringResponse};

///
/// Verschlüsselt eine Nachricht.
///
pub(crate) async fn encrypt(req_body: Json<EncryptDecryptRequest>) -> impl Responder {
    let req_body: EncryptDecryptRequest = req_body.into_inner();
    let plaintext = req_body.message;
    let public_key = req_body.key_pair.to_public_key();
    let number_system_base = req_body.number_system_base;

    let ciphertext = public_key.encrypt(&plaintext, number_system_base);
    let response = SingleStringResponse {
        message: ciphertext
    };

    HttpResponse::Ok().json(response)
}
