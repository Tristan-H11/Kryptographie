use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Query};
use log::info;

use crate::api::serializable_models::{SignRequest, SingleStringResponse, UseFastQuery};

///
/// Signiert eine Nachricht.
///
pub(crate) async fn sign(req_body: Json<SignRequest>, query: Query<UseFastQuery>) -> impl Responder {
    info!("Endpunkt /rsa/sign wurde aufgerufen, use_fast: {}", query.use_fast);
    let req_body: SignRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.plaintext;
    let private_key = req_body.key_pair.to_private_key();

    let signature = private_key.sign(&plaintext, use_fast);
    let response = SingleStringResponse {
        message: signature
    };

    HttpResponse::Ok().json(response)
}