use atomic_counter::{AtomicCounter, RelaxedCounter};

use num::{BigInt, One, Zero};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::big_i;
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::number_theory::small_primes::get_primes_to_300;
use crate::encryption::math_functions::random_elsner::RandomElsner;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::math_functions::traits::parity::Parity;

///
/// Führt einen Primzahltest auf Basis des Miller-Rabin-Tests durch.
///
/// # Argumente
/// * `p` - Die zu testende Zahl >= 11.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
/// * `random_generator` - Generator für gleichverteilte Zufallszahlen.
///
pub struct PrimalityTest {}

impl PrimalityTest {
    /// Prüft, ob `p` wahrscheinlich eine Primzahl ist.
    pub fn calculate(
        p: &BigInt,
        repeats: u32,
        random_generator: &RandomElsner,
        use_fast: bool,
    ) -> bool {
        return if use_fast {
            PrimalityTest::fast(p, repeats, random_generator)
        } else {
            PrimalityTest::own(p, repeats, random_generator)
        };
    }

    fn fast(p: &BigInt, repeats: u32, random_generator: &RandomElsner) -> bool {
        // Enthält noch einige weitere Tests, die für slow nicht vorgesehen sind.
        if PrimalityTest::fails_primitive_prime_checks(p) {
            return false;
        }
        // Sind die primitiven Tests bestanden, läuft miller_rabin an.
        PrimalityTest::miller_rabin(p, repeats, random_generator, true)
    }

    fn own(p: &BigInt, repeats: u32, random_generator: &RandomElsner) -> bool {
        PrimalityTest::miller_rabin(p, repeats, random_generator, false)
    }

    ///
    /// Primitive Prüfung auf eine zusammengesetzte Zahl.
    /// **Achtung: Funktioniert nur für Prime-Kandidaten größer 500**
    ///
    fn fails_primitive_prime_checks(p: &BigInt) -> bool {
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

    /// Führt den Miller-Rabin-Primzahltest auf `p` mit `repeats` Runden aus.
    ///
    /// # Argumente
    /// * `p` - Die zu testende Zahl >= 11.
    /// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
    /// * `random_generator` - Generator für gleichverteilte Zufallszahlen.
    ///
    /// # Rückgabe
    /// `true`, wenn `p` wahrscheinlich eine Primzahl ist, andernfalls `false`.
    ///
    /// Wahrscheinlichkeit: >= 1 - (1/4)^repeats
    ///
    /// # Beispiel
    /// ```
    /// miller_rabin(11, 40) // => true
    /// miller_rabin(2211, 40) // => false
    /// ```
    fn miller_rabin(
        p: &BigInt,
        repeats: u32,
        random_generator: &RandomElsner,
        use_fast: bool,
    ) -> bool {
        let mut d = p.decrement();
        let mut s = BigInt::zero();

        while d.is_even() {
            d.half_assign();
            s.increment_assign();
        }

        // Zähler für den Zugriff auf das Element der Zufallsfolge.
        let n_count = RelaxedCounter::new(0);

        (0..repeats).into_par_iter().all(|_| {
            let n = n_count.add(1);
            let mut a = random_generator.take(&big_i!(2), &p, n);
            while p.is_divisible_by(&a) {
                a = random_generator.take(&big_i!(2), &p, n);
            }
            PrimalityTest::miller_rabin_test(p, &s, &d, &a, use_fast)
        })
    }

    ///
    /// Führt eine Iteration des Miller-Rabin-Tests aus. Gibt zurück, ob die Zahl vermutlich
    /// eine Primzahl ist.
    ///
    fn miller_rabin_test(p: &BigInt, s: &BigInt, d: &BigInt, a: &BigInt, use_fast: bool) -> bool {
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
