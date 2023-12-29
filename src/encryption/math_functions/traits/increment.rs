use bigdecimal::One;
use std::ops::{Add, Sub};

///
/// Trait für die Berechnung von imkrementellen und dekrementellen Werten.
///
pub trait Increment {
    ///
    /// Gibt das Inkrement des Wertes zurück.
    ///
    fn increment(&self) -> Self;

    ///
    /// Erhöht den Wert um 1.
    ///
    fn increment_assign(&mut self);

    ///
    /// Gibt das Dekrement des Wertes zurück.
    ///
    fn decrement(&self) -> Self;

    ///
    /// Verringert den Wert um 1.
    ///
    fn decrement_assign(&mut self);
}

impl<T: One + Clone + Add<Output = T> + Sub<Output = T>> Increment for T {
    fn increment(&self) -> Self {
        self.clone() + T::one()
    }

    fn increment_assign(&mut self) {
        *self = self.increment();
    }

    fn decrement(&self) -> Self {
        self.clone() - T::one()
    }

    fn decrement_assign(&mut self) {
        *self = self.decrement()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::num_bigint::BigInt;

    #[test]
    fn test_increment() {
        assert_eq!(BigInt::from(3).increment(), BigInt::from(4));
        assert_eq!(BigInt::from(0).increment(), BigInt::from(1));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(BigInt::from(3).decrement(), BigInt::from(2));
        assert_eq!(BigInt::from(1).decrement(), BigInt::from(0));
    }

    #[test]
    fn test_increment_assign() {
        let mut uint = BigInt::from(3);
        uint.increment_assign();
        assert_eq!(uint, BigInt::from(4));
    }

    #[test]
    fn test_decrement_assign() {
        let mut uint = BigInt::from(3);
        uint.decrement_assign();
        assert_eq!(uint, BigInt::from(2));
    }
}
