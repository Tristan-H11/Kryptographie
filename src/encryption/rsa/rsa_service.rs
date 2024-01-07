use bigdecimal::num_bigint::{BigInt, Sign};
use log::{debug, info};
use sha2::{Digest, Sha256};

use crate::encryption::math_functions::block_chiffre::{
    create_string_from_blocks_decrypt, create_string_from_blocks_encrypt, encode_string_to_blocks,
};
use crate::encryption::math_functions::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::encryption::math_functions::traits::logarithm::Logarithm;
use crate::encryption::rsa::keys::{RsaKey, RsaKeyType};

pub struct RsaService {
    number_theory_service: NumberTheoryService,
}

impl RsaService {
    pub fn new(number_theory_service: NumberTheoryService) -> RsaService {
        RsaService {
            number_theory_service,
        }
    }

    /// Verschlüsselt eine Nachricht. Die Nachricht wird in Blöcke der Größe `block_size` aufgeteilt und dann
    /// verschlüsselt. Die verschlüsselten Blöcke werden dann zu einem String zusammengefügt.
    /// Nicht-volle Blöcke werden mit führenden Nullen aufgefüllt.
    ///
    /// # Argumente
    /// * `message` - Die zu verschlüsselnde Nachricht.
    /// **ACHTUNG**: Leerzeichen am Ende werden entfernt!
    /// * `g_base` - Die Basis des Zeichensatzes, in der die Nachricht verschlüsselt werden soll.
    /// * `key` - Der zu verwendende Schlüssel.
    ///
    /// # Rückgabe
    /// * `String` - Die verschlüsselte Nachricht.
    pub(crate) fn encrypt(&self, message: &str, g_base: u32, key: &RsaKey) -> String {
        let block_size = key.modulus().log(&g_base.into());
        info!("Verschlüsseln mit blockgröße {}", block_size);

        let chunks = encode_string_to_blocks(message.trim_end(), block_size, g_base);
        let encrypted_chunks = self.exponentiation_each(&key, chunks);

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        create_string_from_blocks_encrypt(encrypted_chunks, block_size + 1, g_base)
    }

    /// Entschlüsselt eine Nachricht. Unterscheidet sich zur Funktion `encrypt` nur in der Hinsicht, dass hier angenommen
    /// wird, dass die verschlüsselten Blöcke immer um 1 größer sind als die Klartextblöcke, in denen die übergebene
    /// Nachricht verschlüsselt wurde.
    ///
    /// # Argumente
    /// * `message` - Die zu entschlüsselnde Nachricht.
    /// * `g_base` - Die Basis des Zeichensatzes, in der die Nachricht verschlüsselt wurde.
    /// * `key` - Der zu verwendende Schlüssel.
    ///
    /// # Rückgabe
    /// * `String` - Die entschlüsselte Nachricht.
    pub(crate) fn decrypt(&self, message: &str, g_base: u32, key: &RsaKey) -> String {
        let block_size = key.modulus().log(&g_base.into()) + 1;
        info!("Entschlüsseln mit blockgröße {}", block_size);

        let chunks = encode_string_to_blocks(message, block_size, g_base);
        let decrypted_chunks = self.exponentiation_each(&key, chunks);

        create_string_from_blocks_decrypt(decrypted_chunks, g_base)
    }

    /// Wendet die schnelle Exponentation auf jedes Element des Vektors an.
    /// Dabei wird der Exponent und der Modul aus dem übergebenen Schlüssel bezogen.
    ///
    /// # Argumente
    /// * `key` - Der Schlüssel mit den Parametern für die Exponentiation.
    /// * `chunks` - Der Vektor mit den zu verrechnenden Elementen
    ///
    /// # Rückgabe
    /// Ein Vektor gleicher Größe mit umgerechneten Werten.
    fn exponentiation_each(&self, key: &&RsaKey, chunks: Vec<BigInt>) -> Vec<BigInt> {
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| {
                self.number_theory_service.fast_exponentiation(
                    chunk,
                    &key.exponent(),
                    &key.modulus(),
                )
            })
            .collect();
        encrypted_chunks
    }

    /// Signiert eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    /// * `message` - Die zu signierende Nachricht.
    /// * `key` - Der private Schlüssel.
    /// * `g_base` - Die Basis des Zeichensatzes, in der die Nachricht signiert werden soll.
    ///
    /// # Rückgabe
    /// * `String` - Die Signatur.
    pub(crate) fn sign(&self, message: &str, key: &RsaKey, g_base: u32) -> String {
        if key.key_type() != RsaKeyType::Private {
            panic!("Der Schlüssel muss privat sein, um eine Nachricht zu signieren!");
        }
        info!("Signieren der Nachricht {}", message);
        let hashed_message = RsaService::get_decimal_hash(message);

        let signature = self.encrypt(&hashed_message.to_str_radix(10), g_base, key);

        signature
    }

    /// Verifiziert eine Nachricht mit der Signatur.
    ///
    /// # Argumente
    /// * `signature` - Die Signatur.
    /// * `message` - Die Nachricht.
    /// * `key` - Der öffentliche Schlüssel.
    /// * `g_base` - Die Basis des Zeichensatzes, in der die Nachricht signiert werden soll.
    ///
    /// # Rückgabe
    /// * `bool` - Gibt an, ob die Verifizierung erfolgreich war.
    pub(crate) fn verify(&self, signature: &str, message: &str, key: &RsaKey, g_base: u32) -> bool {
        if key.key_type() != RsaKeyType::Public {
            panic!("Der Schlüssel muss öffentlich sein, um eine Nachricht zu verifizieren!");
        }
        info!(
            "Verifizieren der Nachricht {} mit Signatur {}",
            message, signature
        );
        let message_big_int = RsaService::get_decimal_hash(message).to_str_radix(10);

        let verification = self.decrypt(signature, g_base, key);

        verification == message_big_int
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

    /// Diese Methode verschlüsselt eine Zahl in Dezimaldarstellung mit dem öffentlichen Schlüssel.
    ///
    /// # Argumente
    /// * `input` - Die zu verschlüsselnde Zahl.
    /// * `public_key` - Der öffentliche Schlüssel.
    ///
    /// # Rückgabe
    /// * `BigInt` - Die verschlüsselte Zahl.
    pub(crate) fn encrypt_decrypt_number(&self, input: &BigInt, public_key: &RsaKey) -> BigInt {
        self.number_theory_service.fast_exponentiation(
            input,
            &public_key.exponent(),
            &public_key.modulus(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    use super::*;

    fn run_test_for_all_services(test: impl Fn(NumberTheoryService)) {
        test(NumberTheoryService::new(Slow)); // Langsame, eigene Implementierung
        test(NumberTheoryService::new(Fast)); // Schnelle, externe Implementierung
    }

    #[test]
    fn test_encrypt_decrypt_happy_flow_1024() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!";

            let keygen_service = RsaKeygenService::new(1024, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(40, 23).unwrap();

            let rsa_service = RsaService::new(service);

            let encrypted_message = rsa_service.encrypt(message, 55296, public_key);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = rsa_service.decrypt(&encrypted_message, 55296, private_key);
            assert_eq!(message, decrypted_message);
        });
    }

    #[test]
    fn test_encrypt_decrypt_empty_string() {
        run_test_for_all_services(|service| {
            let message = "";

            let keygen_service = RsaKeygenService::new(512, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(100, 346).unwrap();

            let rsa_service = RsaService::new(service);

            let encrypted_message = rsa_service.encrypt(message, 55296, public_key);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = rsa_service.decrypt(&encrypted_message, 55296, private_key);
            assert_eq!(message.trim_end(), decrypted_message);
        });
    }

    #[test]
    fn test_with_key_size_64() {
        run_test_for_all_services(|service| {
            let message = "afsöoilj4j ae fae lör e59afß3 2öadf 0";

            let keygen_service = RsaKeygenService::new(64, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(100, 2).unwrap();

            let rsa_service = RsaService::new(service);

            let encrypted_message = rsa_service.encrypt(message, 55296, public_key);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = rsa_service.decrypt(&encrypted_message, 55296, private_key);
            assert_eq!(message, decrypted_message);
        });
    }

    #[test]
    fn test_sign_verify_happy_flow_1024() {
        run_test_for_all_services(|service| {
            let message = "    Das ist eine ganz 456$§% / Testnachricht für die Signatur!    ";

            let keygen_service = RsaKeygenService::new(1024, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(40, 23).unwrap();

            let rsa_service = RsaService::new(service);

            let g_base = 55296;

            let signature = rsa_service.sign(message, private_key, g_base);

            let is_valid = rsa_service.verify(&signature, message, public_key, g_base);
            assert!(is_valid);
        });
    }

    #[test]
    fn test_sign_and_verify_lowest_possible_happy_flow() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";

            let keygen_service = RsaKeygenService::new(257, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(40, 23).unwrap();

            let rsa_service = RsaService::new(service);

            let g_base = 55296;

            let signature = rsa_service.sign(&message, private_key, g_base);

            let is_valid = rsa_service.verify(&signature, &message, public_key, g_base);
            assert!(is_valid);
        });
    }

    #[test]
    fn fail_signature_with_different_messages() {
        run_test_for_all_services(|service| {
            let message_one = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";
            let message_two = "Das ist eine andere Nachricht";

            let keygen_service = RsaKeygenService::new(1024, service.clone());
            let (public_key, private_key) = &keygen_service.generate_keypair(40, 24).unwrap();

            let rsa_service = RsaService::new(service);

            let g_base = 55296;

            let signature = rsa_service.sign(&message_one, private_key, g_base);

            let is_valid = rsa_service.verify(&signature, &message_two, public_key, g_base);
            assert!(!is_valid);
        });
    }

    #[test]
    fn test_encrypt_number() {
        run_test_for_all_services(|service| {
            let keygen_service = RsaKeygenService::new(256, service);
            let rsa_service = RsaService::new(service);

            let (public_key, private_key) = &keygen_service.generate_keypair(10, 19).unwrap();

            let message = BigInt::from_str("123456789").unwrap();

            let encrypted_message = rsa_service.encrypt_decrypt_number(&message, public_key);

            let decrypted_message =
                rsa_service.encrypt_decrypt_number(&encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        });
    }

    #[test]
    fn square_encrypted_numbers() {
        run_test_for_all_services(|service| {
            let keygen_service = RsaKeygenService::new(256, service);
            let rsa_service = RsaService::new(service.clone());

            let (public_key, private_key) = &keygen_service.generate_keypair(10, 19).unwrap();

            let message = BigInt::from_str("48153454374561835379").unwrap();

            let encrypted_message = rsa_service.encrypt_decrypt_number(&message, public_key);
            let squared_encrypted_message =
                service.fast_exponentiation(&encrypted_message, &2.into(), &public_key.modulus());

            let decrypted_message =
                rsa_service.encrypt_decrypt_number(&squared_encrypted_message, private_key);
            assert_eq!(message.pow(2), decrypted_message);
        });
    }

    #[test]
    fn multiply_different_encrypted_numbers() {
        run_test_for_all_services(|service| {
            let keygen_service = RsaKeygenService::new(256, service);
            let rsa_service = RsaService::new(service);

            let (public_key, private_key) = &keygen_service.generate_keypair(10, 19).unwrap();

            let message_one = BigInt::from_str("123456789").unwrap();

            let message_two = BigInt::from_str("987654321").unwrap();

            let encrypted_message_one =
                rsa_service.encrypt_decrypt_number(&message_one, public_key);
            let encrypted_message_two =
                rsa_service.encrypt_decrypt_number(&message_two, public_key);

            let multiplied_encrypted_message = encrypted_message_one * encrypted_message_two;

            let decrypted_message =
                rsa_service.encrypt_decrypt_number(&multiplied_encrypted_message, private_key);

            assert_eq!(message_one * message_two, decrypted_message);
        });
    }
}
