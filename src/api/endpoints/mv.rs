use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::cmp::max;

use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptor, Signer, Verifier,
};
use crate::encryption::core::menezes_vanstone::keys::{
    MenezesVanstoneKeyPair, MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
};
use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::{
    MenezesVanstoneScheme, MenezesVanstoneSignature,
};
use crate::encryption::string_schemes::decimal_unicode_schemes::from_decimal_block_scheme::FromDecimalBlockScheme;
use crate::encryption::string_schemes::decimal_unicode_schemes::keys::DecimalUnicodeConversionSchemeKey;
use crate::encryption::string_schemes::menezes_vanstone::keys::{
    MenezesVanstoneStringPrivateKey, MenezesVanstoneStringPublicKey,
};
use crate::encryption::string_schemes::menezes_vanstone::menezes_vanstone_string_scheme::{
    MenezesVanstoneStringScheme, MvStringCiphertext,
};
use crate::encryption::symmetric_encryption_types::SymmetricEncryptor;
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;
use crate::math_core::traits::logarithm::Logarithm;

#[derive(Deserialize, Clone)]
pub struct MvCreateKeyPairRequestBean {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub coef_a: i32,
    pub random_seed: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EllipticCurveBean {
    pub a: i64,
    pub prime: String,
    pub order_of_subgroup: String,
    pub generator: EcPointBean,
}

impl From<SecureFiniteFieldEllipticCurve> for EllipticCurveBean {
    fn from(curve: SecureFiniteFieldEllipticCurve) -> Self {
        EllipticCurveBean {
            a: curve.a,
            prime: curve.prime.to_string(),
            order_of_subgroup: curve.order_of_subgroup.to_string(),
            generator: EcPointBean::from(curve.generator),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EcPointBean {
    pub x: String,
    pub y: String,
    pub is_infinite: bool,
}

impl From<FiniteFieldEllipticCurvePoint> for EcPointBean {
    fn from(point: FiniteFieldEllipticCurvePoint) -> Self {
        EcPointBean {
            x: point.x.to_string(),
            y: point.y.to_string(),
            is_infinite: point.is_infinite,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MvPublicKeyBean {
    pub curve: EllipticCurveBean,
    pub y: EcPointBean,
}

impl From<MenezesVanstonePublicKey> for MvPublicKeyBean {
    fn from(key: MenezesVanstonePublicKey) -> Self {
        MvPublicKeyBean {
            curve: EllipticCurveBean::from(key.curve),
            y: EcPointBean::from(key.y),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MvPrivateKeyBean {
    pub curve: EllipticCurveBean,
    pub x: String,
}

impl From<MenezesVanstonePrivateKey> for MvPrivateKeyBean {
    fn from(key: MenezesVanstonePrivateKey) -> Self {
        MvPrivateKeyBean {
            curve: EllipticCurveBean::from(key.curve),
            x: key.x.to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct MvKeyPairBean {
    pub public_key: MvPublicKeyBean,
    pub private_key: MvPrivateKeyBean,
}

impl From<MenezesVanstoneKeyPair> for MvKeyPairBean {
    fn from(key_pair: MenezesVanstoneKeyPair) -> Self {
        MvKeyPairBean {
            public_key: MvPublicKeyBean::from(key_pair.public_key),
            private_key: MvPrivateKeyBean::from(key_pair.private_key),
        }
    }
}

#[derive(Deserialize)]
pub struct MvEncryptRequestBean {
    pub public_key: MvPublicKeyBean,
    pub message: String,
    pub radix: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MvCipherTextBean {
    pub encrypted_message: String,
    pub points: Vec<EcPointBean>,
}

impl From<MvStringCiphertext> for MvCipherTextBean {
    fn from(ciphertext: MvStringCiphertext) -> Self {
        let points = ciphertext.points.into_iter().map(Into::into).collect();

        MvCipherTextBean {
            encrypted_message: ciphertext.ciphertext,
            points,
        }
    }
}

#[derive(Deserialize)]
pub struct MvDecryptRequestBean {
    pub private_key: MvPrivateKeyBean,
    pub cipher_text: MvCipherTextBean,
    pub radix: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MvSignatureBean {
    pub r: String,
    pub s: String,
    pub string_representation: String,
}

impl From<MenezesVanstoneSignature> for MvSignatureBean {
    fn from(signature: MenezesVanstoneSignature) -> Self {
        //TODO Sauber ausarbeiten!
        let blocks = vec![signature.r.clone(), signature.s.clone()];

        let radix = 55296;

        // Die größere der beiden Blockgrößen, damit sicher beide Werte enthalten sein werden.
        let block_size = max(
            signature.r.log(&radix.into()) + 1,
            signature.s.log(&radix.into()) + 1,
        );
        let key = DecimalUnicodeConversionSchemeKey { block_size, radix };
        let string_representation = FromDecimalBlockScheme::encrypt(&blocks, &key);

        MvSignatureBean {
            r: signature.r.to_string(),
            s: signature.s.to_string(),
            string_representation,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct MvSignRequestBean {
    pub private_key: MvPrivateKeyBean,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct MvVerifyRequestBean {
    pub public_key: MvPublicKeyBean,
    pub message: String,
    pub signature: MvSignatureBean,
}

/// Erstellt ein neues Schlüsselpaar für das MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Erstellung des Schlüsselpaares enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Schlüsselpaar enthält.
pub(crate) async fn create_key_pair(
    req_body: Json<MvCreateKeyPairRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/createKeyPair wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: MvCreateKeyPairRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;

    let _number_theory_service = match use_fast {
        true => NumberTheoryService::new(Fast),
        false => NumberTheoryService::new(Slow),
    };

    let key_pair = MenezesVanstoneScheme::generate_keypair(
        req_body.coef_a,
        req_body.modulus_width,
        req_body.miller_rabin_rounds,
        req_body.random_seed,
    );

    match key_pair {
        Ok(key_pair) => {
            let response = MvKeyPairBean::from(key_pair);
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Verschlüsselt eine Nachricht mit dem MenezesVanstone-Schema.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Nachricht und den öffentlichen Schlüssel enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die die verschlüsselte Nachricht enthält.
pub(crate) async fn encrypt(
    req_body: Json<MvEncryptRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/encrypt wurde aufgerufen");
    let req_body: MvEncryptRequestBean = req_body.into_inner();

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.public_key.clone().into();

        let public_key = MenezesVanstoneStringPublicKey {
            mv_key: public_key,
            radix: req_body.radix,
        };

        let message = &req_body.message;

        let random_seed = 14; // TODO: Random_seed erwarten
        let service = match query.use_fast {
            true => NumberTheoryWithPrngService::new(Fast, random_seed),
            false => NumberTheoryWithPrngService::new(Slow, random_seed),
        };

        let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &message, &service);

        match ciphertext {
            Ok(ciphertext) => {
                let response = MvCipherTextBean::from(ciphertext);
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        }
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
    req_body: Json<MvDecryptRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/decrypt wurde aufgerufen");
    let req_body: MvDecryptRequestBean = req_body.into_inner();

    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.private_key.clone().into();

        let private_key = MenezesVanstoneStringPrivateKey {
            mv_key: private_key,
            radix: req_body.radix,
        };

        let ciphertext = req_body.cipher_text.clone().into();

        let random_seed = 14; // TODO: Random_seed erwarten
        let service = match query.use_fast {
            true => NumberTheoryWithPrngService::new(Fast, random_seed),
            false => NumberTheoryWithPrngService::new(Slow, random_seed),
        };

        let plaintext = MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, &service);

        match plaintext {
            Ok(plaintext) => {
                let response = SingleStringResponse { message: plaintext };
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        }
    })
}

pub(crate) async fn sign(
    req_body: Json<MvSignRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/sign wurde aufgerufen");

    let req_body: &MvSignRequestBean = &req_body.into_inner();
    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.private_key.clone().into();
        let message = &req_body.message;

        let random_seed = 14; // TODO: Random_seed erwarten
        let service = match query.use_fast {
            true => NumberTheoryWithPrngService::new(Fast, random_seed),
            false => NumberTheoryWithPrngService::new(Slow, random_seed),
        };

        let signature = MenezesVanstoneScheme::sign(&private_key, message, &service);

        match signature {
            Ok(signature) => {
                let response = MvSignatureBean::from(signature);
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        }
    })
}

pub(crate) async fn verify(
    req_body: Json<MvVerifyRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!("Endpunkt /menezesVanstone/verify wurde aufgerufen");

    let req_body: &MvVerifyRequestBean = &req_body.into_inner();
    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.public_key.clone().into();
        let message = &req_body.message;
        let signature = &req_body.signature.clone().into();

        let random_seed = 14; // TODO: Random_seed erwarten
        let service = match query.use_fast {
            true => NumberTheoryWithPrngService::new(Fast, random_seed),
            false => NumberTheoryWithPrngService::new(Slow, random_seed),
        };

        let verified = MenezesVanstoneScheme::verify(&public_key, signature, message, &service);

        match verified {
            Ok(verified) => {
                let response = SingleStringResponse {
                    message: verified.to_string(),
                };
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
        }
    })
}
