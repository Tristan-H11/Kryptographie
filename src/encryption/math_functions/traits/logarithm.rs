use std::ops::Mul;

use bigdecimal::One;

pub trait Logarithm {
    ///
    /// Berechnet den Logarithmus abgerundet zu einer Basis.
    ///
    /// # Argumente
    ///
    /// * `base` - Die Basis des Logarithmus.
    ///
    /// # Rückgabe
    ///
    /// * `u32` - Der abgerundete Logarithmus.
    fn log(&self, base: &Self) -> u32;
}

impl<T: One + Clone + Mul<Output = T> + PartialOrd> Logarithm for T {
    fn log(&self, base: &Self) -> u32 {
        let mut count = 0;
        let mut current_value = T::one();

        loop {
            if &current_value > self {
                // Überschreiten wir den Wert, gehen wir einen Schritt zurück und sind fertig.
                count -= 1;
                break;
            }
            current_value = current_value * base.clone();
            count += 1;
        }

        count
    }
}
