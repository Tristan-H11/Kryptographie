use bigdecimal::num_bigint::{BigInt, Sign};
use log::{debug, info};
use sha2::{Digest, Sha256};

use crate::big_i;
use crate::encryption::math_functions::block_chiffre::{
    encode_string_to_blocks,
    create_string_from_blocks_decrypt, create_string_from_blocks_encrypt,
};
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::traits::logarithm::Logarithm;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    e: BigInt,
    n: BigInt,
    block_size: usize,
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

    #[cfg(test)]
    pub fn get_e(&self) -> BigInt {
        self.e.clone()
    }

    #[cfg(test)]
    pub fn get_n(&self) -> BigInt {
        self.n.clone()
    }

    #[cfg(test)]
    pub fn get_block_size(&self) -> usize {
        self.block_size.clone()
    }

    ///
    /// Verschlüsselt eine Nachricht mit dem öffentlichen Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu verschlüsselnde Nachricht.
    /// **ACHTUNG**: Leerzeichen am Ende werden entfernt!
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    /// * `use_fast` - Gibt an, ob der schnelle Algorithmus verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `String` - Die verschlüsselte Nachricht.
    pub(crate) fn encrypt(&self, message: &str, g_base: u32, use_fast: bool) -> String {
        info!("Verschlüsseln mit blockgröße {}", self.block_size);

        let chunks =
            encode_string_to_blocks(message.trim_end(), self.block_size, true, g_base);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| FastExponentiation::calculate(chunk, &self.e, &self.n, use_fast))
            .collect();

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        create_string_from_blocks_encrypt(encrypted_chunks, self.block_size + 1, g_base)
    }

    /// Verifiziert eine Nachricht mit der Signatur.
    ///
    /// # Argumente
    /// * `signature` - Die Signatur.
    /// * `message` - Die Nachricht.
    /// * `use_fast` - Gibt an, ob der schnelle Algorithmus verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `bool` - Gibt an, ob die Verifizierung erfolgreich war.
    pub(crate) fn verify(&self, signature: &str, message: &str, use_fast: bool) -> bool {
        info!(
            "Verifizieren der Nachricht {} mit Signatur {}",
            message, signature
        );
        let message_big_int = get_decimal_hash(message);

        // Signatur in BigInt umwandeln
        let signature_big_int = BigInt::parse_bytes(signature.as_bytes(), 10)
            .expect("Die Signatur konnte nicht in einen BigInt umgewandelt werden");

        // Verifizierung durchführen: verifizierung = signatur ^ (öffentlicher key vom partner) mod n
        let verification =
            FastExponentiation::calculate(&signature_big_int, &self.e, &self.n, use_fast);

        // Überprüfen, ob die Verifizierung mit der gehashten Nachricht übereinstimmt
        verification == message_big_int
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    d: BigInt,
    n: BigInt,
    block_size: usize,
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

    #[cfg(test)]
    pub fn get_d(&self) -> BigInt {
        self.d.clone()
    }

    #[cfg(test)]
    pub fn get_n(&self) -> BigInt {
        self.n.clone()
    }

    #[cfg(test)]
    pub fn get_block_size(&self) -> usize {
        self.block_size.clone()
    }

    ///
    /// Entschlüsselt eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu entschlüsselnde Nachricht.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt wurde.
    /// * `use_fast` - Gibt an, ob der schnelle Algorithmus verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `String` - Die entschlüsselte Nachricht.
    pub(crate) fn decrypt(&self, message: &str, g_base: u32, use_fast: bool) -> String {
        info!("Entschlüsseln mit blockgröße {}", self.block_size);

        let chunks = encode_string_to_blocks(message, self.block_size, true, g_base);
        let decrypted_chunks = chunks
            .iter()
            .map(|chunk| FastExponentiation::calculate(chunk, &self.d, &self.n, use_fast))
            .collect();

        create_string_from_blocks_decrypt(decrypted_chunks, g_base)
    }

    /// Signiert eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu signierende Nachricht.
    /// * `use_fast` - Gibt an, ob der schnelle Algorithmus verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `String` - Die Signatur.
    pub(crate) fn sign(&self, message: &str, use_fast: bool) -> String {
        info!("Signieren der Nachricht {}", message);
        let message_big_int = get_decimal_hash(message);

        // Signatur berechnen: signatur = message^(eigener privater key) mod n
        let signature = FastExponentiation::calculate(&message_big_int, &self.d, &self.n, use_fast);

        // Signatur als String zurückgeben
        signature.to_str_radix(10)
    }
}

/// Diese Methode berechnet den Hash einer Nachricht.
///
/// # Argumente
/// * `message` - Die Nachricht.
///
/// # Rückgabe
/// * `BigInt` - Der Hash.
pub(crate) fn get_decimal_hash(message: &str) -> BigInt {
    debug!("Hashen der Nachricht {} mit SHA256", message);
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    // Hash Nachricht in einen BigInt umwandeln
    let message_big_int = BigInt::from_bytes_be(Sign::Plus, &hashed_message);
    message_big_int
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
