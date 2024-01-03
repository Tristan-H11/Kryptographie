use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, SingleStringResponse, UseFastQuery};
use crate::encryption::math_functions::block_chiffre::determine_block_size;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryService;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};
use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

#[derive(Deserialize)]
pub struct CreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub random_seed: u32,
    pub number_system_base: u32,
}

/// Erstellt ein neues Schlüsselpaar.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Erstellung des Schlüsselpaares enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Schlüsselpaar enthält.
pub(crate) async fn create_key_pair(
    req_body: Json<CreateKeyPairRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/createKeyPair wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: CreateKeyPairRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    let key_gen_service = RsaKeygenService::new(req_body.modulus_width, number_theory_service);
    let (public_key, private_key) = match key_gen_service
        .generate_keypair(req_body.miller_rabin_rounds, req_body.random_seed)
    {
        Ok(key_pair) => key_pair,
        Err(_) => {
            return HttpResponse::InternalServerError().json(SingleStringResponse {
                message: "Schlüsselerzeugung fehlgeschlagen.".to_string(),
            })
        }
    };

    let block_size_pub = determine_block_size(
        &public_key.modulus(),
        &req_body.number_system_base.into(),
        true,
    )
    .to_string();

    let block_size_priv = determine_block_size(
        &private_key.modulus(),
        &req_body.number_system_base.into(),
        false,
    )
    .to_string();

    let key_pair_response = KeyPair {
        modulus: public_key.modulus().to_str_radix(10),
        e: public_key.exponent().to_str_radix(10),
        d: private_key.exponent().to_str_radix(10),
        block_size_pub,
        block_size_priv,
    };

    HttpResponse::Ok().json(key_pair_response)
}
