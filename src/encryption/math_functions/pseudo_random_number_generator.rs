use atomic_counter::{AtomicCounter, RelaxedCounter};
use bigdecimal::num_bigint::{BigInt, ToBigInt};
use bigdecimal::{BigDecimal, One};
use log::trace;

use crate::big_d;
use crate::encryption::math_functions::traits::increment::Increment;

///
/// Iterator für eine deterministische Zufallszahlfolge.
///
#[derive(Clone)]
pub struct PseudoRandomNumberGenerator {
    sqrt_m: BigDecimal,
}

impl PseudoRandomNumberGenerator {
    ///
    /// Erstellt eine neue Instanz des PseudoRandomNumberGenerator.
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `random_seed` - Seed für die Zufallszahlfolge.
    ///
    /// # Rückgabe
    /// * PseudoRandomNumberGenerator
    ///
    pub fn new(random_seed: u32) -> Self {
        let mut initial_random = random_seed;
        let sqrt_m;
        loop {
            match big_d!(initial_random).sqrt() {
                Some(sqrt) => {
                    if !sqrt.is_integer() {
                        sqrt_m = sqrt;
                        break;
                    } else {
                        initial_random.increment_assign()
                    }
                }
                None => panic!("Wurzel m konnte nicht berechnet werden."),
            }
        }
        return Self { sqrt_m };
    }

    /// Diese Methode gibt eine Zufallszahl im Bereich von a bis b zurück.
    /// Inkrementiert den AtomicCounter n_counter!
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `n_counter` - Der AtomicCounter, der den Index des Elementes aus der Zufallsfolge enthält.
    ///
    /// # Rückgabe
    /// * BigUint
    pub fn take(&self, a: &BigInt, b: &BigInt, n_counter: &RelaxedCounter) -> BigInt {
        trace!(
            "Zufallszahl aus dem Bereich von {} bis {} mit n {}",
            a,
            b,
            n_counter.get()
        );
        let factor = (big_d!(n_counter.inc() as u64) * &self.sqrt_m) % BigDecimal::one();
        let range = big_d!(b - a + BigInt::one());

        // Das unwrap() wird niemals fehlschlagen, weil die Implementation von to_bigint() nur
        // Some, aber niemals None zurückgibt. Es ist unklar, warum es überhaupt Option ist.
        a + (factor * range).to_bigint().unwrap()
    }

    /// Diese Methode gibt eine ungerade Zufallszahl im Bereich von a bis b zurück.
    /// Inkrementiert den AtomicCounter n_counter!
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `n_counter` - Index des Elementes aus der Zufallsfolge.
    ///
    /// # Rückgabe
    /// * BigUint
    pub fn take_uneven(&self, a: &BigInt, b: &BigInt, n_counter: &RelaxedCounter) -> BigInt {
        self.take(a, b, n_counter) | BigInt::one()
    }
}

#[cfg(test)]
mod tests {
    use atomic_counter::RelaxedCounter;
    use bigdecimal::num_bigint::BigInt;

    use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
    use crate::encryption::math_functions::traits::divisible::Divisible;

    #[test]
    fn test_happy_flow() {
        let a: BigInt = 1u32.into();
        let b: BigInt = 997u32.into();

        let random = PseudoRandomNumberGenerator::new(13);

        let n = RelaxedCounter::new(1);

        assert_eq!(random.take(&a, &b, &n), 604u32.into());
        assert_eq!(random.take(&a, &b, &n), 211u32.into());
        assert_eq!(random.take(&a, &b, &n), 815u32.into());
        assert_eq!(random.take(&a, &b, &n), 421u32.into());
        assert_eq!(random.take(&a, &b, &n), 28u32.into());
        assert_eq!(random.take(&a, &b, &n), 632u32.into());
        assert_eq!(random.take(&a, &b, &n), 239u32.into());
        assert_eq!(random.take(&a, &b, &n), 842u32.into());
        assert_eq!(random.take(&a, &b, &n), 449u32.into());
        assert_eq!(random.take(&a, &b, &n), 56u32.into());

        let a: BigInt = 500u32.into();
        let b: BigInt = 6000u32.into();

        let random = PseudoRandomNumberGenerator::new(40);

        for _ in 1..500 {
            let random = random.take(&a, &b, &n);
            assert!(random >= a && random <= b);
        }
    }

    #[test]
    fn test_take_uneven() {
        let a: BigInt = 500u32.into();
        let b: BigInt = 6000u32.into();

        let random = PseudoRandomNumberGenerator::new(23);

        let n = RelaxedCounter::new(1);

        for _ in 1..500 {
            let random = random.take_uneven(&a, &b, &n);
            assert!(random >= a && random <= b);
            assert!(random.is_not_divisible_by(&BigInt::from(2)));
        }
    }
}
