use crate::encryption::keygen_service::KeyGenService;
use crate::encryption::rsa::keys::{PublicKey, PrivateKey};

/// Implementierung des KeyGenService-Traits für RSA.
pub struct RsaKeygenService {
    key_size: usize,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    ///
    /// * `key_width` - Die Breite des Moduls `n`.
    ///
    pub fn new(key_size: usize) -> RsaKeygenService {
        RsaKeygenService {
            key_size,
        }
    }
}

impl KeyGenService for RsaKeygenService {
    fn generate_keypair(&self) -> (PublicKey, PrivateKey) {
        // die primzahlen sind die hälfte der länge des moduls
        todo!("Implementiere diese Funktion!")
    }
}
