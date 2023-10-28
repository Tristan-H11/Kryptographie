use bigdecimal::Zero;
use std::ops::Rem;

pub trait Divisible {
    ///
    /// Gibt zurück, ob die Zahl durch eine andere teilbar ist.
    ///
    fn is_divisible_by(&self, divisor: &Self) -> bool;

    ///
    /// Gibt zurück, ob die Zahl nicht durch eine andere teilbar ist.
    ///
    fn is_not_divisible_by(&self, divisor: &Self) -> bool;
}

impl<T: Zero + Rem<Output = T> + PartialEq + Clone> Divisible for T {
    fn is_divisible_by(&self, divisor: &Self) -> bool {
        self.clone() % divisor.clone() == T::zero()
    }

    fn is_not_divisible_by(&self, divisor: &Self) -> bool {
        !self.is_divisible_by(divisor)
    }
}
