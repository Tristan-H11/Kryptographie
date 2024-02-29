use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::api::serializable_models::{KeyPair, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::{AsymmetricKeyPair, KeyGenerator};
use crate::encryption::rsa::rsa_scheme::{RsaKeyGenConfig, RsaScheme};
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use crate::math_core::traits::logarithm::Logarithm;

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

    let config = RsaKeyGenConfig {
        key_size: req_body.modulus_width,
        miller_rabin_iterations: req_body.miller_rabin_rounds,
        random_seed: req_body.random_seed,
        number_theory_service,
    };

    let key_pair = RsaScheme::generate_keypair(&config);

    let public_key = key_pair.public();
    let private_key = key_pair.private();

    let block_size_pub = public_key.n.log(&req_body.number_system_base.into());
    let block_size_priv = private_key.n.log(&req_body.number_system_base.into()) + 1;

    let key_pair_response = KeyPair {
        modulus: public_key.n.to_str_radix(10),
        e: public_key.e.to_str_radix(10),
        d: private_key.d.to_str_radix(10),
        block_size_pub: block_size_pub.to_string(),
        block_size_priv: block_size_priv.to_string(),
    };

    HttpResponse::Ok().json(key_pair_response)
}
