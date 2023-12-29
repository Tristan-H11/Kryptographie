use std::ops::{Div, Mul, Rem};

use bigdecimal::Zero;

pub trait Divisible {
    ///
    /// Gibt zurück, ob die Zahl durch eine andere teilbar ist.
    ///
    fn is_divisible_by(&self, divisor: &Self) -> bool;

    ///
    /// Gibt zurück, ob die Zahl nicht durch eine andere teilbar ist.
    ///
    fn is_not_divisible_by(&self, divisor: &Self) -> bool;

    ///
    /// Gibt die Hälfte des Wertes zurück.
    /// Achtung! 1.half() => 0
    ///
    fn half(&self) -> Self;

    ///
    /// Setzt den Wert auf die Hälfte des ursprünglichen Wertes.
    /// Achtung! 1.half() => 0
    ///
    fn half_assign(&mut self);

    ///
    /// Gibt das Doppelte des Wertes zurück.
    ///
    fn double(&self) -> Self;

    ///
    /// Setzt den Wert auf das Doppelte des ursprünglichen Wertes.
    ///
    fn double_assign(&mut self);
}

impl<T> Divisible for T
where
    T: Zero + Rem<Output = T> + PartialEq + Clone + Div<Output = T> + Mul<Output = T> + From<u8>,
{
    fn is_divisible_by(&self, divisor: &Self) -> bool {
        self.clone() % divisor.clone() == T::zero()
    }

    fn is_not_divisible_by(&self, divisor: &Self) -> bool {
        !self.is_divisible_by(divisor)
    }

    fn half(&self) -> Self {
        self.clone().div(T::from(2u8))
    }

    fn half_assign(&mut self) {
        *self = self.half();
    }

    fn double(&self) -> Self {
        self.clone() * T::from(2u8)
    }

    fn double_assign(&mut self) {
        *self = self.double();
    }
}

#[cfg(test)]
mod tests {
    use num::BigUint;

    use crate::big_u;

    use super::*;

    #[test]
    fn test_divides() {
        assert_eq!(big_u!(8u32).is_divisible_by(&big_u!(4u32)), true);
        assert_eq!(big_u!(89893457u32).is_divisible_by(&big_u!(1u32)), true);
        assert_eq!(
            big_u!(89893457u32).is_divisible_by(&big_u!(657831u32)),
            false
        );
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(big_u!(8u32).is_not_divisible_by(&big_u!(4u32)), false);
        assert_eq!(
            big_u!(89893457u32).is_not_divisible_by(&big_u!(1u32)),
            false
        );
        assert_eq!(
            big_u!(89893457u32).is_not_divisible_by(&big_u!(657831u32)),
            true
        );
    }

    #[test]
    fn test_half() {
        assert_eq!(big_u!(8u32).half(), big_u!(4u32));
        assert_eq!(big_u!(1u32).half(), big_u!(0u32));
    }

    #[test]
    fn test_half_assign() {
        let mut uint = big_u!(8u32);
        uint.half_assign();
        assert_eq!(uint, big_u!(4u32));

        let mut uint = big_u!(1u32);
        uint.half_assign();
        assert_eq!(uint, big_u!(0u32));
    }

    #[test]
    fn test_double() {
        assert_eq!(big_u!(8u32).double(), big_u!(16u32));
        assert_eq!(big_u!(1u32).double(), big_u!(2u32));
    }

    #[test]
    fn test_double_assign() {
        let mut uint = big_u!(8u32);
        uint.double_assign();
        assert_eq!(uint, big_u!(16u32));

        let mut uint = big_u!(1u32);
        uint.double_assign();
        assert_eq!(uint, big_u!(2u32));
    }
}
