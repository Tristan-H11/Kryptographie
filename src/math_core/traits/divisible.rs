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
    use bigdecimal::num_bigint::BigInt;
    use bigdecimal::One;

    use super::*;

    #[test]
    fn test_divides() {
        assert_eq!(BigInt::from(8).is_divisible_by(&4.into()), true);
        assert_eq!(BigInt::from(89893457).is_divisible_by(&1.into()), true);
        assert_eq!(
            BigInt::from(89893457).is_divisible_by(&657831.into()),
            false
        );
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(BigInt::from(8).is_not_divisible_by(&4.into()), false);
        assert_eq!(BigInt::from(89893457).is_not_divisible_by(&1.into()), false);
        assert_eq!(
            BigInt::from(89893457).is_not_divisible_by(&657831.into()),
            true
        );
    }

    #[test]
    fn test_half() {
        assert_eq!(BigInt::from(8).half(), 4.into());
        assert_eq!(BigInt::one().half(), 0.into());
    }

    #[test]
    fn test_half_assign() {
        let mut x: BigInt = 8.into();
        x.half_assign();
        assert_eq!(x, 4.into());

        let mut y: BigInt = 1.into();
        y.half_assign();
        assert_eq!(y, 0.into());
    }

    #[test]
    fn test_double() {
        assert_eq!(BigInt::from(8).double(), 16.into());
        assert_eq!(BigInt::one().double(), 2.into());
    }

    #[test]
    fn test_double_assign() {
        let mut x: BigInt = 8.into();
        x.double_assign();
        assert_eq!(x, 16.into());

        let mut y: BigInt = 1.into();
        y.double_assign();
        assert_eq!(y, 2.into());
    }
}
