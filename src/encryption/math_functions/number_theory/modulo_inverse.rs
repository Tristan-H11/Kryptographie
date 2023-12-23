use std::io::{Error, ErrorKind};

use num::traits::Euclid;
use num::{BigInt, One};

use crate::encryption::math_functions::number_theory::extended_euclid::ExtendedEuclid;

/// Implementiert den Algorithmus zur Berechnung des modularen Inversen.
pub struct ModuloInverse {}

impl ModuloInverse {
    /// Berechnet das modulare Inverse von `n` modulo `modul`.
    ///
    /// # Argumente
    ///
    /// * `n` - Die Zahl, für die das inverse berechnet werden soll.
    /// * `modul` - Der Modulus.
    /// * `use_fast` - Gibt an, ob die schnelle Implementierung verwendet werden soll.
    ///
    /// # Rückgabewert
    ///
    /// * Das modulare Inverse von `n` modulo `modul`.
    ///
    /// # Fehler
    ///
    /// * `Error::InvalidInput` - Wenn `n` und `modul` nicht teilerfremd sind, dann existiert kein Inverse.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let n = BigInt::from(2);
    /// let modul = BigInt::from(5);
    ///
    /// let result = ModuloInverse::calculate(&n, &modul, true);
    ///
    /// assert_eq!(result, Ok(BigInt::from(3)));
    /// ```
    pub fn calculate(n: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
        let (ggt, _x, y) = ExtendedEuclid::calculate(modul, n, use_fast);
        if !ggt.is_one() {
            let no_inverse_error =
                Error::new(ErrorKind::InvalidInput, format!("n hat keinen Inverse"));
            return Err(no_inverse_error);
        }
        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + y).rem_euclid(modul));
    }
}
