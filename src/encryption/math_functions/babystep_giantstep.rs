use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use bigdecimal::{One, Zero};
use bigdecimal::num_bigint::BigInt;

use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::traits::increment::Increment;

///
/// Berechnet den Logarithmus eines Elementes einer Restklasse zur einer Basis
/// reduziert durch den Modul.
/// *!Modul muss Prim sein!*
///
/// # Argumente
/// * `base` - Eine primitive Wurzel der Restklasse des Moduls (g).
/// * `element` - Ein Element der Restklasse des Moduls (b).
/// * `modul` - *!Eine Primzahl!* (p).
///
/// # Rückgabe
/// * Result<x,Error>
///
/// `x` = log_b(g) mod p
/// also `x`, sodass g^x = b mod p
///
/// Error, wenn x nicht existiert. Wenn base eine primitive Wurzel, dann existiert x immer
///
/// TODO: Liste sortieren nach Größe zweiter Komponente
/// TODO: Error wenn falsche Eingabe (Modul keine Primzahl etc.)
pub fn shanks(base: &BigInt, element: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
    //aufrundung: nachkommateil abschneiden (to_bigint) +1
    let mut m = (modul - BigInt::one()).sqrt();
    if (&m * &m) != (modul - BigInt::one()) {
        m += BigInt::one();
    }

    let g_ex_m = FastExponentiation::calculate(base, &m, modul, use_fast);
    let mut hash: HashMap<BigInt, BigInt> = HashMap::new();
    let mut j = BigInt::zero();
    while j < m {
        let giantstep = FastExponentiation::calculate(&g_ex_m, &j, modul, use_fast);
        hash.insert(j.clone(), giantstep);
        j.increment_assign();
    }

    let mut i = BigInt::zero();
    while i < m {
        j = BigInt::zero();
        let babystep =
            (element * FastExponentiation::calculate(base, &(modul - BigInt::one() - &i), modul, use_fast)) % modul;
        while j < m {
            if hash.get(&j).unwrap() == &babystep {
                return Ok((&m * &j + &i) % (modul - BigInt::one()));
            }
            j.increment_assign();
        }
        i.increment_assign();
    }
    let no_exponent_error = Error::new(
        ErrorKind::InvalidInput,
        format!(
            "kein Exponent 'x', sodass {}^x = {} mod {}",
            base, element, modul
        ),
    );
    return Err(no_exponent_error);
}

pub fn log_naiv(base: &BigInt, element: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
    let mut x = BigInt::one();
    while &x < modul {
        if &FastExponentiation::calculate(base, &x, modul, use_fast) == element {
            return Ok(x);
        }
        x.increment_assign();
    }
    let no_exponent_error = Error::new(
        ErrorKind::InvalidInput,
        format!(
            "kein Exponent 'x', sodass {}^x = {} mod {}",
            base, element, modul
        ),
    );
    return Err(no_exponent_error);
}
