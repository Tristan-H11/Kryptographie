use bigdecimal::num_bigint::BigUint;
use log::info;

use crate::big_u;
use crate::encryption::math_functions::big_int_util::log_base_g;
use crate::encryption::math_functions::block_chiffre::{
    create_blocks_from_string_decrypt, create_blocks_from_string_encript,
    create_string_from_blocks, create_string_from_blocks_decrypt,
};
use crate::encryption::math_functions::number_theory::fast_exponentiation;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    e: BigUint,
    n: BigUint,
    block_size: usize,
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
        // Maximale Blockbreite = log_g(n), wenn g=55296 ist.
        let g = big_u!(55296u16);
        let block_size = log_base_g(&n, &g) as usize;
        PublicKey { e, n, block_size }
    }

    ///
    /// Gibt den öffentlichen Exponenten als String zurück.
    ///
    pub fn get_e_as_str(&self) -> String {
        self.e.to_str_radix(10)
    }

    #[cfg(test)]
    pub fn get_e_for_test(&self) -> BigUint {
        self.e.clone()
    }

    #[cfg(test)]
    pub fn get_n_for_test(&self) -> BigUint {
        self.n.clone()
    }

    pub(crate) fn encrypt(&self, message: &str, base_length: u32) -> String {
        println!("Verschlüsseln mit blockgröße {}", self.block_size);

        let chunks =
            create_blocks_from_string_encript(message, self.block_size - 1, true, base_length);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| fast_exponentiation(chunk, &self.e, &self.n))
            .collect();

        create_string_from_blocks(encrypted_chunks)
    }

    pub(crate) fn verify(&self, _signature: &str, _message: &str) -> bool {
        todo!("Implementiere diese Funktion!")
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    d: BigUint,
    n: BigUint,
    block_size: usize,
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
        // Maximale Blockbreite = log_g(n), wenn g=55296 ist.
        let g = big_u!(55296u16);
        let block_size = log_base_g(&n, &g) as usize;
        PrivateKey { d, n, block_size }
    }

    ///
    /// Gibt den privaten Exponenten als String zurück.
    ///
    pub fn get_d_as_str(&self) -> String {
        self.d.to_str_radix(10)
    }

    #[cfg(test)]
    pub fn get_d(&self) -> BigUint {
        self.d.clone()
    }

    #[cfg(test)]
    pub fn get_n(&self) -> BigUint {
        self.n.clone()
    }

    pub(crate) fn decrypt(&self, message: &str, base_length: u32) -> String {
        info!("Entschlüsseln mit blockgröße {}", self.block_size);

        let chunks = create_blocks_from_string_decrypt(message, true, base_length);
        let decrypted_chunks = chunks
            .iter()
            .map(|chunk| fast_exponentiation(chunk, &self.d, &self.n))
            .collect();

        create_string_from_blocks_decrypt(decrypted_chunks)
    }

    pub(crate) fn sign(&self, _message: &str) -> String {
        todo!("Implementiere diese Funktion!")
    }
}
