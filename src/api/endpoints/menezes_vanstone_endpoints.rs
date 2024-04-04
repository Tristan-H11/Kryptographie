use crate::api::serializable_models::UseFastQuery;

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

#[derive(Serialize)]
pub struct MvKeyPair {
    pub coef_a: u32,
    pub coef_b: u32,
    pub prime: String,
    pub generator_x: String,
    pub generator_y: String,
    pub public_key_x: String,
    pub public_key_y: String,
    pub private_key: String,
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

    let key_pair = MvKeyPair {
        coef_a: 2,
        coef_b: 3,
        prime: "17".to_string(),
        generator_x: "2".to_string(),
        generator_y: "3".to_string(),
        public_key_x: "6".to_string(),
        public_key_y: "4".to_string(),
        private_key: "1".to_string(),
    };

    HttpResponse::Ok().json(key_pair)
}
