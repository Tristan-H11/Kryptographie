use crate::api::basic::call_checked_with_parsed_big_ints;
use crate::api::serializable_models::{SingleStringResponse, UseFastQuery};
use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptor, AsymmetricKeyPair, KeyGenerator, Signer, Verifier,
};
use crate::encryption::core::rsa::keys::{RsaPrivateKey, RsaPublicKey};
use crate::encryption::core::rsa::rsa_scheme::{RsaKeyGenConfig, RsaScheme};
use crate::encryption::string_schemes::rsa::keys::{
    RsaWithStringPrivateKey, RsaWithStringPublicKey,
};
use crate::encryption::string_schemes::rsa::rsa_with_string_scheme::RsaWithStringScheme;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
    Fast, Slow,
};
use crate::math_core::traits::logarithm::Logarithm;
use actix_web::http::StatusCode;
use actix_web::web::{Json, Query};
use actix_web::{HttpResponse, HttpResponseBuilder, Responder};
use bigdecimal::num_bigint::{BigInt, ParseBigIntError};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;

#[derive(Deserialize)]
pub struct RsaCreateKeyPairRequestBean {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub random_seed: u32,
    pub number_system_base: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RsaKeyPairBean {
    pub modulus: String,
    pub e: String,
    pub d: String,
    pub block_size_pub: String,
    pub block_size_priv: String,
}

impl RsaKeyPairBean {
    /// Wandelt das serialisierte Schlüsselpaar in einen privaten Schlüssel um.
    ///
    /// # Rückgabe
    /// * `RsaKey` - Der private Schlüssel.
    ///
    /// # Fehler
    /// * `ParseBigIntError` - Falls die BigInts nicht geparst werden können.
    fn to_private_key(&self) -> Result<RsaPrivateKey, ParseBigIntError> {
        debug!("Serialisiere KeyPair zu PrivateKey");
        Ok(RsaPrivateKey {
            d: self.d.parse()?,
            n: self.modulus.parse()?,
        })
    }

    /// Wandelt das serialisierte Schlüsselpaar in einen öffentlichen Schlüssel um.
    ///
    /// # Rückgabe
    /// * `RsaKey` - Der öffentliche Schlüssel.
    ///
    /// # Fehler
    /// * `ParseBigIntError` - Falls die BigInts nicht geparst werden können.
    fn to_public_key(&self) -> Result<RsaPublicKey, ParseBigIntError> {
        debug!("Serialisiere KeyPair zu PublicKey");
        Ok(RsaPublicKey {
            e: self.e.parse()?,
            n: self.modulus.parse()?,
        })
    }
}

#[derive(Deserialize)]
pub struct RsaEncryptDecryptRequestBean {
    pub message: String,
    pub key_pair: RsaKeyPairBean,
    pub number_system_base: u32,
}

#[derive(Deserialize)]
pub struct RsaSignRequestBean {
    pub plaintext: String,
    pub key_pair: RsaKeyPairBean,
    pub radix: u32,
}

#[derive(Deserialize)]
pub struct RsaVerifyRequestBean {
    pub plaintext: String,
    pub signature: String,
    pub key_pair: RsaKeyPairBean,
    pub radix: u32,
}

#[derive(Deserialize)]
pub struct RsaMultiplicationRequestBean {
    pub factor_one: String,
    pub factor_two: String,
    pub key_pair: RsaKeyPairBean,
}

#[derive(Serialize)]
pub struct RsaMultiplicationResponseBean {
    pub encrypted_factor_one: String,
    pub encrypted_factor_two: String,
    pub encrypted_result: String,
    pub decrypted_result: String,
}

/// Erstellt ein neues Schlüsselpaar.
///
/// # Arguments
/// * `req_body` - Die Anfrage, die die Parameter für die Erstellung des Schlüsselpaares enthält.
///
/// # Returns
/// * `HttpResponse` - Die Antwort, die das Schlüsselpaar enthält.
pub(crate) async fn create_key_pair(
    req_body: Json<RsaCreateKeyPairRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/createKeyPair wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaCreateKeyPairRequestBean = req_body.into_inner();
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

    let key_pair_response = RsaKeyPairBean {
        modulus: public_key.n.to_str_radix(10),
        e: public_key.e.to_str_radix(10),
        d: private_key.d.to_str_radix(10),
        block_size_pub: block_size_pub.to_string(),
        block_size_priv: block_size_priv.to_string(),
    };

    HttpResponse::Ok().json(key_pair_response)
}

///
/// Verschlüsselt eine Nachricht.
///
pub(crate) async fn encrypt(
    req_body: Json<RsaEncryptDecryptRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/encrypt wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaEncryptDecryptRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.message;
    let number_system_base = req_body.number_system_base;

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.key_pair.to_public_key()?;

        // Hier wird ein Dummy-Seed verwendet, weil er für RSA nicht notwendig ist.
        let number_theory_service = match use_fast {
            true => NumberTheoryWithPrngService::new(Fast, 13),
            false => NumberTheoryWithPrngService::new(Slow, 13),
        };

        let rsa_with_string_key = RsaWithStringPublicKey {
            rsa_public_key: public_key,
            radix: number_system_base,
        };

        let ciphertext =
            RsaWithStringScheme::encrypt(&rsa_with_string_key, &plaintext, &number_theory_service);
        let response = SingleStringResponse {
            message: ciphertext,
        };

        Ok(HttpResponse::Ok().json(response))
    })
}

/// Endpunkt zum Entschlüsseln einer Nachricht mit RSA.
/// # Argumente
/// * `req_body` - Die Anfrage, die den verschlüsselten Text und den privaten Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die den entschlüsselten Text enthält.
pub(crate) async fn decrypt(
    req_body: Json<RsaEncryptDecryptRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/decrypt wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaEncryptDecryptRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;
    let ciphertext = req_body.message;
    let number_system_base = req_body.number_system_base;

    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.key_pair.to_private_key()?;

        // Hier wird ein Dummy-Seed verwendet, weil er für RSA nicht notwendig ist.
        let number_theory_service = match use_fast {
            true => NumberTheoryWithPrngService::new(Fast, 13),
            false => NumberTheoryWithPrngService::new(Slow, 13),
        };

        let rsa_with_string_key = RsaWithStringPrivateKey {
            rsa_private_key: private_key,
            radix: number_system_base,
        };

        let plaintext =
            RsaWithStringScheme::decrypt(&rsa_with_string_key, &ciphertext, &number_theory_service);
        let response = SingleStringResponse { message: plaintext };

        Ok(HttpResponse::Ok().json(response))
    })
}

/// Endpunkt zum Signieren einer Nachricht mit RSA.
///
/// # Argumente
/// * `req_body` - Die Anfrage, die den zu signierenden Text und den privaten Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die die Signatur enthält.
pub(crate) async fn sign(
    req_body: Json<RsaSignRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/sign wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaSignRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.plaintext;
    let radix = req_body.radix;

    call_checked_with_parsed_big_ints(|| {
        let private_key = req_body.key_pair.to_private_key()?;

        // Hier wird ein Dummy-Seed verwendet, weil er für RSA nicht notwendig ist.
        let number_theory_service = match use_fast {
            true => NumberTheoryWithPrngService::new(Fast, 13),
            false => NumberTheoryWithPrngService::new(Slow, 13),
        };

        let rsa_with_string_key = RsaWithStringPrivateKey {
            rsa_private_key: private_key,
            radix,
        };

        let signature =
            RsaWithStringScheme::sign(&rsa_with_string_key, &plaintext, &number_theory_service);
        let response = SingleStringResponse { message: signature };

        Ok(HttpResponse::Ok().json(response))
    })
}

/// Endpunkt zum Verifizieren einer Nachricht mit RSA.
///
/// # Argumente
/// * `req_body` - Die Anfrage, die den zu verifizierenden Text, die Signatur und den öffentlichen Schlüssel enthält.
/// * `query` - Die Abfrage, ob der schnelle oder der langsame Algorithmus verwendet werden soll.
///
/// # Rückgabe
/// * `HttpResponse` - Die Antwort, die den verifizierten Text enthält.
pub(crate) async fn verify(
    req_body: Json<RsaVerifyRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/verify wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaVerifyRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;

    let plaintext = req_body.plaintext;
    let signature = req_body.signature;
    let radix = req_body.radix;

    call_checked_with_parsed_big_ints(|| {
        let public_key = req_body.key_pair.to_public_key()?;

        // Hier wird ein Dummy-Seed verwendet, weil er für RSA nicht notwendig ist.
        let number_theory_service = match use_fast {
            true => NumberTheoryWithPrngService::new(Fast, 13),
            false => NumberTheoryWithPrngService::new(Slow, 13),
        };

        let rsa_with_string_key = RsaWithStringPublicKey {
            rsa_public_key: public_key,
            radix,
        };

        let plaintext = RsaWithStringScheme::verify(
            &rsa_with_string_key,
            &signature,
            &plaintext,
            &number_theory_service,
        );
        let response = SingleStringResponse {
            message: plaintext.to_string(),
        };

        Ok(HttpResponse::Ok().json(response))
    })
}

/// Multipliziert zwei Zahlen miteinander.
pub(crate) async fn multiplication(
    req_body: Json<RsaMultiplicationRequestBean>,
    query: Query<UseFastQuery>,
) -> impl Responder {
    info!(
        "Endpunkt /rsa/multiplication wurde aufgerufen, use_fast: {}",
        query.use_fast
    );
    let req_body: RsaMultiplicationRequestBean = req_body.into_inner();
    let use_fast = query.use_fast;

    // Hier wird ein Dummy-Seed verwendet, weil er für RSA nicht notwendig ist.
    let number_theory_service = match use_fast {
        true => NumberTheoryWithPrngService::new(Fast, 13),
        false => NumberTheoryWithPrngService::new(Slow, 13),
    };

    call_checked_with_parsed_big_ints(|| {
        let factor_one = BigInt::from_str(&req_body.factor_one)?;
        let factor_two = BigInt::from_str(&req_body.factor_two)?;

        let public_key = req_body.key_pair.to_public_key()?;
        let private_key = req_body.key_pair.to_private_key()?;

        let encrypted_factor_one =
            RsaScheme::encrypt(&public_key, &factor_one, &number_theory_service);
        let encrypted_factor_two =
            RsaScheme::encrypt(&public_key, &factor_two, &number_theory_service);

        let encrypted_result = &encrypted_factor_one * &encrypted_factor_two;

        let result = RsaScheme::decrypt(&private_key, &encrypted_result, &number_theory_service);

        let response = RsaMultiplicationResponseBean {
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
