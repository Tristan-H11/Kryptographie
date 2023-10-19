use crate::encryption::keygen_service::KeyGenService;
use ibig::UBig;

/// Implementierung des KeyGenService-Traits für RSA.
pub struct RsaKeygenService {
    prime_one: UBig,
    prime_two: UBig,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    ///
    /// * `prime_one` - Die erste Primzahl.
    /// * `prime_two` - Die zweite Primzahl.
    ///
    pub fn new(prime_one: &UBig, prime_two: &UBig) -> RsaKeygenService {
        RsaKeygenService {
            prime_one: prime_one.clone(),
            prime_two: prime_two.clone(),
        }
    }
}

impl KeyGenService for RsaKeygenService {
    fn generate_keypair(&self) -> (String, String) {
        todo!("Implementiere diese Funktion!")
    }
}
