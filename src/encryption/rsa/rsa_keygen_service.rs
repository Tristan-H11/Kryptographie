use crate::encryption::keygen_service::KeyGenService;
use bigdecimal::num_bigint::BigUint;

/// Implementierung des KeyGenService-Traits für RSA.
pub struct RsaKeygenService {
    prime_one: BigUint,
    prime_two: BigUint,
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
    pub fn new(prime_one: &BigUint, prime_two: &BigUint) -> RsaKeygenService {
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
