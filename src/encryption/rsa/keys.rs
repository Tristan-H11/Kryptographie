use bigdecimal::num_bigint::BigUint;
use crate::encryption::encryption_services::{Encryption, Signing, Verification};

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    e: BigUint,
    n: BigUint,
}

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    d: BigUint,
    n: BigUint,
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

impl Encryption for PrivateKey {
    fn encrypt(&self, message: &BigUint) -> BigUint {
        todo!("Implementiere diese Funktion!")
    }
}

impl Signing for PrivateKey {
    fn sign(&self, message: &String) -> String {
        todo!("Implementiere diese Funktion!")
    }
}
