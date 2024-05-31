use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor, KeyGenWithPrimeConfig,
    KeyGenerator,
};
use crate::encryption::core::el_gamal::keys::{
    ElGamalKeyPair, ElGamalPrivateKey, ElGamalPublicKey,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use log::debug;

pub struct ElGamalScheme;

impl EncryptionScheme for ElGamalScheme {}

impl AsymmetricEncryptionScheme for ElGamalScheme {}

/// Die Konfiguration für die Schlüsselgenerierung für das ElGamal-Kryptosystem in primen Restklassengruppen.
///
/// # Felder
/// * `modulus_width` - Die Breite des Modulus `p`.
/// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test bei der Generierung von Primzahlen.
/// * `random_seed` - Der Seed für die gleichverteilte Zufallszahlerzeugung.
/// * `number_theory_service` - Der Service für die Zahlentheorie.
#[derive(Clone, Debug)]
pub struct ElGamalKeyGenConfig {
    pub modulus_width: u32,
    pub miller_rabin_iterations: u32,
    pub random_seed: u32,
    pub number_theory_service: NumberTheoryService,
}

impl KeyGenWithPrimeConfig for ElGamalKeyGenConfig {
    fn characteristic(&self) -> u32 {
        self.modulus_width
    }

    fn miller_rabin_iterations(&self) -> u32 {
        self.miller_rabin_iterations
    }

    fn random_seed(&self) -> u32 {
        self.random_seed
    }

    fn number_theory_service(&self) -> NumberTheoryService {
        self.number_theory_service
    }
}

impl KeyGenerator<ElGamalPublicKey, ElGamalPrivateKey, ElGamalScheme> for ElGamalScheme {
    type KeyPair = ElGamalKeyPair;

    ///
    /// Generiert ein Schlüsselpaar für das ElGamal-Kryptosystem in primen Restklassengruppen.
    ///
    /// # Argumente
    /// * `config` - Die Konfiguration für den Schlüsselgenerierungsvorgang.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    fn generate_keypair(config: &impl KeyGenWithPrimeConfig) -> Self::KeyPair {
        debug!(
            "Generieren eines neuen ElGamal-Schlüsselpaares mit Konfiguration: {:?}",
            config
        );
        let random_generator =
            PseudoRandomNumberGenerator::new(config.random_seed(), config.number_theory_service());
        let counter = RelaxedCounter::new(1);

        // Generieren der sicheren Primzahl p und der Primitivwurzel g
        let (p, g) = random_generator.generate_secure_prime_with_primitive_root(
            config.characteristic(),
            config.miller_rabin_iterations(),
            &counter,
        );

        // Generieren des privaten Schlüssels x (Zufallszahl zwischen 1 und p-2)
        let p_minus_two = p.decrement().decrement();
        let x = random_generator.take(&1.into(), &p_minus_two, &counter);

        // Berechnen des öffentlichen Schlüsselwertes y
        let y = config
            .number_theory_service()
            .fast_exponentiation(&g, &x, &p);

        ElGamalKeyPair {
            public_key: ElGamalPublicKey { p: p.clone(), g, y },
            private_key: ElGamalPrivateKey { p, x },
        }
    }
}

impl Encryptor<ElGamalScheme> for ElGamalScheme {
    type Input = BigInt;
    type Output = (BigInt, BigInt);
    type Key = ElGamalPublicKey;
}

impl AsymmetricEncryptor<ElGamalScheme> for ElGamalScheme {
    /// Verschlüsselt eine Nachricht mit dem öffentlichen Schlüssel des ElGamal-Kryptosystems in primen Restklassengruppen.
    ///
    /// # Argumente
    /// * `key` - Der öffentliche Schlüssel.
    /// * `plaintext` - Die zu verschlüsselnde Nachricht.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem verschlüsselten Nachrichtenteil `a` und dem zweiten verschlüsselten Nachrichtenteil `b`.
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let p = &key.p;
        let g = &key.g;
        let y = &key.y;

        // Generieren des Zufallszahl k (Zufallszahl zwischen 1 und p-2)
        let p_minus_two = p.decrement().decrement();
        let k = service.take_random_number_in_range(&1.into(), &p_minus_two);

        // Berechnen des ersten verschlüsselten Nachrichtenteils c1
        let a = service.number_theory_service.fast_exponentiation(g, &k, p);

        // Berechnen des zweiten verschlüsselten Nachrichtenteils c2
        let b = (service.number_theory_service.fast_exponentiation(y, &k, p) * plaintext) % p;

        (a, b)
    }
}

impl Decryptor<ElGamalScheme> for ElGamalScheme {
    type Input = (BigInt, BigInt);
    type Output = BigInt;
    type Key = ElGamalPrivateKey;
}

impl AsymmetricDecryptor<ElGamalScheme> for ElGamalScheme {
    /// Entschlüsselt eine Nachricht mit dem privaten Schlüssel des ElGamal-Kryptosystems in primen Restklassengruppen.
    ///
    /// # Argumente
    /// * `key` - Der private Schlüssel.
    /// * `ciphertext` - Das zu entschlüsselnde Nachrichtentupel.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// Die entschlüsselte Nachricht.
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let p = &key.p;
        let x = &key.x;
        let (a, b) = ciphertext;

        // Berechnen von z = (a^x)^-1 mod p = a^(p-1-x) mod p
        let z = service
            .number_theory_service
            .fast_exponentiation(a, &(p.decrement() - x), p);

        // Berechnen des Klartextes m = b * z mod p
        let plaintext = (b * z) % p;

        plaintext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
    use bigdecimal::FromPrimitive;
    use std::time::SystemTime;

    #[test]
    fn test_el_gamal_key_generation_happy_flow() {
        let service = NumberTheoryService::new(Fast);
        let config = ElGamalKeyGenConfig {
            modulus_width: 32,
            miller_rabin_iterations: 100,
            // Pseudozufälliger Wert, weil dieser Test mit jedem Input erfolgreich sein muss
            random_seed: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
            number_theory_service: service,
        };

        let keypair = ElGamalScheme::generate_keypair(&config);
        let calculated_y = service.fast_exponentiation(
            &keypair.public_key.g,
            &keypair.private_key.x,
            &keypair.public_key.p,
        );

        // Das berechnete y muss mit dem y-Wert des öffentlichen Schlüssels übereinstimmen.
        // Besser lässt sich der Schlüssel nicht verifizieren, da die Generierung von p und g
        // zufällig ist und somit nicht vorhersehbar ist.
        assert_eq!(keypair.public_key.y, calculated_y);
    }

    #[test]
    fn test_el_gamal_encryption_decryption_happy_flow() {
        let service = NumberTheoryService::new(Fast);
        let config = ElGamalKeyGenConfig {
            modulus_width: 32,
            miller_rabin_iterations: 100,
            random_seed: 42,
            number_theory_service: service,
        };

        let service = NumberTheoryWithPrngService::new(Fast, 13);

        let keypair = ElGamalScheme::generate_keypair(&config);
        let public_key = keypair.public_key;
        let private_key = keypair.private_key;

        let plaintext = BigInt::from_i32(42).unwrap();
        let ciphertext = ElGamalScheme::encrypt(&public_key, &plaintext, &service);
        let decrypted_plaintext = ElGamalScheme::decrypt(&private_key, &ciphertext, &service);

        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_el_gamal_encryption_decryption_big_keys() {
        let service = NumberTheoryService::new(Fast);
        let config = ElGamalKeyGenConfig {
            modulus_width: 512,
            miller_rabin_iterations: 20,
            random_seed: 94,
            number_theory_service: service,
        };

        let service = NumberTheoryWithPrngService::new(Fast, 13);

        let keypair = ElGamalScheme::generate_keypair(&config);
        let public_key = keypair.public_key;
        let private_key = keypair.private_key;

        let plaintext = BigInt::from_i32(156776).unwrap();
        let ciphertext = ElGamalScheme::encrypt(&public_key, &plaintext, &service);
        println!("ciphertext: {:?}", ciphertext);
        let decrypted_plaintext = ElGamalScheme::decrypt(&private_key, &ciphertext, &service);

        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_trivial_with_zero_input() {
        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let config = ElGamalKeyGenConfig {
            modulus_width: 32,
            miller_rabin_iterations: 100,
            random_seed: 77,
            number_theory_service: NumberTheoryService::new(Fast),
        };

        let keypair = ElGamalScheme::generate_keypair(&config);
        let public_key = keypair.public_key;
        let private_key = keypair.private_key;

        let plaintext = BigInt::from_i32(0).unwrap();
        let ciphertext = ElGamalScheme::encrypt(&public_key, &plaintext, &service);
        // Der zweite Teil muss offensichtlich 0 sein, weil er ein Produkt mit 0 (plaintext) ist.
        assert_eq!(ciphertext.1, 0.into());
        println!("ciphertext: {:?}", ciphertext);
        let decrypted_plaintext = ElGamalScheme::decrypt(&private_key, &ciphertext, &service);

        assert_eq!(plaintext, decrypted_plaintext);
    }
}
