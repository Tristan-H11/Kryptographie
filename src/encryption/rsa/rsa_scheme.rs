use crate::encryption::asymmetric_encryption_types::{
    AsymmetricEncryptionScheme, AsymmetricDecryptor, AsymmetricEncryptor, KeyGenWithPrimeConfig, KeyGenerator, Signer,
    Verifier,
};
use crate::encryption::rsa::keys::{RsaKeyPair, RsaPrivateKey, RsaPublicKey};
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
use crate::shared::errors::ArithmeticError;
use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::One;
use log::{debug, trace};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};

pub struct RsaScheme {}

impl EncryptionScheme for RsaScheme {}

impl AsymmetricEncryptionScheme for RsaScheme {}

#[derive(Clone, Debug)]
pub struct RsaKeyGenConfig {
    pub key_size: u32,
    pub miller_rabin_iterations: u32,
    pub random_seed: u32,
    pub number_theory_service: NumberTheoryService,
}

impl KeyGenWithPrimeConfig for RsaKeyGenConfig {
    fn characteristic(&self) -> u32 {
        self.key_size
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

impl KeyGenerator<RsaPublicKey, RsaPrivateKey, RsaScheme> for RsaScheme {
    type KeyPair = RsaKeyPair;

    fn generate_keypair(config: &impl KeyGenWithPrimeConfig) -> Self::KeyPair {
        debug!(
            "Generiere Schlüsselpaar mit key_size {} und Miller-Rabin-Iterations {}",
            config.characteristic(),
            config.miller_rabin_iterations()
        );
        let random_generator =
            &PseudoRandomNumberGenerator::new(config.random_seed(), config.number_theory_service());

        let (prime_one, prime_two) = random_generator
            .get_distinct_primes(config.characteristic(), config.miller_rabin_iterations());

        let n = &prime_one * &prime_two;
        debug!("n ist {}", n);

        let phi = (&prime_one.decrement()) * (&prime_two.decrement());
        let e = Self::generate_e(&phi, random_generator, config.number_theory_service());
        let d = Self::generate_d(&e, &phi, config.number_theory_service()).unwrap(); // TODO: Abfangen später

        RsaKeyPair {
            public_key: RsaPublicKey { e, n: n.clone() },
            private_key: RsaPrivateKey { d, n },
        }
    }
}

impl Encryptor<RsaScheme> for RsaScheme {
    type Input = BigInt;
    type Output = BigInt;
    type Key = RsaPublicKey;
}

impl AsymmetricEncryptor<RsaScheme> for RsaScheme {
    fn encrypt(key: &Self::Key, plaintext: &Self::Input, service: NumberTheoryService) -> Self::Output {
        service.fast_exponentiation(plaintext, &key.e, &key.n)
    }
}

impl Decryptor<RsaScheme> for RsaScheme {
    type Input = BigInt;
    type Output = BigInt;
    type Key = RsaPrivateKey;
}

impl AsymmetricDecryptor<RsaScheme> for RsaScheme {
    fn decrypt(key: &Self::Key, ciphertext: &BigInt, service: NumberTheoryService) -> BigInt {
        service.fast_exponentiation(ciphertext, &key.d, &key.n)
    }
}

impl Signer<RsaScheme> for RsaScheme {
    type Input = BigInt;
    type Output = BigInt;
    type Key = RsaPrivateKey;

    fn sign(key: &Self::Key, message: &Self::Input, service: NumberTheoryService) -> Self::Output {
        service.fast_exponentiation(message, &key.d, &key.n)
    }
}

impl Verifier<RsaScheme> for RsaScheme {
    type Input = BigInt;
    type Output = bool;
    type Key = RsaPublicKey;

    fn verify(
        key: &Self::Key,
        message: &Self::Input,
        signature: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        let decrypted_signature = service.fast_exponentiation(signature, &key.e, &key.n);
        decrypted_signature == *message
    }
}

impl RsaScheme {
    /// Generiert eine Zahl `e` mit `1 < e < phi` und `ggT(e, phi) = 1`.
    ///
    /// # Argumente
    /// * `phi` - Die Zahl `phi`.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `e`.
    fn generate_e(
        phi: &BigInt,
        random_generator: &PseudoRandomNumberGenerator,
        service: NumberTheoryService,
    ) -> BigInt {
        debug!("Generiere e mit phi {}", phi);

        let n_counter = RelaxedCounter::new(1);

        let mut e = random_generator.take(&3.into(), &phi.decrement(), &n_counter);
        while e < *phi {
            let ggt = service.extended_euclid(&e, phi).ggt;
            if ggt.is_one() {
                debug!("Generierter e {} ist relativ prim zu phi {}", e, phi);
                return e;
            }
            trace!("Generierter e {} ist nicht relativ prim zu phi {}", e, phi);
            e.increment_assign();
        }
        panic!("Kein e gefunden, das relativ prim zu phi {} ist", phi);
    }

    /// Generiert eine Zahl `d` mit `1 < d < phi` und `e * d = 1 mod phi`.
    /// d ist damit das multiplikative Inverse von e mod phi.
    ///
    /// # Argumente
    /// * `e` - Die Zahl `e`.
    /// * `phi` - Die Zahl `phi`.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `d`.
    ///
    /// # Fehler
    /// * `ArithmeticError::NoInverseError` - Falls kein multiplikatives Inverses gefunden werden konnte.
    fn generate_d(
        e: &BigInt,
        phi: &BigInt,
        service: NumberTheoryService,
    ) -> Result<BigInt, ArithmeticError> {
        trace!("Generiere d mit e {} und phi {}", e, phi);
        let d = service.modulo_inverse(e, phi)?;
        debug!("d ist {}", d);
        Ok(d)
    }
}
