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
