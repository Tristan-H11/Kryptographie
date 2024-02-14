use crate::api::basic::call_checked_with_parsed_big_ints;
use actix_web::http::StatusCode;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, HttpResponseBuilder, Responder};
use bigdecimal::num_bigint::BigInt;
use log::info;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::api::serializable_models::{KeyPair, SingleStringResponse, UseFastQuery};
use crate::encryption::rsa::rsa_service::RsaService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

#[derive(Deserialize)]
pub struct MultiplicationRequest {
    pub factor_one: String,
    pub factor_two: String,
    pub key_pair: KeyPair,
}

#[derive(Serialize)]
pub struct MultiplicationResponse {
    pub encrypted_factor_one: String,
    pub encrypted_factor_two: String,
    pub encrypted_result: String,
    pub decrypted_result: String,
}

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

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    let rsa_service = RsaService::new(number_theory_service);

    call_checked_with_parsed_big_ints(|| {
        let factor_one = BigInt::from_str(&req_body.factor_one)?;
        let factor_two = BigInt::from_str(&req_body.factor_two)?;

        let public_key = req_body.key_pair.to_public_key()?;
        let private_key = req_body.key_pair.to_private_key()?;

        let encrypted_factor_one = rsa_service.encrypt_decrypt_number(&factor_one, &public_key);
        let encrypted_factor_two = rsa_service.encrypt_decrypt_number(&factor_two, &public_key);

        let encrypted_result = &encrypted_factor_one * &encrypted_factor_two;

        let result = rsa_service.encrypt_decrypt_number(&encrypted_result, &private_key);

        let response = MultiplicationResponse {
            encrypted_factor_one: encrypted_factor_one.to_str_radix(10),
            encrypted_factor_two: encrypted_factor_two.to_str_radix(10),
            encrypted_result: encrypted_result.to_str_radix(10),
            decrypted_result: result.to_str_radix(10),
        };

        if (factor_one * factor_two) != result {
            return Ok(
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(
                    SingleStringResponse {
                        message: "Multiplikation fehlgeschlagen: Produkt größer als Modulus!"
                            .to_string(),
                    },
                ),
            );
        }

        Ok(HttpResponse::Ok().json(response))
    })
}
