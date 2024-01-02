use crate::encryption::rsa::keys::{RsaKey, RsaKeyType};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub modulus: String,
    pub e: String,
    pub d: String,
    pub block_size_pub: String,
    pub block_size_priv: String,
}

impl KeyPair {
    pub(crate) fn to_private_key(&self) -> RsaKey {
        // TODO Serialiserungsfehler behandeln
        debug!("Serialisiere KeyPair zu PrivateKey");
        RsaKey::new(
            RsaKeyType::Private,
            self.d.parse().unwrap(),
            self.modulus.parse().unwrap(),
        )
    }

    pub(crate) fn to_public_key(&self) -> RsaKey {
        // TODO Serialiserungsfehler behandeln
        debug!("Serialisiere KeyPair zu PublicKey");
        RsaKey::new(
            RsaKeyType::Public,
            self.e.parse().unwrap(),
            self.modulus.parse().unwrap(),
        )
    }
}

#[derive(Serialize)]
pub struct SingleStringResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct EncryptDecryptRequest {
    pub message: String,
    pub key_pair: KeyPair,
    pub number_system_base: u32,
}

#[derive(Deserialize)]
pub struct UseFastQuery {
    pub use_fast: bool,
}
