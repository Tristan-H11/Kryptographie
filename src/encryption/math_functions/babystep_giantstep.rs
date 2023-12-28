use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};

use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
use crate::encryption::math_functions::traits::increment::Increment;

/// Berechnet den Logarithmus der Basis `base` von einem Element `element` einer Restklasse
/// reduziert durch den Modulus `modul`. Der Modulus muss prim sein!
///
/// # Argumente
///
/// * `base` - Eine primitive Wurzel der Restklasse des Moduls.
/// * `element` - Ein Element der Restklasse des Moduls.
/// * `modul` - Der Modulus.
///
/// # Rückgabewert
///
/// * Der berechnete Logarithmus.
///
/// # Fehler
///
/// * `ErrorKind::InvalidInput` - Wenn der Logarithmus nicht existiert.
///
/// # Beispiel
///
/// ```rust
/// let base = BigInt::from(3);
/// let element = BigInt::from(4);
/// let modul = BigInt::from(7);
///
/// let result = shanks(&base, &element, &modul, true);
///
/// assert_eq!(result, Ok(BigInt::from(4)));
/// ```
pub fn shanks(base: &BigInt, element: &BigInt, modul: &BigInt, use_fast: bool) -> Result<BigInt, Error> {
    //aufrundung: nachkommateil abschneiden (to_bigint) +1
    let mut m = (modul - BigInt::one()).sqrt();
    if (&m * &m) != (modul - BigInt::one()) {
        m += BigInt::one();
    }

    //Berechnet Giantsteps und speichert sie
    let g_ex_m = FastExponentiation::calculate(base, &m, modul, use_fast);
    let mut hash: HashMap<BigInt, BigInt> = HashMap::new();
    let mut j = BigInt::zero();
    while j < m {
        let giantstep = FastExponentiation::calculate(&g_ex_m, &j, modul, use_fast);
        hash.insert(j.clone(), giantstep);
        j.increment_assign();
    }

    //Berechnet Babysteps und vergleicht sie mit Giantsteps
    let mut i = BigInt::zero();
    while i < m {
        j = BigInt::zero();
        let babystep = (element * FastExponentiation::calculate(base, &(modul - BigInt::one() - &i), modul, use_fast)) % modul;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big_i;
    use num::BigInt;

    #[test]
    fn shanks_test() {
        assert_eq!(
            shanks(&big_i!(8), &big_i!(555), &big_i!(677), false).unwrap(), //TODO UseFast einbauen
            big_i!(134)
        );
        assert_eq!(
            shanks(&big_i!(11), &big_i!(3), &big_i!(29), false).unwrap(), //TODO UseFast einbauen
            big_i!(17)
        );
        assert_eq!(
            shanks(&big_i!(10), &big_i!(25), &big_i!(97), false).unwrap(), //TODO UseFast einbauen
            big_i!(22)
        );
        assert_eq!(
            shanks(&big_i!(3), &big_i!(4), &big_i!(7), false).unwrap(), //TODO UseFast einbauen
            big_i!(4)
        );
        assert!(shanks(&big_i!(4), &big_i!(6), &big_i!(7), false).is_err()); //Da Base nicht primitive Wurzel! //TODO UseFast einbauen
    }
}
