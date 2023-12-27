use atomic_counter::RelaxedCounter;

use num::{BigInt, One, Zero};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::big_i;
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::number_theory::small_primes::get_primes_to_300;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::math_functions::traits::parity::Parity;

/// Diese Struktur stellt Methoden zur Verfügung, um die Primzahleigenschaft eines
/// Integers zu testen.
pub struct PrimalityTest {}

impl PrimalityTest {
    /// Diese Methode führt einen probabilistischen Primzahltest für den angegebenen Integer durch.
    ///
    /// # Argumente
    /// * `p`: Der Integer, für den der Primzahltest durchgeführt werden soll.
    /// * `repeats`: Die Anzahl der Wiederholungen des Tests.
    /// * `random_generator`: Ein Pseudozufallszahlengenerator, der für die Erzeugung
    ///   der Zufallszahlen verwendet wird.
    /// * `use_fast`: Gibt an, ob der schnelle Primzahltest verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `true`, wenn der Integer eine Primzahl ist, `false`, wenn nicht.
    pub fn calculate(
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
        use_fast: bool,
    ) -> bool {
        return if use_fast {
            PrimalityTest::fast(p, repeats, random_generator)
        } else {
            PrimalityTest::own(p, repeats, random_generator)
        };
    }

    fn fast(p: &BigInt, repeats: u32, random_generator: &PseudoRandomNumberGenerator) -> bool {
        // Enthält noch einige weitere Tests, die für slow nicht vorgesehen sind.
        if PrimalityTest::fails_primitive_prime_checks(p) {
            return false;
        }
        // Sind die primitiven Tests bestanden, läuft miller_rabin an.
        PrimalityTest::miller_rabin(p, repeats, random_generator, true)
    }

    fn own(p: &BigInt, repeats: u32, random_generator: &PseudoRandomNumberGenerator) -> bool {
        PrimalityTest::miller_rabin(p, repeats, random_generator, false)
    }

    ///
    /// Primitive Prüfung auf eine zusammengesetzte Zahl.
    /// **Achtung: Funktioniert nur für Prim-Kandidaten größer 300**
    ///
    fn fails_primitive_prime_checks(p: &BigInt) -> bool {
        if p < &big_i!(300) {
            panic!("Primitive Primzahltests nur für p > 300 implementiert.");
        }
        // Gerade Zahlen sind nie prim.
        if p.is_even() {
            return true;
        }

        let small_primes = get_primes_to_300();

        let prime_division_test = small_primes
            .into_par_iter()
            .any(|prime| p.is_divisible_by(&big_i!(prime)));
        prime_division_test
    }

    /// Diese Methode führt einen Miller-Rabin-Primzahltest für den angegebenen Integer durch.
    ///
    /// # Arguments
    /// * `p`: Der Integer, für den der Primzahltest durchgeführt werden soll.
    /// * `repeats`: Die Anzahl der Wiederholungen des Tests.
    /// * `random_generator`: Ein Pseudozufallszahlengenerator, der für die Erzeugung
    ///   der Zufallszahlen verwendet wird.
    /// * `use_fast`: Gibt an, ob der schnelle Miller-Rabin-Test verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `true`, wenn der Integer wahrscheinlich eine Primzahl ist, `false`, wenn nicht.
    fn miller_rabin(
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
        use_fast: bool,
    ) -> bool {
        let mut d = p.decrement();
        let mut s = BigInt::zero();

        while d.is_even() {
            d.half_assign();
            s.increment_assign();
        }

        // Zähler für den Zugriff auf das Element der Zufallsfolge.
        let n_counter = RelaxedCounter::new(0);

        (0..repeats).into_par_iter().all(|_| {
            let mut a = random_generator.take(&big_i!(2), &p, &n_counter);
            while p.is_divisible_by(&a) {
                a = random_generator.take(&big_i!(2), &p, &n_counter);
            }
            PrimalityTest::miller_rabin_iteration(p, &s, &d, &a, use_fast)
        })
    }

    /// Diese Methode führt eine Iteration des Miller-Rabin-Primzahltests für den angegebenen Integer durch.
    ///
    /// # Arguments
    /// * `p`: Der Integer, für den der Primzahltest durchgeführt werden soll.
    /// * `s`: Der Exponent 's' des Miller-Rabin-Tests.
    /// * `d`: Der Defekt des Integers 'p'.
    /// * `a`: Die Zufallszahl, die für den Test verwendet wird.
    /// * `use_fast`: Gibt an, ob der schnelle Miller-Rabin-Test verwendet werden soll.
    ///
    /// # Rückgabe
    /// * `true`, wenn der Integer wahrscheinlich eine Primzahl ist, `false`, wenn nicht.
    fn miller_rabin_iteration(
        p: &BigInt,
        s: &BigInt,
        d: &BigInt,
        a: &BigInt,
        use_fast: bool,
    ) -> bool {
        let mut x = FastExponentiation::calculate(a, d, p, use_fast);

        if x.is_one() || x == p.decrement() {
            return true;
        }

        let mut r = BigInt::zero();

        while &r < s {
            x = FastExponentiation::calculate(&x, &big_i!(2), p, use_fast);
            if x == p.decrement() {
                return true;
            }
            r.increment_assign();
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigInt;
    use std::str::FromStr;

    #[test]
    fn miller_rabin_test_own() {
        let random_generator: &PseudoRandomNumberGenerator = &PseudoRandomNumberGenerator::new(11);
        assert_eq!(
            PrimalityTest::calculate(&big_i!(11), 100, random_generator, false),
            true
        );
        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3884010174220797539108782582068795892283779").unwrap(),
                40,
                random_generator,
                false,
            ),
            false
        );

        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3061046931436983206004510256116356531107241").unwrap(),
                40,
                random_generator,
                false
            ),
            false
        );

        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3348205994756289303286119224981125339947473").unwrap(),
                40,
                random_generator,
                false
            ),
            false
        );
        assert_eq!(
            PrimalityTest::calculate(&big_i!(2211), 40, random_generator, false),
            false
        );
        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap(),
                400,
                random_generator,
                false
            ),
            true
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_fast_with_small_p() {
        let random_generator: &PseudoRandomNumberGenerator = &PseudoRandomNumberGenerator::new(11);
        PrimalityTest::calculate(&big_i!(11), 100, random_generator, true);
    }

    #[test]
    fn test_no_panic_own_with_small_p() {
        let random_generator: &PseudoRandomNumberGenerator = &PseudoRandomNumberGenerator::new(11);
        PrimalityTest::calculate(&big_i!(11), 100, random_generator, false);
    }

    #[test]
    fn miller_rabin_test_fast() {
        let random_generator: &PseudoRandomNumberGenerator = &PseudoRandomNumberGenerator::new(11);

        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3884010174220797539108782582068795892283779").unwrap(),
                40,
                random_generator,
                true,
            ),
            false
        );

        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3061046931436983206004510256116356531107241").unwrap(),
                40,
                random_generator,
                true
            ),
            false
        );

        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("3348205994756289303286119224981125339947473").unwrap(),
                40,
                random_generator,
                true
            ),
            false
        );
        assert_eq!(
            PrimalityTest::calculate(&big_i!(2211), 40, random_generator, false),
            false
        );
        assert_eq!(
            PrimalityTest::calculate(
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap(),
                400,
                random_generator,
                true
            ),
            true
        );
    }
}
