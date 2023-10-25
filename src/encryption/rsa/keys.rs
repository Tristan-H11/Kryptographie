use bigdecimal::num_bigint::BigUint;
use log::{debug, info};
use crate::encryption::math_functions::block_chiffre::{create_blocks_from_string, create_string_from_blocks};
use crate::encryption::math_functions::number_theory::fast_exponentiation;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    e: BigUint,
    n: BigUint,
    size: usize,
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
    pub fn new(e: BigUint, n: BigUint, size: usize) -> PublicKey {
        PublicKey {
            e,
            n,
            size
        }
    }

    ///
    /// Gibt den öffentlichen Exponenten als String zurück.
    ///
    pub fn get_e(&self) -> String {
        self.e.to_str_radix(10)
    }

    pub(crate) fn encrypt(&self, message: &str) -> Vec<BigUint> {
        // self.n.bits in bytes wandeln (/8) und dann nochmal halbieren, um genug Abstand zu n zu bekommen.
        let block_size = (self.size / (8 * 2));
        println!("Verschlüsseln mit blockgröße {}", block_size);

        let chunks = create_blocks_from_string(message, block_size as usize, true);
        let encrypted_chunks = chunks.iter()
            .map(|chunk| fast_exponentiation(chunk, &self.e, &self.n))
            .collect();
        encrypted_chunks
        // create_string_from_blocks(encrypted_chunks)
    }

    pub(crate) fn verify(&self, signature: &str, message: &str) -> bool {
        todo!("Implementiere diese Funktion!")
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    d: BigUint,
    n: BigUint,
    size: usize
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
    pub fn new(d: BigUint, n: BigUint, size: usize) -> PrivateKey {
        PrivateKey {
            d,
            n,
            size
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

    pub(crate) fn decrypt(&self, message: &Vec<BigUint>) -> String {
        // info!("Entschlüsseln mit blockgröße {}", block_size);

        // let chunks = create_blocks_from_string(message, block_size as usize, false);
        let decrypted_chunks = message.iter()
            .map(|chunk| fast_exponentiation(chunk, &self.d, &self.n))
            .collect();

        create_string_from_blocks(decrypted_chunks)
    }

    pub(crate) fn sign(&self, message: &str) -> String {
        todo!("Implementiere diese Funktion!")
    }
}
