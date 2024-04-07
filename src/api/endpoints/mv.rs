use std::thread::current;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::{AsymmetricDecryptor, AsymmetricEncryptor};
use crate::encryption::core::menezes_vanstone::keys::{MenezesVanstoneKeyPair, MenezesVanstonePrivateKey, MenezesVanstonePublicKey};
use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::{MenezesVanstoneCiphertext, MenezesVanstoneScheme};
use crate::encryption::string_schemes::menezes_vanstone::keys::{
    MenezesVanstoneStringPrivateKey, MenezesVanstoneStringPublicKey,
};
use crate::encryption::string_schemes::menezes_vanstone::menezes_vanstone_string_scheme::{
    MenezesVanstoneStringScheme, MvStringCiphertext,
};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};

#[derive(Deserialize)]
pub struct MvCreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub coef_a: u32,
    pub random_seed: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EllipticCurve {
    pub a: i32,
    pub prime: String,
    pub order_of_subgroup: String,
    pub generator: EcPoint,
}

impl From<SecureFiniteFieldEllipticCurve> for EllipticCurve {
    fn from(curve: SecureFiniteFieldEllipticCurve) -> Self {
        EllipticCurve {
            a: curve.a,
            prime: curve.prime.to_string(),
            order_of_subgroup: curve.order_of_subgroup.to_string(),
            generator: EcPoint::from(curve.generator),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct EcPoint {
    pub x: String,
    pub y: String,
    pub is_infinite: bool,
}

impl From<FiniteFieldEllipticCurvePoint> for EcPoint {
    fn from(point: FiniteFieldEllipticCurvePoint) -> Self {
        EcPoint {
            x: point.x.to_string(),
            y: point.y.to_string(),
            is_infinite: point.is_infinite,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MvPublicKey {
    pub curve: EllipticCurve,
    pub y: EcPoint,
}

impl From<MenezesVanstonePublicKey> for MvPublicKey {
    fn from(key: MenezesVanstonePublicKey) -> Self {
        MvPublicKey {
            curve: EllipticCurve::from(key.curve),
            y: EcPoint::from(key.y),
        }
    }
}


#[derive(Serialize, Deserialize, Default)]
pub struct MvPrivateKey {
    pub curve: EllipticCurve,
    pub x: String,
}

impl From<MenezesVanstonePrivateKey> for MvPrivateKey {
    fn from(key: MenezesVanstonePrivateKey) -> Self {
        MvPrivateKey {
            curve: EllipticCurve::from(key.curve),
            x: key.x.to_string(),
        }
    }
}

#[derive(Serialize, Default)]
pub struct MvKeyPair {
    pub public_key: MvPublicKey,
    pub private_key: MvPrivateKey,
}

impl From<MenezesVanstoneKeyPair> for MvKeyPair {
    fn from(key_pair: MenezesVanstoneKeyPair) -> Self {
        MvKeyPair {
            public_key: MvPublicKey::from(key_pair.public_key),
            private_key: MvPrivateKey::from(key_pair.private_key),
        }
    }

}

#[derive(Deserialize)]
pub struct MvEncryptRequest {
    pub public_key: MvPublicKey,
    pub message: String,
    pub radix: u32,
}

#[derive(Deserialize, Serialize)]
pub struct MvCipherText {
    pub encrypted_message: String,
    pub points: Vec<EcPoint>,
}

#[derive(Deserialize)]
pub struct MvDecryptRequest {
    pub private_key: MvPrivateKey,
    pub cipher_text: MvCipherText,
    pub radix: u32,
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
pub(crate) async fn encrypt(
    req_body: Json<MvEncryptRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/encrypt wurde aufgerufen");
    let req_body: MvEncryptRequest = req_body.into_inner();

    call_checked_with_parsed_big_ints(|| {
        let generator = FiniteFieldEllipticCurvePoint {
            x: req_body.public_key.generator.x.parse().unwrap(),
            y: req_body.public_key.generator.y.parse().unwrap(),
            is_infinite: req_body.public_key.generator.is_infinite,
        };

        let curve = SecureFiniteFieldEllipticCurve {
            a: req_body.public_key.curve.a,
            prime: req_body.public_key.curve.prime.parse().unwrap(),
            order_of_subgroup: req_body.public_key.curve.order_of_subgroup.parse().unwrap(),
            generator,
        };

        let y = FiniteFieldEllipticCurvePoint {
            x: req_body.public_key.y.x.parse().unwrap(),
            y: req_body.public_key.y.y.parse().unwrap(),
            is_infinite: req_body.public_key.y.is_infinite,
        };

        let public_key = MenezesVanstonePublicKey {
            curve,
            y,
        };

        let public_key = MenezesVanstoneStringPublicKey {
            mv_key: public_key,
            radix: req_body.radix,
        };

        let message = &req_body.message;

        let service = match query.use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &message, service);

        let points = ciphertext
            .points
            .iter()
            .map(|point| EcPoint {
                x: point.x.to_string(),
                y: point.y.to_string(),
                is_infinite: point.is_infinite,
            })
            .collect();

        let response = MvCipherText {
            encrypted_message: ciphertext.ciphertext,
            points,
        };

        Ok(HttpResponse::Ok().json(response))
    })
}

/// Entschlüsselt eine Nachricht mit dem MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die verschlüsselte Nachricht enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die die entschlüsselte Nachricht enthält.
pub(crate) async fn decrypt(
    req_body: Json<MvDecryptRequest>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/decrypt wurde aufgerufen");
    let req_body: MvDecryptRequest = req_body.into_inner();

    call_checked_with_parsed_big_ints(|| {

        let generator = FiniteFieldEllipticCurvePoint {
            x: req_body.private_key.curve.generator.x.parse().unwrap(),
            y: req_body.private_key.curve.generator.y.parse().unwrap(),
            is_infinite: req_body.private_key.curve.generator.is_infinite,
        };
        let curve = SecureFiniteFieldEllipticCurve {
            a: req_body.private_key.curve.a,
            prime: req_body.private_key.curve.prime.parse().unwrap(),
            order_of_subgroup: req_body.private_key.curve.order_of_subgroup.parse().unwrap(),
            generator,
        };

        let private_key = MenezesVanstonePrivateKey {
            curve,
            x: req_body.private_key.x.parse().unwrap(),
        };

        let private_key = MenezesVanstoneStringPrivateKey {
            mv_key: private_key,
            radix: req_body.radix,
        };

        let ciphertext = MvStringCiphertext {
            ciphertext: req_body.cipher_text.encrypted_message.clone(),
            points: req_body
                .cipher_text
                .points
                .iter()
                .map(|point| FiniteFieldEllipticCurvePoint {
                    x: point.x.parse().unwrap(),
                    y: point.y.parse().unwrap(),
                    is_infinite: false,
                })
                .collect(),
        };

        let service = match query.use_fast {
            true => NumberTheoryService::new(Fast),
            false => NumberTheoryService::new(Slow),
        };

        let plaintext = MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, service);

        let response = SingleStringResponse { message: plaintext };

        Ok(HttpResponse::Ok().json(response))
    })
}
