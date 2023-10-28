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
    /// Gibt das Dekrement des Wertes zurück.
    ///
    fn decrement(&self) -> Self;
}

impl<T: One + Clone + Add<Output = T> + Sub<Output = T>> Increment for T {
    fn increment(&self) -> Self {
        self.clone() + T::one()
    }

    fn decrement(&self) -> Self {
        self.clone() - T::one()
    }
}
