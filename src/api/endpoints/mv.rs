use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};

use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MvCreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub coef_a: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EllipticCurve {
    pub a: u32,
    pub b: u32,
    pub prime: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EcPoint {
    pub x: String,
    pub y: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MvPublicKey {
    pub curve: EllipticCurve,
    pub generator: EcPoint,
    pub y: EcPoint,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MvPrivateKey {
    pub curve: EllipticCurve,
    pub x: String,
}

#[derive(Serialize, Default)]
pub struct MvKeyPair {
    pub public_key: MvPublicKey,
    pub private_key: MvPrivateKey,
}

#[derive(Deserialize)]
pub struct MvEncryptRequest {
    pub public_key: MvPublicKey,
    pub message: String,
}

#[derive(Serialize)]
pub struct MvCipherText {
    pub encrypted_message: String,
    pub points: Vec<EcPoint>,
}

pub struct MvDecryptRequest {
    pub private_key: MvPrivateKey,
    pub cipher_text: MvCipherText,
}

/// Erstellt ein neues Schlüsselpaar für das MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Erstellung des Schlüsselpaares enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Schlüsselpaar enthält.
pub(crate) async fn create_key_pair(
    req_body: Json<MvCreateKeyPairRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/createKeyPair wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let _req_body: MvCreateKeyPairRequest = req_body.into_inner();
    let use_fast = query.use_fast;

    let _number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    // TODO

    let mv_key_pair = MvKeyPair::default();

    HttpResponse::Ok().json(mv_key_pair)
}

/// Verschlüsselt eine Nachricht mit dem MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Nachricht und den öffentlichen Schlüssel enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die die verschlüsselte Nachricht enthält.
pub(crate) async fn encrypt(req_body: Json<MvEncryptRequest>) -> impl Responder {
    info!("Endpunkt /menezesVanstone/encrypt wurde aufgerufen");
    let _req_body: MvEncryptRequest = req_body.into_inner();

    // TODO

    let encrypted_message = "TODO".to_string();

    let response = MvCipherText {
        encrypted_message,
        points: vec![EcPoint::default()],
    };

    HttpResponse::Ok().json(response)
}

/// Entschlüsselt eine Nachricht mit dem MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die verschlüsselte Nachricht enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die die entschlüsselte Nachricht enthält.
pub(crate) async fn decrypt(req_body: Json<MvEncryptRequest>) -> impl Responder {
    info!("Endpunkt /menezesVanstone/decrypt wurde aufgerufen");
    let _req_body: MvEncryptRequest = req_body.into_inner();

    // TODO

    let decrypted_message = "TODO".to_string();

    HttpResponse::Ok().json(SingleStringResponse {
        message: decrypted_message,
    })
}
