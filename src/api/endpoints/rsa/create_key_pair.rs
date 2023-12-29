use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Query};
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, UseFastQuery};
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
    let (public_key, private_key) = key_gen_service.generate_keypair(
        req_body.miller_rabin_rounds,
        req_body.random_seed,
        req_body.number_system_base,
    );

    let key_pair_response = KeyPair {
        modulus: public_key.n.to_str_radix(10),
        e: public_key.e.to_str_radix(10),
        d: private_key.d.to_str_radix(10),
        block_size_pub: public_key.block_size.to_string(),
        block_size_priv: private_key.block_size.to_string(),
    };

    HttpResponse::Ok().json(key_pair_response)
}
