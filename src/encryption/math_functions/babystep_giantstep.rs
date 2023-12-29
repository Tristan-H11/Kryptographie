use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use bigdecimal::{One, Zero};
use bigdecimal::num_bigint::BigInt;

use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::traits::increment::Increment;

pub struct Shanks {
    number_theory_service: NumberTheoryService,
}

impl Shanks {
    pub fn new(number_theory_service: NumberTheoryService) -> Shanks {
        Shanks {
            number_theory_service,
        }
    }
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
    /// let result = Shanks::calculate(&base, &element, &modul, true);
    ///
    /// assert_eq!(result, Ok(BigInt::from(4)));
    /// ```
    pub fn calculate(
        self,
        base: &BigInt,
        element: &BigInt,
        modul: &BigInt,
        use_fast: bool,
    ) -> Result<BigInt, Error> {
        //aufrundung: nachkommateil abschneiden (to_bigint) +1
        let mut m = (modul - BigInt::one()).sqrt();
        if (&m * &m) != (modul - BigInt::one()) {
            m += BigInt::one();
        }

        //Berechnet Giantsteps und speichert sie
        let g_ex_m = self.number_theory_service.fast_exponentiation(base, &m, modul);
        let mut hash: HashMap<BigInt, BigInt> = HashMap::new();
        let mut j = BigInt::zero();
        while j < m {
            let giantstep = self.number_theory_service.fast_exponentiation(&g_ex_m, &j, modul);
            hash.insert(j.clone(), giantstep);
            j.increment_assign();
        }

        //Berechnet Babysteps und vergleicht sie mit Giantsteps
        let mut i = BigInt::zero();
        while i < m {
            j = BigInt::zero();
            let babystep =
                (element * self.number_theory_service.fast_exponentiation(
                    base,
                    &(modul - BigInt::one() - &i),
                    modul,
                )) % modul;
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
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use crate::big_i;

    use super::*;

    #[test]
    fn shanks_test() {
        assert_eq!(
            Shanks::calculate(&big_i!(8), &big_i!(555), &big_i!(677), false).unwrap(), //TODO UseFast einbauen
            big_i!(134)
        );
        assert_eq!(
            Shanks::calculate(&big_i!(11), &big_i!(3), &big_i!(29), false).unwrap(), //TODO UseFast einbauen
            big_i!(17)
        );
        assert_eq!(
            Shanks::calculate(&big_i!(10), &big_i!(25), &big_i!(97), false).unwrap(), //TODO UseFast einbauen
            big_i!(22)
        );
        assert_eq!(
            Shanks::calculate(&big_i!(3), &big_i!(4), &big_i!(7), false).unwrap(), //TODO UseFast einbauen
            big_i!(4)
        );
        assert!(Shanks::calculate(&big_i!(4), &big_i!(6), &big_i!(7), false).is_err());
        //Da Base nicht primitive Wurzel! //TODO UseFast einbauen
    }
}
