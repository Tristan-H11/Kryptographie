use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::One;
use log::{debug, trace};

use crate::big_i;
use crate::encryption::math_functions::number_theory::extended_euclid::ExtendedEuclid;
use crate::encryption::math_functions::number_theory::modulo_inverse::ModuloInverse;
use crate::encryption::math_functions::number_theory::primality_test::PrimalityTest;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::rsa::keys::{PrivateKey, PublicKey};

///
/// Ein Service zum Generieren von Schlüsselpaaren für RSA.
///
pub struct RsaKeygenService {
    key_size: u32,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    /// * `key_size` - Die Breite des Moduls `n`, mit welchem die Schlüssel berechnet werden.
    ///
    pub fn new(key_size: u32) -> RsaKeygenService {
        debug!(
            "Erstellen eines neuen RsaKeygenService mit key_size {}",
            key_size
        );
        RsaKeygenService { key_size }
    }

    ///
    /// Generiert ein Schlüsselpaar für RSA.
    ///
    /// # Argumente
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_seed` - Der Seed für die gleichverteilte Zufallszahlerzeugung.
    /// * `g_base` - Die Basis des Zeichensatzes, der für die Verschlüsselung verwendet wird.
    /// * `use_fast` - Gibt an, ob die schnellen Varianten der Algorithmen verwendet werden sollen.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    pub(crate) fn generate_keypair(
        &self,
        miller_rabin_iterations: u32,
        random_seed: u32,
        g_base: u32,
        use_fast: bool,
    ) -> (PublicKey, PrivateKey) {
        debug!(
            "Generiere Schlüsselpaar mit key_size {} und Miller-Rabin-Iterations {}",
            self.key_size, miller_rabin_iterations
        );
        let random_generator = &PseudoRandomNumberGenerator::new(random_seed);

        let (prime_one, prime_two) =
            self.get_distinct_primes(miller_rabin_iterations, random_generator, use_fast);

        let n = &prime_one * &prime_two;
        debug!("n ist {}", n);

        let phi = (&prime_one - BigInt::one()) * (&prime_two - BigInt::one());
        let e = self.generate_e(&phi, random_generator, use_fast);
        let d = self.generate_d(&e, &phi, use_fast);
        let public_key = PublicKey::new(e, n.clone(), g_base);
        let private_key = PrivateKey::new(d, n, g_base);
        debug!("Schlüsselpaar generiert");
        (public_key, private_key)
    }

    /// Generiert zwei verschiedene Primzahlen mit der angegebenen Breite.
    ///
    /// # Argumente
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    /// * `use_fast` - Gibt an, ob die schnellen Varianten der Algorithmen verwendet werden sollen.
    fn get_distinct_primes(
        &self,
        miller_rabin_iterations: u32,
        random_generator: &PseudoRandomNumberGenerator,
        use_fast: bool,
    ) -> (BigInt, BigInt) {
        let prim_size = self.key_size / 2;
        let n_counter = RelaxedCounter::new(1);
        let prime_one = self.generate_prime(
            prim_size,
            miller_rabin_iterations,
            random_generator,
            &n_counter,
            use_fast,
        );
        // Vor diesem Aufruf ist n_counter schon von generate_prime inkrementiert worden.
        let mut prime_two = self.generate_prime(
            prim_size,
            miller_rabin_iterations,
            random_generator,
            &n_counter,
            use_fast,
        );

        while prime_one == prime_two {
            trace!(
                "Generierter prime_one {} ist gleich prime_two {}. Starte neuen Versuch",
                prime_one,
                prime_two
            );
            prime_two = self.generate_prime(
                prim_size,
                miller_rabin_iterations,
                random_generator,
                &n_counter,
                use_fast,
            );
        }
        (prime_one, prime_two)
    }

    /// Generiert eine Primzahl mit der angegebenen Breite.
    ///
    /// # Argumente
    /// * `size` - Die Breite der Primzahl.
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    /// * `index_for_random_generator` - Der Index der Zufallszahl, welche der Folge entnommen werden soll.
    /// * `use_fast` - Gibt an, ob die schnellen Varianten der Algorithmen verwendet werden sollen.
    ///
    /// # Rückgabe
    /// Die generierte Primzahl.
    fn generate_prime(
        &self,
        size: u32,
        miller_rabin_iterations: u32,
        random_generator: &PseudoRandomNumberGenerator,
        n_counter: &RelaxedCounter,
        use_fast: bool,
    ) -> BigInt {
        debug!(
            "Generiere eine Primzahl mit size {} und Miller-Rabin-Iterations {}",
            size, miller_rabin_iterations
        );

        let upper_bound = &big_i!(2).pow(size);
        let lower_bound = &big_i!(2).pow(size - 1);

        let mut prime_candidate = random_generator.take_uneven(lower_bound, upper_bound, n_counter);

        while !PrimalityTest::calculate(
            &prime_candidate,
            miller_rabin_iterations,
            random_generator,
            use_fast,
        ) {
            trace!(
                "Generierter Primkandidat {} ist keine Primzahl",
                prime_candidate
            );
            prime_candidate = random_generator.take_uneven(lower_bound, upper_bound, n_counter);
        }
        debug!(
            "Generierter Primkandidat {} ist eine Primzahl",
            prime_candidate
        );
        prime_candidate
    }

    /// Generiert eine Zahl `e` mit `1 < e < phi` und `ggT(e, phi) = 1`.
    ///
    /// # Argumente
    /// * `phi` - Die Zahl `phi`.
    /// * `random_generator` - Der Pseudo-Zufallszahlengenerator.
    /// * `use_fast` - Gibt an, ob die schnellen Varianten der Algorithmen verwendet werden sollen.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `e`.
    fn generate_e(
        &self,
        phi: &BigInt,
        random_generator: &PseudoRandomNumberGenerator,
        use_fast: bool,
    ) -> BigInt {
        debug!("Generiere e mit phi {}", phi);

        let n_counter = RelaxedCounter::new(1);

        let mut e = random_generator.take(&big_i!(3u8), &phi.decrement(), &n_counter);
        while e < *phi {
            let ggt = ExtendedEuclid::calculate(&e, phi, use_fast).0;
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
    /// * `use_fast` - Gibt an, ob die schnellen Varianten der Algorithmen verwendet werden sollen.
    ///
    /// # Rückgabe
    /// Die generierte Zahl `d`.
    fn generate_d(&self, e: &BigInt, phi: &BigInt, use_fast: bool) -> BigInt {
        trace!("Generiere d mit e {} und phi {}", e, phi);
        let d = match ModuloInverse::calculate(e, phi, use_fast) {
            Ok(d) => d,
            Err(_) => panic!("Kein d gefunden, das e * d = 1 mod phi erfüllt"),
        };
        debug!("d ist {}", d);
        d
    }
}
