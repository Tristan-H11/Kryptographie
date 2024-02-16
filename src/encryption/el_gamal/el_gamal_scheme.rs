use crate::encryption::asymmetric_encryption_types::{
    AsymmetricEncryptionScheme, KeyGenWithPrimeConfig, KeyGenerator,
};
use crate::encryption::el_gamal::keys::{ElGamalKeyPair, ElGamalPrivateKey, ElGamalPublicKey};
use crate::encryption::encryption_types::EncryptionScheme;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
use atomic_counter::RelaxedCounter;
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
