use std::io::{Error, ErrorKind};

use num::{BigInt, One};
use num::traits::Euclid;

use crate::encryption::math_functions::number_theory::extended_euclid::ExtendedEuclid;

/// Berechnet das Inverse-Element in einem Restklassenring.
///
/// Das Inverse-Element einer Zahl `n` im Restklassenring modulo `modul` ist
/// eine andere Zahl `x`, so dass `(n * x) % modul = 1`
/// (das neutrale Element der Multiplikation).
///
/// # Argumente
/// * `n` - Die zu invertierende Zahl.
/// * `modul` - Die Modulo-Zahl, gegen die die Inversion durchgeführt wird.
///
/// # Rückgabe
/// * Result<inverse, Error>
/// Das Inverse-Element von `n` im Restklassenring modulo `modul`.
/// Wenn keine Inverse existiert (wenn `n` und `modul` nicht teilerfremd sind),
/// wird ein Error zurückgegeben.
pub struct ModuloInverse {}

impl ModuloInverse {
    /// Berechnet das multiplikative Inverse-Element in einem Restklassenring.
    pub fn calculate(n: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
        return ModuloInverse::modulo_inverse(n, modul, use_fast);
    }

    /// Eigene Implementation des modularen Inversen.
    fn modulo_inverse(n: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
        let (ggt, _x, y) = ExtendedEuclid::calculate(modul, n, use_fast);
        // Wenn ggT nicht 1, existiert kein Inverse. -> Error
        if !ggt.is_one() {
            let no_inverse_error = Error::new(ErrorKind::InvalidInput, format!("n hat keinen Inverse"));
            return Err(no_inverse_error);
        }
        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + y).rem_euclid(modul));
    }
}