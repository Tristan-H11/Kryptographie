use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};

use crate::encryption::math_functions::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::encryption::math_functions::traits::increment::Increment;

#[derive(Clone, Copy)]
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
    ) -> Result<BigInt, Error> {
        //aufrundung: nachkommateil abschneiden (to_bigint) +1
        let mut m = (modul - BigInt::one()).sqrt();
        if (&m * &m) != (modul - BigInt::one()) {
            m += BigInt::one();
        }

        //Berechnet Giantsteps und speichert sie
        let g_ex_m = self
            .number_theory_service
            .fast_exponentiation(base, &m, modul);
        let mut hash: HashMap<BigInt, BigInt> = HashMap::new();
        let mut j = BigInt::zero();
        while j < m {
            let giantstep = self
                .number_theory_service
                .fast_exponentiation(&g_ex_m, &j, modul);
            hash.insert(j.clone(), giantstep);
            j.increment_assign();
        }

        //Berechnet Babysteps und vergleicht sie mit Giantsteps
        let mut i = BigInt::zero();
        while i < m {
            j = BigInt::zero();
            let babystep = (element
                * self.number_theory_service.fast_exponentiation(
                    base,
                    &(modul - BigInt::one() - &i),
                    modul,
                ))
                % modul;
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
    use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

    use super::*;

    fn run_test_for_all_services(test: impl Fn(NumberTheoryService)) {
        test(NumberTheoryService::new(Slow)); // Langsame, eigene Implementierung
        test(NumberTheoryService::new(Fast)); // Schnelle, externe Implementierung
    }

    #[test]
    fn shanks_test() {
        run_test_for_all_services(|service| {
            let shanks_service = Shanks::new(service);
            let result = shanks_service
                .calculate(&8.into(), &555.into(), &677.into())
                .unwrap();
            assert_eq!(result, 134.into());
            let result = shanks_service
                .calculate(&11.into(), &3.into(), &29.into())
                .unwrap();
            assert_eq!(result, 17.into());
            let result = shanks_service
                .calculate(&10.into(), &25.into(), &97.into())
                .unwrap();
            assert_eq!(result, 22.into());
            let result = shanks_service
                .calculate(&3.into(), &4.into(), &7.into())
                .unwrap();
            assert_eq!(result, 4.into());
            let result = shanks_service.calculate(&4.into(), &6.into(), &7.into());
            assert!(result.is_err());
            //Da Base nicht primitive Wurzel!
        });
    }
}
