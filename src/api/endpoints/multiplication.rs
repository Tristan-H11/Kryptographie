use actix_web::http::StatusCode;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, HttpResponseBuilder, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use std::str::FromStr;

use crate::api::serializable_models::{
    MultiplicationRequest, MultiplicationResponse, SingleStringResponse, UseFastQuery,
};
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

/// Multipliziert zwei Zahlen miteinander.
pub(crate) async fn multiplication(
    req_body: Json<MultiplicationRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/multiplication wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: MultiplicationRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let factor_one = BigInt::from_str(&req_body.factor_one).unwrap();
    let factor_two = BigInt::from_str(&req_body.factor_two).unwrap();

    let public_key = req_body.key_pair.to_public_key();
    let private_key = req_body.key_pair.to_private_key();

    let encrypted_factor_one = public_key.encrypt_number(&factor_one, use_fast);
    let encrypted_factor_two = public_key.encrypt_number(&factor_two, use_fast);

    let encrypted_result = &encrypted_factor_one * &encrypted_factor_two;

    let result = private_key.decrypt_number(&encrypted_result, use_fast);

    let response = MultiplicationResponse {
        encrypted_factor_one: encrypted_factor_one.to_str_radix(10),
        encrypted_factor_two: encrypted_factor_two.to_str_radix(10),
        encrypted_result: encrypted_result.to_str_radix(10),
        decrypted_result: result.to_str_radix(10),
    };

    // TODO: Will man das wirklich haben oder ist die Interpreation dem nutzer überlassen? Sonst halt
    // nen Feld in der Resposne für "could be wrong" oder so einfügen.
    if (factor_one * factor_two) != result {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(
            SingleStringResponse {
                message: "Multiplikation fehlgeschlagen: Produkt größer als Modulus!".to_string(),
            },
        );
    }

    HttpResponse::Ok().json(response)
}
