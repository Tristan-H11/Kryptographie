use bigdecimal::num_bigint::{BigInt, ToBigInt};
use bigdecimal::{BigDecimal, One, Zero};
use rand::random;

use crate::big_d;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;

///
/// Iterator für einen Zufallswert nach dem Schema aus dem Skript.
///
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
    ///
    /// # Rückgabe
    /// * RandomElsner
    ///
    pub fn new(a: &BigInt, b: &BigInt) -> Self {
        let sqrt_m;
        loop {
            let m = BigDecimal::from(random::<u128>());
            match m.sqrt() {
                Some(sqrt) => {
                    if sqrt.is_not_divisible_by(&BigDecimal::one()) {
                        sqrt_m = sqrt;
                        break;
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
    /// Konstruktor für Testfälle, um deterministische Werte zu erhalten.
    ///
    #[cfg(test)]
    pub fn new_deterministic(sqrt_m: BigDecimal, a: &BigInt, b: &BigInt) -> Self {
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
}
