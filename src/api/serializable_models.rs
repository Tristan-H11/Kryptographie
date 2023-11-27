use log::debug;
use serde::{Deserialize, Serialize};

use crate::encryption::rsa::keys::{PrivateKey, PublicKey};

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub modulus: String,
    pub e: String,
    pub d: String,
    pub block_size_pub: String,
    pub block_size_priv: String,
}

impl KeyPair {
    pub(crate) fn to_private_key(&self) -> PrivateKey {
        debug!("Serialisiere KeyPair zu PrivateKey");
        PrivateKey::new_with_blocksize(
            (&self).d.parse().unwrap(),
            (&self).modulus.parse().unwrap(),
            (&self.block_size_priv).parse().unwrap(),
        )
    }

    pub(crate) fn to_public_key(&self) -> PublicKey {
        debug!("Serialisiere KeyPair zu PublicKey");
        PublicKey::new_with_blocksize(
            (&self).e.parse().unwrap(),
            (&self).modulus.parse().unwrap(),
            (&self.block_size_pub).parse().unwrap(),
        )
    }
}

#[derive(Serialize)]
pub struct SingleStringResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct ExponentiationRequest {
    pub exponent: String,
    pub base: String,
    pub modulus: String,
}

#[derive(Deserialize)]
pub struct ExtendedEuclidRequest {
    pub a: String,
    pub b: String,
}

#[derive(Serialize)]
pub struct ExtendedEuclidResponse {
    pub x: String,
    pub y: String,
    pub ggt: String,
}

#[derive(Deserialize)]
pub struct CreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub random_seed: u32,
    pub number_system_base: u32,
}

#[derive(Deserialize)]
pub struct EncryptDecryptRequest {
    pub message: String,
    pub key_pair: KeyPair,
    pub number_system_base: u32,
}

#[derive(Deserialize)]
pub struct SignRequest {
    pub plaintext: String,
    pub key_pair: KeyPair,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub plaintext: String,
    pub signature: String,
    pub key_pair: KeyPair,
}

#[derive(Deserialize)]
pub struct UseFastQuery {
    pub use_fast: bool,
}
