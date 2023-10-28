use std::ops::Rem;
use bigdecimal::Zero;

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

impl<T: Zero + Clone + Rem<Output=T> + PartialEq + From<u8>> Parity for T {
    fn is_even(&self) -> bool {
        self.clone() % T::from(2u8) == T::zero()
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}