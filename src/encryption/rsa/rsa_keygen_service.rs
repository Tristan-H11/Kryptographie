use crate::encryption::keygen_service::KeyGenService;
use crate::encryption::rsa::keys::{PrivateKey, PublicKey};

/// Implementierung des KeyGenService-Traits für RSA.
pub struct RsaKeygenService {
    width_one: usize,
    width_two: usize,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    ///
    /// * `prime_one` - Die Breite der ersten Primzahl.
    /// * `prime_two` - Die Breite der zweiten Primzahl.
    ///
    pub fn new(width_one: usize, width_two: usize) -> RsaKeygenService {
        RsaKeygenService {
            width_one,
            width_two,
        }
    }
}

impl KeyGenService for RsaKeygenService {
    fn generate_keypair(&self) -> (PrivateKey, PublicKey) {
        todo!("Implementiere diese Funktion!")
    }
}
