use bigdecimal::Zero;
use std::ops::{Div, Mul, Rem};

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

impl<
        T: Zero + Rem<Output = T> + PartialEq + Clone + Div<Output = T> + Mul<Output = T> + From<u8>,
    > Divisible for T
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
