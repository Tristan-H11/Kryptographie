use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use crate::encryption::rsa::keys::PrivateKey;
use crate::rest::serializable_models::{FromToSerializable, SignRequest, SingleStringResponse};

///
/// Signiert eine Nachricht.
///
pub(crate) async fn sign(req_body: Json<SignRequest>) -> impl Responder {
    let req_body: SignRequest = req_body.into_inner();
    let plaintext = req_body.plaintext;
    let private_key = PrivateKey::from_serializable(req_body.private_key);

    let signature = private_key.sign(&plaintext);
    let response = SingleStringResponse {
        message: signature
    };

    HttpResponse::Ok().json(response)
}