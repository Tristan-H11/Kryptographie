use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;

use crate::api::serializable_models::{EncryptDecryptRequest, SingleStringResponse, UseFastQuery};

///
/// Entschlüsselt eine Nachricht.
///
pub(crate) async fn decrypt(
    req_body: Json<EncryptDecryptRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/decrypt wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: EncryptDecryptRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let ciphertext = req_body.message;
    let private_key = req_body.key_pair.to_private_key();
    let number_system_base = req_body.number_system_base;

    let plaintext = private_key.decrypt(&ciphertext, number_system_base, use_fast);
    let response = SingleStringResponse { message: plaintext };

    HttpResponse::Ok().json(response)
}
