use bigdecimal::num_bigint::{BigInt, ToBigInt};
use bigdecimal::{BigDecimal, One, Zero};

use crate::big_d;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;

///
/// Iterator für einen Zufallswert nach dem Schema aus dem Skript.
///
#[derive(Clone)]
pub struct RandomElsner {
    sqrt_m: BigDecimal,
    n: BigInt,
    a: BigInt,
    range: BigDecimal,
}

impl RandomElsner {
    ///
    /// Erstellt eine neue Instanz von RandomElsner.
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `random_seed` - Seed für die Zufallszahlen. Darf keine Quadratzahl sein
    ///
    /// # Rückgabe
    /// * RandomElsner
    ///
    pub fn new(a: &BigInt, b: &BigInt, random_seed: &BigInt) -> Self {
        let sqrt_m;
        loop {
            match big_d!(random_seed.clone()).sqrt() {
                Some(sqrt) => {
                    if sqrt.is_not_divisible_by(&BigDecimal::one()) {
                        sqrt_m = sqrt;
                        break;
                    } else {
                        panic!("Random_seed darf keine Quadratzahl sein!") // TODO Anständig machen.
                    }
                }
                None => panic!("Wurzel m konnte nicht berechnet werden."),
            }
        }
        return Self {
            sqrt_m,
            n: BigInt::zero(),
            a: a.clone(),
            range: big_d!(BigInt::from(b - a + BigInt::one())),
        };
    }


    ///
    /// Gibt eine zufällige Zahl im Bereich von a bis b zurück.
    ///
    /// # Rückgabe
    /// * BigUint
    ///
    pub fn take(&mut self) -> BigInt {
        self.n.increment_assign();

        let factor = (&self.n * &self.sqrt_m) % BigDecimal::one();
        // Das unwrap() wird niemals fehlschlagen, weil die Implementation von to_bigint() nur
        // Some, aber niemals None zurückgibt. Es ist unklar, warum es überhaupt Option ist.
        let step = (factor * &self.range).to_bigint().unwrap();
        &self.a + step
    }

    ///
    /// Gibt eine zufällige ungerade Zahl im Bereich von a bis b zurück.
    ///
    /// # Rückgabe
    /// * BigUint
    pub fn take_uneven(&mut self) -> BigInt {
        self.take() | BigInt::one()
    }
}
