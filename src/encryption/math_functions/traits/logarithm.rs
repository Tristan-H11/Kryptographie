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
    fn log(&self, base: &Self) -> usize;
}

impl<T: One + Clone + Mul<Output = T> + PartialOrd> Logarithm for T {
    fn log(&self, base: &Self) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::big_i;
    use num::BigInt;
    use super::*;

    #[test]
    fn test_log_base_g() {
        let x1 = big_i!(16);
        let base1 = big_i!(2);
        assert_eq!(x1.log(&base1), 4);

        let x2 = big_i!(6);
        let base2 = big_i!(4);
        assert_eq!(x2.log(&base2), 1);

        let x3 = big_i!(7);
        let base3 = big_i!(3);
        assert_eq!(x3.log(&base3), 1);

        let x4 = big_i!(2).pow(256);
        let base4 = big_i!(55296);
        assert_eq!(x4.log(&base4), 16);
    }
}
