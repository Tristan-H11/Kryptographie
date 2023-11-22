use std::sync::{Arc, Mutex};
use num::{BigInt, One, Zero};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::big_i;
use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::number_theory::small_primes::get_primes_to_500;
use crate::encryption::math_functions::random_elsner::RandomElsner;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::math_functions::traits::parity::Parity;
use crate::encryption::math_functions::traits::rapid_math_ops::RapidMathOps;

///
/// Führt einen Primzahltest auf Basis des Miller-Rabin-Tests durch.
///
/// # Argumente
/// * `p` - Die zu testende Zahl >= 11.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
/// * `random_generator` - Generator für gleichverteilte Zufallszahlen.
///
pub struct PrimalityTest {
    p: BigInt,
    repeats: u32,
    random_generator: RandomElsner,
}

impl RapidMathOps<bool> for PrimalityTest {
    fn fast(&self) -> bool {
        // Enthält noch einige weitere Tests, die für slow nicht vorgesehen sind.
        if self.fails_primitive_prime_checks() {
            return false;
        }
        // Sind die primitiven Tests bestanden, läuft miller_rabin an.
        self.miller_rabin(&self.p, self.repeats, &self.random_generator, true)
    }

    fn own(&self) -> bool {
        self.miller_rabin(&self.p, self.repeats, &self.random_generator, false)
    }
}

impl PrimalityTest {
    /// Erstellt eine neue Instanz von PrimalityTest.
    pub fn new(p: BigInt, repeats: u32, random_generator: RandomElsner) -> Self {
        PrimalityTest {
            p,
            repeats,
            random_generator,
        }
    }

    ///
    /// Primitive Prüfung auf eine zusammengesetzte Zahl.
    /// **Achtung: Funktioniert nur für Prime-Kandidaten größer 500**
    ///
    fn fails_primitive_prime_checks(&self) -> bool {
        // Gerade Zahlen sind nie prim.
        if self.p.is_even() {
            return true;
        }

        let small_primes = get_primes_to_500();

        let prime_division_test = small_primes.into_par_iter().any(|prime| {
            self.p.is_divisible_by(&big_i!(prime))
        });
        //TODO Man könnte noch den fermatschen Primzahltest einbauen. Obs das aber schneller macht..?
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
    fn miller_rabin(&self, p: &BigInt, repeats: u32, random_generator: &RandomElsner, use_fast: bool) -> bool {
        let mut d = p.decrement();
        let mut s = BigInt::zero();

        while d.is_even() {
            d.half_assign();
            s.increment_assign();
        }

        let n_arc = Arc::new(Mutex::new(1u128));

        (0..repeats).into_par_iter().all(|_| {
            // TODO Tristan: Hübsch machen
            let n = {
                let mut n = n_arc.lock().unwrap();
                n.increment_assign();
                &mut n.clone()
            };
            let mut a = random_generator.take(&big_i!(2), &p, n);
            while p.is_divisible_by(&a) {
                a = random_generator.take(&big_i!(2), &p, n);
            }
            self.miller_rabin_test(p, &s, &d, &a, use_fast)
        })
    }

    ///
    /// Führt eine Iteration des Miller-Rabin-Tests aus. Gibt zurück, ob die Zahl vermutlich
    /// eine Primzahl ist.
    ///
    fn miller_rabin_test(p: &BigInt, s: &BigInt, d: &BigInt, a: &BigInt, use_fast: bool) -> bool {
        let mut fast_exp = FastExponentiation::new(a.clone(), d.clone(), p.clone());
        let mut x = fast_exp.calculate(use_fast);

        if x.is_one() || x == p.decrement() {
            return true;
        }

        let mut r = BigInt::zero();

        while &r < s {
            fast_exp.set_base(x);
            x = fast_exp.calculate(use_fast);
            if x == p.decrement() {
                return true;
            }
            r.increment_assign();
        }
        return false;
    }
}