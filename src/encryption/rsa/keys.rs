use bigdecimal::num_bigint::BigInt;
use log::debug;
use sha2::Digest;

use crate::big_i;
use crate::encryption::math_functions::traits::logarithm::Logarithm;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    pub e: BigInt,
    pub n: BigInt,
    pub block_size: usize,
}

impl PublicKey {
    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    pub fn new(e: BigInt, n: BigInt, g_base: u32) -> PublicKey {
        // Maximale Blockbreite = log_g(n).
        let block_size = n.log(&big_i!(g_base));
        debug!("Blocksize in der PublicKey-Erstellung: {}", block_size);
        PublicKey { e, n, block_size }
    }

    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `block_size` - Die Blockgröße.
    pub fn new_with_blocksize(e: BigInt, n: BigInt, block_size: usize) -> PublicKey {
        // Maximale Blockbreite = log_g(n).
        debug!("Blocksize in der PublicKey-Erstellung: {}", block_size);
        PublicKey { e, n, block_size }
    }

    ///
    /// Gibt den öffentlichen Exponenten als String zurück.
    ///
    pub fn get_e_as_str(&self) -> String {
        self.e.to_str_radix(10)
    }

    ///
    /// Gibt den Modul als String zurück.
    ///
    pub fn get_n_as_str(&self) -> String {
        self.n.to_str_radix(10)
    }

    ///
    /// Gibt die Blockgröße als String zurück.
    ///
    pub fn get_block_size_as_str(&self) -> String {
        self.block_size.to_string()
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    pub d: BigInt,
    pub n: BigInt,
    pub block_size: usize,
}

impl PrivateKey {
    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    pub fn new(d: BigInt, n: BigInt, g_base: u32) -> PrivateKey {
        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        let block_size = n.log(&big_i!(g_base)) + 1;
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }

    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `block_size` - Die Blockgröße.
    pub fn new_with_blocksize(d: BigInt, n: BigInt, block_size: usize) -> PrivateKey {
        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }

    ///
    /// Gibt den privaten Exponenten als String zurück.
    ///
    pub fn get_d_as_str(&self) -> String {
        self.d.to_str_radix(10)
    }

    ///
    /// Gibt den Modul als String zurück.
    ///
    pub fn get_n_as_str(&self) -> String {
        self.n.to_str_radix(10)
    }

    ///
    /// Gibt die Blockgröße als String zurück.
    ///
    pub fn get_block_size_as_str(&self) -> String {
        self.block_size.to_string()
    }
}

#[cfg(test)]
mod rsa_keys_test {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_happy_flow_1024() {
        // Intensiver Test, der die Verschlüsselung und Entschlüsselung wiederholt testet.
        let message = "bbbbbbbbbbbbbbb  äääääääääääääää  !&    ";
        let range = 2; // TODO hochstellen, wenn nötig

        let result = (0..range).into_par_iter().all(|_| {
            let keygen_service = RsaKeygenService::new(1024);
            let (public_key, private_key) = keygen_service.generate_keypair(40, 23, 55296, false); //TODO UseFast einbauen

            let encrypted_message = public_key.encrypt(message, 55296, false); //TODO UseFast einbauen
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = private_key.decrypt(&encrypted_message, 55296, false); //TODO UseFast einbauen
            message.trim_end() == decrypted_message
        });
        assert!(result);
    }

    #[test]
    fn test_happy_flow_1024_var_2() {
        let message = "Hallo wie geht es dir?";
        let keygen_service = RsaKeygenService::new(1024);
        let (public_key, private_key) = keygen_service.generate_keypair(40, 13, 55296, false); //TODO UseFast einbauen;

        let encrypted_message = public_key.encrypt(message, 55296, false); //TODO UseFast einbauen
        println!("Verschlüsselte Nachricht: {}", encrypted_message);

        let decrypted_message = private_key.decrypt(&encrypted_message, 55296, false); //TODO UseFast einbauen

        assert_eq!(message.trim_end(), decrypted_message);
    }

    #[test]
    fn test_sign_and_verify_lowest_possible_happy_flow() {
        let keygen = RsaKeygenService::new(258);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(10, 17, g_base, false); //TODO UseFast einbauen

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message, false); //TODO UseFast einbauen

        let is_valid = public_key.verify(&signature, &message, false); //TODO UseFast einbauen
        assert!(is_valid);
    }

    #[test]
    fn test_sign_and_verify_highest_unhappy_flow() {
        let keygen = RsaKeygenService::new(256);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(10, 13, g_base, false); //TODO UseFast einbauen

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message, false); //TODO UseFast einbauen

        let is_valid = public_key.verify(&signature, &message, false); //TODO UseFast einbauen
        assert!(!is_valid);
    }
}
