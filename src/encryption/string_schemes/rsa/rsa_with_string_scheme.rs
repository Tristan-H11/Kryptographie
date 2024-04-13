use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor, KeyGenWithPrimeConfig,
    KeyGenerator, Signer, Verifier,
};

use crate::encryption::core::rsa::rsa_scheme::RsaScheme;
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::encryption::string_schemes::decimal_unicode_schemes::from_decimal_block_scheme::FromDecimalBlockScheme;
use crate::encryption::string_schemes::decimal_unicode_schemes::keys::DecimalUnicodeConversionSchemeKey;
use crate::encryption::string_schemes::decimal_unicode_schemes::to_decimal_block_scheme::ToDecimalBlockScheme;
use crate::encryption::string_schemes::rsa::keys::{
    RsaWithStringKeyPair, RsaWithStringPrivateKey, RsaWithStringPublicKey,
};
use crate::encryption::symmetric_encryption_types::{SymmetricDecryptor, SymmetricEncryptor};
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::traits::logarithm::Logarithm;
use crate::shared::hashing::sha256;

pub struct RsaWithStringScheme {}

impl EncryptionScheme for RsaWithStringScheme {}

impl AsymmetricEncryptionScheme for RsaWithStringScheme {}

impl<'a> Encryptor<RsaWithStringScheme> for RsaWithStringScheme {
    type Input = str;
    type Output = String;
    type Key = RsaWithStringPublicKey;
}

impl AsymmetricEncryptor<RsaWithStringScheme> for RsaWithStringScheme {
    /// Verschlüsselt eine beliebig lange Zeichenkette, in dem diese in Blöcke fester Größe aufgeteilt,
    /// und dann unter einer Decimal-Unicode-Abbildung mittels RSA verschlüsselt werden.
    /// Zu beachten ist dabei, dass nicht-volle Blöcke mit führenden Unicode-Nullen aufgefüllt werden.
    ///
    /// # Argumente
    /// * `key` - Der zu verwendende Schlüssel.
    /// * `plaintext` - Der zu verschlüsselnde Klartext.
    /// * `service` - Der zu verwendende NumberTheoryService.
    ///
    /// # Rückgabe
    /// * `String` - Die verschlüsselte Nachricht.
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        let radix = key.radix;
        let block_size = key.rsa_public_key.n.log(&radix.into());
        let pre_key = DecimalUnicodeConversionSchemeKey { radix, block_size };
        let rsa_key = &key.rsa_public_key;

        let chunks = ToDecimalBlockScheme::encrypt(plaintext, &pre_key);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| RsaScheme::encrypt(rsa_key, chunk, service))
            .collect();

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        let post_key = DecimalUnicodeConversionSchemeKey {
            radix,
            block_size: block_size + 1,
        };
        FromDecimalBlockScheme::encrypt(&encrypted_chunks, &post_key)
    }
}

impl<'a> Decryptor<RsaWithStringScheme> for RsaWithStringScheme {
    type Input = str;
    type Output = String;
    type Key = RsaWithStringPrivateKey;
}

impl AsymmetricDecryptor<RsaWithStringScheme> for RsaWithStringScheme {
    /// Entschlüsselt eine beliebig lange Zeichenkette, in dem diese in Blöcke fester Größe aufgeteilt,
    /// und dann unter einer Decimal-Unicode-Abbildung mittels RSA entschlüsselt werden.
    /// Im Regelfall wurde diese Nachricht vorher mit `encrypt` verschlüsselt und dann ist davon
    /// auszugehen, dass die verschlüsselten Blöcke immer um 1 größer sind als die Klartextblöcke.
    ///
    /// # Argumente
    /// * `key` - Der zu verwendende Schlüssel.
    /// * `ciphertext` - Der zu verschlüsselnde Klartext.
    /// * `service` - Der zu verwendende NumberTheoryService.
    ///
    /// # Rückgabe
    /// * `String` - Die entschlüsselte Nachricht.
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        let radix = key.radix;
        let rsa_key = &key.rsa_private_key;
        let block_size = rsa_key.n.log(&radix.into()) + 1;

        let unicode_conversion_key = DecimalUnicodeConversionSchemeKey { radix, block_size };
        let chunks = FromDecimalBlockScheme::decrypt(ciphertext, &unicode_conversion_key);
        let decrypted_chunks = chunks
            .iter()
            .map(|chunk| RsaScheme::decrypt(rsa_key, chunk, service))
            .collect();

        ToDecimalBlockScheme::decrypt(&decrypted_chunks, &unicode_conversion_key)
    }
}

impl<'a> Signer<RsaWithStringScheme> for RsaWithStringScheme {
    type Input = str;
    type Output = String;
    type Key = RsaWithStringPrivateKey;

    /// Signiert eine Nachricht. Dafür wird die Nachricht gehasht und dann unter einer
    /// Decimal-Unicode-Abbildung mittels RSA signiert.
    ///
    /// # Argumente
    /// * `key` - Der private Schlüssel.
    /// * `message` - Die zu signierende Nachricht.
    /// * `service` - Der zu verwendende NumberTheoryService.
    ///
    /// # Rückgabe
    /// * `String` - Die Signatur.
    ///
    fn sign(key: &Self::Key, message: &Self::Input, service: NumberTheoryService) -> Self::Output {
        let radix = key.radix;
        let rsa_key = &key.rsa_private_key;
        let block_size = rsa_key.n.log(&radix.into());
        let hashed_message = sha256(message).to_str_radix(10);

        let pre_key = DecimalUnicodeConversionSchemeKey { radix, block_size };
        let chunks = ToDecimalBlockScheme::encrypt(&hashed_message, &pre_key);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| RsaScheme::sign(rsa_key, chunk, service))
            .collect();

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        let post_key = DecimalUnicodeConversionSchemeKey {
            radix,
            block_size: block_size + 1,
        };
        FromDecimalBlockScheme::encrypt(&encrypted_chunks, &post_key)
    }
}

impl<'a> Verifier<RsaWithStringScheme> for RsaWithStringScheme {
    type Signature = str;
    type Message = str;
    type Output = bool;
    type Key = RsaWithStringPublicKey;

    /// Verifiert eine Nachricht gegen eine Signatur.
    ///
    /// # Argumente
    /// * `key` - Der öffentliche Schlüssel.
    /// * `signature` - Die Signatur.
    /// * `message` - Die Nachricht.
    /// * `service` - Der zu verwendende NumberTheoryService.
    ///
    /// # Rückgabe
    /// * `bool` - Gibt an, ob die Verifizierung erfolgreich war.
    fn verify(
        key: &Self::Key,
        signature: &Self::Signature,
        message: &Self::Message,
        service: NumberTheoryService,
    ) -> Self::Output {
        let radix = key.radix;
        let rsa_key = &key.rsa_public_key;
        let block_size = rsa_key.n.log(&radix.into());

        let message_unicode_conversion_key =
            DecimalUnicodeConversionSchemeKey { radix, block_size };
        let signature_unicode_conversion_key = DecimalUnicodeConversionSchemeKey {
            radix,
            block_size: block_size + 1,
        };

        let hashed_message = sha256(message).to_str_radix(10);
        // Die g-adisch entwickelten Werte der gehashten Nachricht
        let message_chunks =
            ToDecimalBlockScheme::encrypt(&hashed_message, &message_unicode_conversion_key);
        // Die verschlüsselten Werte der Signatur
        let encrypted_signature_chunks =
            FromDecimalBlockScheme::decrypt(signature, &signature_unicode_conversion_key);

        message_chunks
            .iter()
            .zip(encrypted_signature_chunks.iter())
            .all(|(message_chunk, encrypted_signature_chunk)| {
                RsaScheme::verify(rsa_key, message_chunk, encrypted_signature_chunk, service)
            })
    }
}

impl RsaWithStringScheme {
    // TODO KeyGenConfig anpassen? Das hier passt nicht mehr ins Muster

    fn generate_keypair(config: &impl KeyGenWithPrimeConfig, radix: u32) -> RsaWithStringKeyPair {
        let rsa_key_pair = RsaScheme::generate_keypair(config);

        let public_key = RsaWithStringPublicKey {
            rsa_public_key: rsa_key_pair.public_key,
            radix,
        };
        let private_key = RsaWithStringPrivateKey {
            rsa_private_key: rsa_key_pair.private_key,
            radix,
        };

        RsaWithStringKeyPair {
            public_key,
            private_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::core::rsa::rsa_scheme::RsaKeyGenConfig;
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
        Fast, Slow,
    };

    use super::*;

    fn run_test_for_all_services(test: impl Fn(NumberTheoryService)) {
        test(NumberTheoryService::new(Slow)); // Langsame, eigene Implementierung
        test(NumberTheoryService::new(Fast)); // Schnelle, externe Implementierung
    }

    #[test]
    fn test_encrypt_decrypt_happy_flow_1024() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!";

            let config = RsaKeyGenConfig {
                key_size: 1024,
                miller_rabin_iterations: 30,
                random_seed: 73,
                number_theory_service: service.clone(),
            };
            encryption_decryption_assert(config, message, service);
        });
    }

    #[test]
    fn test_encrypt_decrypt_empty_string() {
        run_test_for_all_services(|service| {
            let message = "";

            let config = RsaKeyGenConfig {
                key_size: 512,
                miller_rabin_iterations: 30,
                random_seed: 3,
                number_theory_service: service.clone(),
            };
            encryption_decryption_assert(config, message, service);
        });
    }

    #[test]
    fn test_with_key_size_64() {
        run_test_for_all_services(|service| {
            let message = "afsöoilj4j ae fae lör e59afß3 2öadf 0";

            let config = RsaKeyGenConfig {
                key_size: 64,
                miller_rabin_iterations: 30,
                random_seed: 874,
                number_theory_service: service.clone(),
            };
            encryption_decryption_assert(config, message, service);
        });
    }

    /// Diese Methode führt die Verschlüsselung und Entschlüsselung einer Nachricht mit einem
    /// gegebenen Schlüssel durch und prüft, ob die ursprüngliche Nachricht wiederhergestellt werden kann.
    fn encryption_decryption_assert(
        config: RsaKeyGenConfig,
        message: &str,
        service: NumberTheoryService,
    ) {
        let radix = 55296;
        let key_pair = RsaWithStringScheme::generate_keypair(&config, radix);
        let (public_key, private_key) = (&key_pair.public_key, &key_pair.private_key);

        let encrypted_message = RsaWithStringScheme::encrypt(public_key, message, service.clone());

        let decrypted_message =
            RsaWithStringScheme::decrypt(private_key, &encrypted_message, service.clone());
        assert_eq!(message, decrypted_message);
    }

    #[test]
    fn test_sign_verify_happy_flow_1024() {
        run_test_for_all_services(|service| {
            let message = "    Das ist eine ganz 456$§% / Testnachricht für die Signatur!    ";

            let config = RsaKeyGenConfig {
                key_size: 1024,
                miller_rabin_iterations: 30,
                random_seed: 653,
                number_theory_service: service.clone(),
            };
            sign_verify_assert(config, 55296, message, service, true);
        });
    }

    #[test]
    fn test_sign_verify_with_line_break_end() {
        run_test_for_all_services(|service| {
            let message =
                "Das ist eine ganz interessante Testnachricht für die Signatur!\r\n \n \n";

            let config = RsaKeyGenConfig {
                key_size: 512,
                miller_rabin_iterations: 30,
                random_seed: 55,
                number_theory_service: service.clone(),
            };
            sign_verify_assert(config, 55296, message, service, true);
        });
    }

    #[test]
    fn test_sign_and_verify_lowest_possible_happy_flow() {
        run_test_for_all_services(|service| {
            let message = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";

            let config = RsaKeyGenConfig {
                key_size: 257,
                miller_rabin_iterations: 30,
                random_seed: 40,
                number_theory_service: service.clone(),
            };
            sign_verify_assert(config, 55296, message, service, true);
        });
    }

    #[test]
    fn fail_signature_with_different_messages() {
        run_test_for_all_services(|service| {
            let message_one = "Das ist eine ganz interessante Testnachricht für die Signatur!    ";
            let message_two = "Das ist eine andere Nachricht";

            let config = RsaKeyGenConfig {
                key_size: 512,
                miller_rabin_iterations: 30,
                random_seed: 17,
                number_theory_service: service.clone(),
            };
            let radix = 55296;
            let key_pair = RsaWithStringScheme::generate_keypair(&config, radix);
            let (public_key, private_key) = (&key_pair.public_key, &key_pair.private_key);

            let _radix = 55296;

            let signature = RsaWithStringScheme::sign(private_key, message_one, service.clone());

            let is_valid =
                RsaWithStringScheme::verify(public_key, &signature, message_two, service.clone());
            assert!(!is_valid);
        });
    }

    fn sign_verify_assert(
        config: RsaKeyGenConfig,
        radix: u32,
        message: &str,
        service: NumberTheoryService,
        expected: bool,
    ) {
        let key_pair = RsaWithStringScheme::generate_keypair(&config, radix);
        let (public_key, private_key) = (&key_pair.public_key, &key_pair.private_key);

        let signature = RsaWithStringScheme::sign(private_key, message, service.clone());

        let is_valid =
            RsaWithStringScheme::verify(public_key, &signature, message, service.clone());
        assert_eq!(expected, is_valid);
    }
}
