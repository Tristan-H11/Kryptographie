use bigdecimal::{BigDecimal, One, Zero};
use bigdecimal::num_bigint::{BigInt, BigUint, ToBigInt};
use rand::random;
use crate::encryption::math_functions::traits::divisible::Divisible;

///
/// Iterator für einen Zufallswert nach dem Schema aus dem Skript.
///
pub struct RandomElsner {
    sqrt_m: BigDecimal,
    n: BigDecimal,
    a: BigUint,
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
    pub fn new(a: &BigUint, b: &BigUint) -> Self {
        let sqrt_m;
        loop {
            let m = BigDecimal::from(random::<u128>());
            match m.sqrt() {
                Some(sqrt) => {
                    if sqrt.is_not_divisible_by(&BigDecimal::one()){
                        sqrt_m = sqrt;
                        break;
                    }
                },
                None => panic!("Wurzel m konnte nicht berechnet werden."),
            }
        }
        return Self {
            sqrt_m,
            n: BigDecimal::zero(),
            a: a.clone(),
            range: BigDecimal::from(BigInt::from(b - a + BigUint::one())),
        };
    }

    ///
    /// Konstruktor für Testfälle, um deterministische Werte zu erhalten.
    ///
    #[cfg(test)]
    pub fn new_deterministic(sqrt_m: BigDecimal, a: &BigUint, b: &BigUint) -> Self {
        return Self {
            sqrt_m,
            n: BigDecimal::zero(),
            a: a.clone(),
            range: BigDecimal::from(BigInt::from(b - a + BigUint::one())),
        };
    }

    ///
    /// Gibt eine zufällige Zahl im Bereich von a bis b zurück.
    ///
    /// # Rückgabe
    /// * BigUint
    ///
    pub fn take(&mut self) -> BigUint {
        self.n += BigDecimal::one();
        let num = (((&self.n * &self.sqrt_m) % BigDecimal::one()) * &self.range).with_scale(0);
        return &self.a + (BigDecimal::to_bigint(&num).unwrap()).to_biguint().unwrap();
    }
}
