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

    ///
    /// Gibt den öffentlichen Exponenten als String zurück.
    ///
    pub fn get_e(&self) -> String {
        self.e.to_str_radix(10)
    }

    ///
    /// Gibt den Modul n zurück.
    ///
    pub fn get_n(&self) -> String {
        self.n.to_str_radix(10)
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

    ///
    /// Gibt den privaten Exponenten als String zurück.
    ///
    pub fn get_d(&self) -> String {
        self.d.to_str_radix(10)
    }

    ///
    /// Gibt den Modul n zurück.
    ///
    pub fn get_n(&self) -> String {
        self.n.to_str_radix(10)
    }
}


impl Encryption for PublicKey {
    fn encrypt(&self, message: &str) -> String {
        todo!("Implementiere diese Funktion!")
    }
}

impl Verification for PublicKey {
    fn verify(&self, signature: &str, message: &str) -> bool {
        todo!("Implementiere diese Funktion!")
    }
}

impl Decryption for PrivateKey {
    fn decrypt(&self, message: &str) -> String {
        todo!("Implementiere diese Funktion!")
    }
}

impl Signing for PrivateKey {
    fn sign(&self, message: &str) -> String {
        todo!("Implementiere diese Funktion!")
    }
}
