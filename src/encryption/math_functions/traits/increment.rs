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
