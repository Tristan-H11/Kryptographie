use crate::encryption::rsa::keys::{RsaKey, RsaKeyType};
use bigdecimal::num_bigint::ParseBigIntError;
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
    /// Wandelt das serialisierte Schlüsselpaar in einen privaten Schlüssel um.
    ///
    /// # Rückgabe
    /// * `RsaKey` - Der private Schlüssel.
    ///
    /// # Fehler
    /// * `ParseBigIntError` - Falls die BigInts nicht geparst werden können.
    pub(crate) fn to_private_key(&self) -> Result<RsaKey, ParseBigIntError> {
        debug!("Serialisiere KeyPair zu PrivateKey");
        Ok(RsaKey::new(
            RsaKeyType::Private,
            self.d.parse()?,
            self.modulus.parse()?,
        ))
    }

    /// Wandelt das serialisierte Schlüsselpaar in einen öffentlichen Schlüssel um.
    ///
    /// # Rückgabe
    /// * `RsaKey` - Der öffentliche Schlüssel.
    ///
    /// # Fehler
    /// * `ParseBigIntError` - Falls die BigInts nicht geparst werden können.
    pub(crate) fn to_public_key(&self) -> Result<RsaKey, ParseBigIntError> {
        debug!("Serialisiere KeyPair zu PublicKey");
        Ok(RsaKey::new(
            RsaKeyType::Public,
            self.e.parse()?,
            self.modulus.parse()?,
        ))
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
