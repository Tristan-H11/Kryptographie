///
/// Bündelt Operationen für schnelle und langsame Berechnungen von Algorithmen.
///
/// # Argumente
///
/// * `T` - Der Typ der zu berechnenden Werte.
pub trait RapidMathOps<T> {

    ///
    /// Führt eine schnelle Berechnungen des Algorithmus mittels Library-Methoden durch.
    ///
    fn fast(&self) -> T;

    ///
    /// Führt eine vergleichsweise langsame Berechnung mit selbst-implementierten Algorithmen durch.
    ///
    fn own(&self) -> T;

    ///
    /// Führt eine Berechnung des Algorithmus durch.
    ///
    /// # Argumente
    ///
    /// * `use_fast` - Gibt an, ob die schnelle oder langsame Berechnung durchgeführt werden soll.
    fn calculate(&self, use_fast: bool) -> T {
        if use_fast {
            self.fast()
        } else {
            self.own()
        }
    }
}