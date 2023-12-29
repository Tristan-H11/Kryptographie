use bigdecimal::num_bigint::{BigInt, Sign};
use log::{debug, info};
use sha2::{Digest, Sha256};

use crate::encryption::math_functions::block_chiffre::{
    create_string_from_blocks_decrypt, create_string_from_blocks_encrypt, encode_string_to_blocks,
};
use crate::encryption::math_functions::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::encryption::rsa::keys::{PrivateKey, PublicKey};

pub struct RsaService {
    number_theory_service: NumberTheoryService,
}

impl RsaService {
    pub fn new(number_theory_service: NumberTheoryService) -> RsaService {
        RsaService {
            number_theory_service,
        }
    }

    /// Verschlüsselt eine Nachricht mit dem öffentlichen Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu verschlüsselnde Nachricht.
    /// **ACHTUNG**: Leerzeichen am Ende werden entfernt!
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    /// * `public_key` - Der öffentliche Schlüssel.
    ///
    /// # Rückgabe
    /// * `String` - Die verschlüsselte Nachricht.
    pub(crate) fn encrypt(&self, message: &str, g_base: u32, public_key: PublicKey) -> String {
        info!("Verschlüsseln mit blockgröße {}", public_key.block_size);

        let chunks =
            encode_string_to_blocks(message.trim_end(), public_key.block_size, true, g_base);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| {
                self.number_theory_service
                    .fast_exponentiation(chunk, &public_key.e, &public_key.n)
            })
            .collect();

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        create_string_from_blocks_encrypt(encrypted_chunks, public_key.block_size + 1, g_base)
    }

    /// Verifiziert eine Nachricht mit der Signatur.
    ///
    /// # Argumente
    /// * `signature` - Die Signatur.
    /// * `message` - Die Nachricht.
    /// * `public_key` - Der öffentliche Schlüssel.
    ///
    /// # Rückgabe
    /// * `bool` - Gibt an, ob die Verifizierung erfolgreich war.
    pub(crate) fn verify(&self, signature: &str, message: &str, public_key: PublicKey) -> bool {
        info!(
            "Verifizieren der Nachricht {} mit Signatur {}",
            message, signature
        );
        let message_big_int = RsaService::get_decimal_hash(message);

        // Signatur in BigInt umwandeln
        let signature_big_int = BigInt::parse_bytes(signature.as_bytes(), 10)
            .expect("Die Signatur konnte nicht in einen BigInt umgewandelt werden");

        // Verifizierung durchführen: verifizierung = signatur ^ (öffentlicher key vom partner) mod n
        let verification = self.number_theory_service.fast_exponentiation(
            &signature_big_int,
            &public_key.e,
            &public_key.n,
        );

        // Überprüfen, ob die Verifizierung mit der gehashten Nachricht übereinstimmt
        verification == message_big_int
    }

    /// Entschlüsselt eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu entschlüsselnde Nachricht.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt wurde.
    /// * `private_key` - Der private Schlüssel.
    ///
    /// # Rückgabe
    /// * `String` - Die entschlüsselte Nachricht.
    pub(crate) fn decrypt(&self, message: &str, g_base: u32, private_key: PrivateKey) -> String {
        info!("Entschlüsseln mit blockgröße {}", private_key.block_size);

        let chunks = encode_string_to_blocks(message, private_key.block_size, true, g_base);
        let decrypted_chunks = chunks
            .iter()
            .map(|chunk| {
                self.number_theory_service.fast_exponentiation(
                    chunk,
                    &private_key.d,
                    &private_key.n,
                )
            })
            .collect();

        create_string_from_blocks_decrypt(decrypted_chunks, g_base)
    }

    /// Signiert eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu signierende Nachricht.
    ///
    /// # Rückgabe
    /// * `String` - Die Signatur.
    pub(crate) fn sign(&self, message: &str, private_key: PrivateKey) -> String {
        info!("Signieren der Nachricht {}", message);
        let message_big_int = RsaService::get_decimal_hash(message);

        // Signatur berechnen: signatur = message^(eigener privater key) mod n
        let signature = self.number_theory_service.fast_exponentiation(
            &message_big_int,
            &private_key.d,
            &private_key.n,
        );

        // Signatur als String zurückgeben
        signature.to_str_radix(10)
    }

    /// Diese Methode berechnet den Hash einer Nachricht.
    ///
    /// # Argumente
    /// * `message` - Die Nachricht.
    ///
    /// # Rückgabe
    /// * `BigInt` - Der Hash.
    fn get_decimal_hash(message: &str) -> BigInt {
        debug!("Hashen der Nachricht {} mit SHA256", message);
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        let hashed_message = hasher.finalize();

        // Hash Nachricht in einen BigInt umwandeln
        let message_big_int = BigInt::from_bytes_be(Sign::Plus, &hashed_message);
        message_big_int
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;
    use super::*;

    fn run_test_for_all_services(test: impl Fn(NumberTheoryService)) {
        test(NumberTheoryService::new(Slow)); // Langsame, eigene Implementierung
        test(NumberTheoryService::new(Fast)); // Schnelle, externe Implementierung
    }

    #[test]
    fn test_happy_flow_1024() {
        run_test_for_all_services(|service| {
            let message = "bbbbbbbbbbbbbbb  äääääääääääääää  !&    ";

            let keygen_service = RsaKeygenService::new(1024, service.clone());
            let (public_key, private_key) = keygen_service.generate_keypair(40, 23, 55296);

            let rsa_service = RsaService::new(service);

            let encrypted_message = rsa_service.encrypt(message, 55296, public_key);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = rsa_service.decrypt(&encrypted_message, 55296, private_key);
            assert_eq!(message.trim_end(), decrypted_message);
        });
    }

    #[test]
    fn test_sign_and_verify_lowest_possible_happy_flow() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";

            let keygen_service = RsaKeygenService::new(258, service.clone());
            let (public_key, private_key) = keygen_service.generate_keypair(40, 23, 55296);

            let rsa_service = RsaService::new(service);

            let signature = rsa_service.sign(&message, private_key);

            let is_valid = rsa_service.verify(&signature, &message, public_key);
            assert!(is_valid);
        });
    }

    #[test]
    fn test_sign_and_verify_highest_unhappy_flow() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";

            let keygen_service = RsaKeygenService::new(256, service.clone());
            let (public_key, private_key) = keygen_service.generate_keypair(40, 23, 55296);

            let rsa_service = RsaService::new(service);

            let signature = rsa_service.sign(&message, private_key);

            let is_valid = rsa_service.verify(&signature, &message, public_key);
            assert!(!is_valid);
        });
    }
}
