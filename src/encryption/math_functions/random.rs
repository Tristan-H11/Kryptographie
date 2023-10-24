use bigdecimal::num_bigint::{BigInt, BigUint, ToBigInt};
use bigdecimal::{BigDecimal, One, Zero};
use rand::random;

#[derive(Debug)]
pub struct Random {
    sqrt_m: BigDecimal,
    n: BigDecimal,
}

impl Random {
    pub fn create() -> Self {
        let mut m = BigDecimal::from(random::<u128>());
        while m.sqrt().unwrap() == BigDecimal::zero() {
            m = BigDecimal::from(random::<u128>());
        }
        return Self {
            sqrt_m: m.sqrt().unwrap(),
            n: BigDecimal::zero(),
        };
    }

    pub fn take(&mut self, a: &BigUint, b: &BigUint) -> BigUint {
        self.n += BigDecimal::one();
        let range = b - a + BigUint::one();
        let num =
            (&self.n * &self.sqrt_m) % BigDecimal::one() * BigDecimal::from(BigInt::from(range));
        return a + (BigDecimal::to_bigint(&num).unwrap()).to_biguint().unwrap();
    }
}
