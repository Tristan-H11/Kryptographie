use atomic_counter::RelaxedCounter;
use num::{BigInt, One, Zero};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::big_i;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryService;
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceTrait;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::math_functions::traits::parity::Parity;

/// Diese Struktur stellt Methoden zur Verfügung, um die Primzahleigenschaft eines
/// Integers zu testen.
pub struct PrimalityTest {
    pub number_theory_service: NumberTheoryService,
}

impl PrimalityTest {
    /// Erstellt eine neue Instanz des PrimalityTest.
    pub fn new(number_theory_service: NumberTheoryService) -> PrimalityTest {
        PrimalityTest {
            number_theory_service
        }
    }

    ///
    /// Primitive Prüfung auf eine zusammengesetzte Zahl.
    /// **Achtung: Funktioniert nur für Prim-Kandidaten größer 300**
    ///
    pub fn fails_primitive_prime_checks(p: &BigInt) -> bool {
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
    pub fn miller_rabin(
        &self,
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
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
                a = random_generator.take(&2.into(), &p, &n_counter);
            }
            self.miller_rabin_iteration(p, &s, &d, &a)
        })
    }

    /// Diese Methode führt eine Iteration des Miller-Rabin-Primzahltests für den angegebenen Integer durch.
    ///
    /// # Arguments
    /// * `p`: Der Integer, für den der Primzahltest durchgeführt werden soll.
    /// * `s`: Der Exponent 's' des Miller-Rabin-Tests.
    /// * `d`: Der Defekt des Integers 'p'.
    /// * `a`: Die Zufallszahl, die für den Test verwendet wird.
    ///
    /// # Rückgabe
    /// * `true`, wenn der Integer wahrscheinlich eine Primzahl ist, `false`, wenn nicht.
    fn miller_rabin_iteration(
        &self,
        p: &BigInt,
        s: &BigInt,
        d: &BigInt,
        a: &BigInt,
    ) -> bool {
        let mut x = self.number_theory_service.fast_exponentiation(a, d, p);

        if x.is_one() || x == p.decrement() {
            return true;
        }

        let mut r = BigInt::zero();

        while &r < s {
            x = self.number_theory_service.fast_exponentiation(&x, &big_i!(2), p);
            if x == p.decrement() {
                return true;
            }
            r.increment_assign();
        }
        return false;
    }
}

fn get_primes_to_300() -> [u32; 61] {
    return [
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
        197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
    ];
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_fast_with_small_p() {
        PrimalityTest::fails_primitive_prime_checks(&big_i!(11));
    }

    #[test]
    fn test_no_panic_with_big_p() {
        PrimalityTest::fails_primitive_prime_checks(&big_i!(1001));
    }
}
