use serde::{Deserialize, Serialize};

pub trait FromToSerializable {
    type T;
    ///
    /// Erstellt aus dem Datenmodell ein Modell für die Serialisierung.
    ///
    fn to_serializable(&self) -> Self::T;

    ///
    /// Erstellt aus dem Modell für die Serialisierung ein Datenmodell.
    ///
    fn from_serializable(serializable: &Self::T) -> Self;
}

impl FromToSerializable for crate::encryption::rsa::keys::PublicKey {
    type T = PublicKey;

    fn to_serializable(&self) -> Self::T {
        PublicKey {
            modulus: self.get_n_as_str(),
            e: self.get_e_as_str(),
            block_size: self.get_block_size_as_str(),
        }
    }

    fn from_serializable(serializable: &Self::T) -> Self {
        crate::encryption::rsa::keys::PublicKey::new_with_blocksize(
            serializable.e.parse().unwrap(),
            serializable.modulus.parse().unwrap(),
            serializable.block_size.parse().unwrap()
        )
    }
}

impl FromToSerializable for crate::encryption::rsa::keys::PrivateKey {
    type T = PrivateKey;

    fn to_serializable(&self) -> Self::T {
        PrivateKey {
            modulus: self.get_n_as_str(),
            d: self.get_d_as_str(),
            block_size: self.get_block_size_as_str(),
        }
    }

    fn from_serializable(serializable: &Self::T) -> Self {
        crate::encryption::rsa::keys::PrivateKey::new_with_blocksize(
            serializable.d.parse().unwrap(),
            serializable.modulus.parse().unwrap(),
            serializable.block_size.parse().unwrap()
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    pub modulus: String,
    pub e: String,
    pub block_size: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateKey {
    pub modulus: String,
    pub d: String,
    pub block_size: String,
}


#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

#[derive(Serialize)]
pub struct SingleStringResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct CreateKeyPairRequest {
    pub modulus_width: u32,
    pub miller_rabin_rounds: u32,
    pub random_seed: u32,
    pub number_system_base: u32,
}

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub plaintext: String,
    pub public_key: PublicKey,
    pub number_system_base: u32,
}