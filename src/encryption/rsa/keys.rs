use bigdecimal::num_bigint::BigUint;
use crate::encryption::encryption_services::{Decryption, Encryption, Signing, Verification};

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    e: BigUint,
    n: BigUint,
}

impl PublicKey {
    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    ///
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    ///
    pub fn new(e: BigUint, n: BigUint) -> PublicKey {
        PublicKey {
            e,
            n,
        }
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    d: BigUint,
    n: BigUint,
}

impl PrivateKey {
    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    ///
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    ///
    pub fn new(d: BigUint, n: BigUint) -> PrivateKey {
        PrivateKey {
            d,
            n,
        }
    }
}


impl Encryption for PublicKey {
    fn encrypt(&self, message: &BigUint) -> BigUint {
        todo!("Implementiere diese Funktion!")
    }
}

impl Verification for PublicKey {
    fn verify(&self, signature: &String, message: &String) -> bool {
        todo!("Implementiere diese Funktion!")
    }
}

impl Decryption for PrivateKey {
    fn decrypt(&self, message: &BigUint) -> BigUint {
        todo!("Implementiere diese Funktion!")
    }
}

impl Signing for PrivateKey {
    fn sign(&self, message: &String) -> String {
        todo!("Implementiere diese Funktion!")
    }
}
