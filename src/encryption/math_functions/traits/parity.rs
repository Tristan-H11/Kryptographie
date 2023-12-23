use bigdecimal::num_bigint::BigInt;

///
/// Trait für die Prüfung der Parität einer Zahl
///
pub trait Parity {
    ///
    /// Gibt zurück, ob die Zahl gerade ist.
    ///
    fn is_even(&self) -> bool;

    ///
    /// Gibt zurück, ob die Zahl ungerade ist.
    ///
    fn is_odd(&self) -> bool;
}

impl Parity for BigInt {
    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    fn is_odd(&self) -> bool {
        self.bit(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big_i;

    #[test]
    fn test_is_even() {
        let big_int: BigInt = big_i!(34563u32);
        assert_eq!(big_int.is_even(), false);
        assert_eq!(big_i!(8564u32).is_even(), true);
    }

    #[test]
    fn test_is_uneven() {
        assert_eq!(big_i!(24390u32).is_odd(), false);
        assert_eq!(big_i!(1435099u32).is_odd(), true);
    }
}
