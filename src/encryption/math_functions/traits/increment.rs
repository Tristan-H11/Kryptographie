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
    use crate::big_u;
    use num::BigUint;

    #[test]
    fn test_increment() {
        assert_eq!(big_u!(3u32).increment(), big_u!(4u32));
        assert_eq!(big_u!(0u32).increment(), big_u!(1u32));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(big_u!(3u32).decrement(), big_u!(2u32));
        assert_eq!(big_u!(1u32).decrement(), big_u!(0u32));
    }

    #[test]
    fn test_increment_assign() {
        let mut uint = big_u!(3u32);
        uint.increment_assign();
        assert_eq!(uint, big_u!(4u32));
    }

    #[test]
    fn test_decrement_assign() {
        let mut uint = big_u!(3u32);
        uint.decrement_assign();
        assert_eq!(uint, big_u!(2u32));
    }
}
