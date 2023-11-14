use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use log::info;
use serde::{Deserialize, Serialize};
use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

trait ToSerializable {
    type T;
    ///
    /// Erstellt aus dem Datenmodell ein Modell für die Serialisierung.
    ///
    fn to_serializable(&self) -> Self::T;
}

impl ToSerializable for crate::encryption::rsa::keys::PublicKey {
    type T = PublicKey;

    fn to_serializable(&self) -> Self::T {
        PublicKey {
            modulus: self.get_n_as_str(),
            e: self.get_e_as_str(),
            block_size: self.get_block_size_as_str()
        }
    }
}

impl ToSerializable for crate::encryption::rsa::keys::PrivateKey {
    type T = PrivateKey;

    fn to_serializable(&self) -> Self::T {
        PrivateKey {
            modulus: self.get_n_as_str(),
            d: self.get_d_as_str(),
            block_size: self.get_block_size_as_str()
        }
    }
}

#[derive(Serialize)]
pub struct PublicKey {
    pub modulus: String,
    pub e: String,
    pub block_size: String
}

#[derive(Serialize)]
pub struct PrivateKey {
    pub modulus: String,
    pub d: String,
    pub block_size: String
}


#[derive(Serialize)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey
}

#[derive(Deserialize)]
pub struct CreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub random_seed: u32,
    pub number_system_base: u32
}

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
        public_key: public_key.to_serializable(),
        private_key: private_key.to_serializable()
    };

    HttpResponse::Ok().json(key_pair_response)
}
