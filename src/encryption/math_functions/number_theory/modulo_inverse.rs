use std::io::{Error, ErrorKind};
use num::{BigInt, One};
use num::traits::Euclid;
use crate::encryption::math_functions::number_theory::extended_euclid::ExtendedEuclid;
use crate::encryption::math_functions::traits::rapid_math_ops::RapidMathOps;

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
pub struct ModuloInverse {
    n: BigInt,
    modul: BigInt,
}

impl RapidMathOps<Result<BigInt, Error>> for ModuloInverse {
    fn fast(&self) -> Result<BigInt, Error> {
        ModuloInverse::modulo_inverse(&self.n, &self.modul, true)
    }

    fn own(&self) -> Result<BigInt, Error> {
        ModuloInverse::modulo_inverse(&self.n, &self.modul, false)
    }
}


impl ModuloInverse {
    /// Erstellt eine neue Instanz von ModuloInverse.
    pub fn new(n: BigInt, modul: BigInt) -> Self {
        ModuloInverse {
            n,
            modul,
        }
    }

    /// Eigene Implementation des modularen Inversen.
    fn modulo_inverse(n: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
        let euclid = ExtendedEuclid::new(n.clone(), modul.clone());
        let (ggt, _x, y) = euclid.calculate(use_fast);
        // Wenn ggT nicht 1, existiert kein Inverse. -> Error
        if !ggt.is_one() {
            let no_inverse_error = Error::new(ErrorKind::InvalidInput, format!("n hat keinen Inverse"));
            return Err(no_inverse_error);
        }
        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + y).rem_euclid(modul));
    }
}