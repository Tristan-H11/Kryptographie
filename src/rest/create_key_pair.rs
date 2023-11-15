use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use log::info;

use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;
use crate::rest::serializable_models::{CreateKeyPairRequest, KeyPair};

///
/// Erstellt ein neues Schlüsselpaar.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Erstellung des Schlüsselpaares enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Schlüsselpaar enthält.
pub(crate) async fn create_key_pair(req_body: Json<CreateKeyPairRequest>) -> impl Responder {
    info!("Erstelle neues Schlüsselpaar.");
    let req_body: CreateKeyPairRequest = req_body.into_inner();

    let key_gen_service = RsaKeygenService::new(req_body.modulus_width);
    let (public_key, private_key) = key_gen_service.generate_keypair(req_body.miller_rabin_rounds, req_body.random_seed, req_body.number_system_base);

    let key_pair_response = KeyPair {
        modulus: public_key.get_n_as_str(),
        e: public_key.get_e_as_str(),
        d: private_key.get_d_as_str(),
        block_size_pub: public_key.get_block_size_as_str(),
        block_size_priv: private_key.get_block_size_as_str(),
    };

    HttpResponse::Ok().json(key_pair_response)
}
