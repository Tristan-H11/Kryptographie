use bigdecimal::num_bigint::BigInt;
use bigdecimal::One;
use log::{debug, error, trace};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::big_i;
use crate::encryption::math_functions::number_theory::{
    extended_euclid, miller_rabin, modulo_inverse,
};
use crate::encryption::math_functions::random_elsner::RandomElsner;
use crate::encryption::math_functions::small_primes::get_primes_to_10_000;
use crate::encryption::math_functions::traits::divisible::Divisible;
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
    ///
    /// * `key_width` - Die Breite des Moduls `n`, mit welchem die Schlüssel berechnet werden.
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
    ///
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_seed`- Seed für die gleichverteilte Zufallszahlerzeugung.
    ///
    /// # Rückgabe
    ///
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    ///
    pub(crate) fn generate_keypair(
        &self,
        miller_rabin_iterations: u32,
        random_seed: &BigInt,
        g_base: &BigInt,
    ) -> (PublicKey, PrivateKey) {
        debug!(
            "Generiere Schlüsselpaar mit key_size {} und Miller-Rabin-Iterations {}",
            self.key_size, miller_rabin_iterations
        );
        let random_generator = &mut RandomElsner::new(random_seed);

        let (prime_one, prime_two) =
            self.get_distinct_primes(miller_rabin_iterations, random_generator);

        let n = &prime_one * &prime_two;
        debug!("n ist {}", n);

        let phi = (&prime_one - BigInt::one()) * (&prime_two - BigInt::one());
        let e = self.generate_e(&phi, random_generator);
        let d = self.generate_d(&e, &phi);
        let public_key = PublicKey::new(e, n.clone(), g_base);
        let private_key = PrivateKey::new(d, n, g_base);
        debug!("Schlüsselpaar generiert");
        (public_key, private_key)
    }

    ///
    /// Generiert zwei verschiedene Primzahlen mit der angegebenen Breite.
    ///
    fn get_distinct_primes(
        &self,
        miller_rabin_iterations: u32,
        random_generator: &mut RandomElsner,
    ) -> (BigInt, BigInt) {
        let prim_size = self.key_size / 2;

        let (prime_one, mut prime_two) = (
            //rayon::join( TODO Tristan: wieder einbauen
            self.generate_prime(prim_size, miller_rabin_iterations, random_generator),
            self.generate_prime(prim_size, miller_rabin_iterations, random_generator),
        );
        while prime_one == prime_two {
            trace!(
                "Generierter prime_one {} ist gleich prime_two {}. Starte neuen Versuch",
                prime_one,
                prime_two
            );
            prime_two = self.generate_prime(prim_size, miller_rabin_iterations, random_generator);
        }
        (prime_one, prime_two)
    }

    ///
    /// Generiert eine Primzahl mit der angegebenen Breite.
    ///
    /// # Argumente
    ///
    /// * `size` - Die Breite der Primzahl.
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `random_seed` - Der Seed für die gleichverteilte Zufallszahlerzeugung.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Primzahl.
    ///
    fn generate_prime(
        &self,
        size: u32,
        miller_rabin_iterations: u32,
        random_generator: &mut RandomElsner,
    ) -> BigInt {
        debug!(
            "Generiere eine Primzahl mit size {} und Miller-Rabin-Iterations {}",
            size, miller_rabin_iterations
        );

        let upper_bound = &big_i!(2).pow(size);
        let lower_bound = &big_i!(2).pow(size - 1);

        let mut prime_candidate = random_generator.take_uneven(lower_bound, upper_bound);

        loop {
            if prime_candidate < big_i!(10000) {
                error!("Die erzeugte Zufallszahl ist kleiner als 10'000, der Algorithmus wird fehlschlagen!");
                // TODO schön machen. Prüfung und entsprechend nur is_in_first_10000_primes aufrufen
            }
            if passes_primitive_prime_checks(&prime_candidate) {
                // Ist die Zahl laut primitiver Tests scheinbar keine zusammengesetzte,
                // so darf Miller-Rabin laufen
                if miller_rabin(&prime_candidate, miller_rabin_iterations, random_generator) {
                    // Gibt der Miller-Rabin `wahr` zurück, sind wir fertig
                    break;
                }
            }

            trace!(
                "Generierter Primkandidat {} ist keine Primzahl",
                prime_candidate
            );
            prime_candidate = random_generator.take_uneven(lower_bound, upper_bound);
        }

        debug!(
            "Generierter Primkandidat {} ist eine Primzahl",
            prime_candidate
        );
        prime_candidate
    }

    ///
    /// Generiert eine Zahl `e` mit `1 < e < phi` und `ggT(e, phi) = 1`.
    ///
    /// # Argumente
    /// * `phi` - Die Zahl `phi`.
    /// * `random_seed` - Seed für die gleichverteilte Zufallszahlerzeugung.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Zahl `e`.
    ///
    fn generate_e(&self, phi: &BigInt, random_generator: &mut RandomElsner) -> BigInt {
        debug!("Generiere e mit phi {}", phi);

        let fermat_four = big_i!(65537);
        let mut e;
        if &phi.decrement() < &fermat_four {
            e = random_generator.take(&big_i!(3u8), &phi.decrement());
        } else {
            e = fermat_four
        };
        while e < *phi {
            let euclid = &extended_euclid(&e, &phi).0;
            if euclid.is_one() {
                debug!("Generierter e {} ist relativ prim zu phi {}", e, phi);
                return e;
            }
            trace!("Generierter e {} ist nicht relativ prim zu phi {}", e, phi);
            e += BigInt::one();
        }
        panic!("Kein e gefunden, das relativ prim zu phi {} ist", phi);
    }

    ///
    /// Generiert eine Zahl `d` mit `1 < d < phi` und `e * d = 1 mod phi`.
    /// d ist damit das multiplikative Inverse von e mod phi.
    ///
    /// # Argumente
    ///
    /// * `e` - Die Zahl `e`.
    /// * `phi` - Die Zahl `phi`.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Zahl `d`.
    ///
    fn generate_d(&self, e: &BigInt, phi: &BigInt) -> BigInt {
        trace!("Generiere d mit e {} und phi {}", e, phi);
        let d = match modulo_inverse(e, phi) {
            Ok(d) => d,
            Err(_) => panic!("Kein d gefunden, das e * d = 1 mod phi erfüllt"),
        };
        debug!("d ist {}", d);
        d
    }
}

///
/// Primitive Prüfung auf übliche Composite-Zahlen einer Nicht-Null und ungeraden Eingabezahl
/// **Achtung: Funktioniert nur für Prime-Kandidaten größer 10'000**
///
fn passes_primitive_prime_checks(prime_candidate: &BigInt) -> bool {
    let small_primes = get_primes_to_10_000();

    let prime_division_test = small_primes.into_par_iter().all(|prime| {
        prime_candidate.is_not_divisible_by(&big_i!(prime))
    });
    //TODO Man könnte noch den fermatschen Primzahltest einbauen. Obs das aber schneller macht..?
    prime_division_test
}
