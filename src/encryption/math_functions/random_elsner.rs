use bigdecimal::num_bigint::{BigInt, BigUint, ToBigInt};
use bigdecimal::{BigDecimal, One, Zero};
use rand::random;

#[derive(Debug)]
pub struct RandomElsner {
    pub(crate) sqrt_m: BigDecimal,
    pub(crate) n: BigDecimal,
}

impl RandomElsner {

    ///
    /// Erstellt eine neue Instanz von RandomElsner.
    ///
    /// # Rückgabe
    /// * RandomElsner
    ///
    pub fn new() -> Self {
        let mut m = BigDecimal::from(random::<u128>());
        while (m.sqrt().unwrap() % BigDecimal::one()) == BigDecimal::zero() {
            m = BigDecimal::from(random::<u128>());
        }
        return Self {
            sqrt_m: m.sqrt().unwrap(),
            n: BigDecimal::zero(),
        };
    }

    ///
    /// Gibt eine zufällige Zahl im Bereich von a bis b zurück.
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    ///
    /// # Rückgabe
    /// * BigUint
    ///
    pub fn take(&mut self, a: &BigUint, b: &BigUint) -> BigUint {
        self.n += BigDecimal::one();
        let range = b - a + BigUint::one();
        let num =
            (&self.n * &self.sqrt_m) % BigDecimal::one() * BigDecimal::from(BigInt::from(range));
        return a + (BigDecimal::to_bigint(&num).unwrap()).to_biguint().unwrap();
    }
}
