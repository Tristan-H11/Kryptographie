use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use log::info;

use crate::api::serializable_models::{SignRequest, SingleStringResponse};

///
/// Signiert eine Nachricht.
///
pub(crate) async fn sign(req_body: Json<SignRequest>) -> impl Responder {
    info!("Endpunkt /rsa/sign wurde aufgerufen");
    let req_body: SignRequest = req_body.into_inner();
    let plaintext = req_body.plaintext;
    let private_key = req_body.key_pair.to_private_key();

    let signature = private_key.sign(&plaintext);
    let response = SingleStringResponse {
        message: signature
    };

    HttpResponse::Ok().json(response)
}