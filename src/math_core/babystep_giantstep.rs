use std::collections::BTreeMap;

use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::traits::increment::Increment;
use crate::shared::errors::ArithmeticError;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};
use serde::Serialize;

#[derive(Clone, Copy)]
pub struct Shanks {
    number_theory_service: NumberTheoryService,
}

pub struct ShanksResult {
    pub result: BigInt,
    pub map: Vec<(BigInt, BigInt)>,
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
    /// * Der berechnete Logarithmus und die zugrundeliegende Tabelle der GiantSteps.
    ///
    /// # Fehler
    ///
    /// * `ArithmeticError::NoDiscreteLogarithmError` - Wenn der Logarithmus nicht existiert.
    pub fn calculate(
        self,
        base: &BigInt,
        element: &BigInt,
        modul: &BigInt,
    ) -> Result<ShanksResult, ArithmeticError> {
        //aufrundung: nachkommateil abschneiden (to_bigint) +1
        let mut m = (modul - BigInt::one()).sqrt();
        if (&m * &m) != (modul - BigInt::one()) {
            m += BigInt::one();
        }

        //Berechnet Giantsteps und speichert sie
        let g_ex_m = self
            .number_theory_service
            .fast_exponentiation(base, &m, modul);
        let mut map: BTreeMap<BigInt, BigInt> = BTreeMap::new();
        let mut j = BigInt::zero();
        while j < m {
            let giantstep = self
                .number_theory_service
                .fast_exponentiation(&g_ex_m, &j, modul);
            map.insert(giantstep, j.clone());
            j.increment_assign();
        }

        // Map in einen Vektor von Tupeln umwandeln
        let mut sorted_map: Vec<(BigInt, BigInt)> = map
            .clone()
            .into_iter()
            .map(|(key, value)| (value, key))
            .collect();

        // Vektor nach den Values sortieren (also nach dem 2. Element des Tupels)
        sorted_map.sort_by(|a, b| a.0.cmp(&b.0));

        //Berechnet Babysteps und vergleicht sie mit Giantsteps
        let mut i = BigInt::zero();
        while i < m {
            let babystep = (element
                * self.number_theory_service.fast_exponentiation(
                    base,
                    &(modul - BigInt::one() - &i),
                    modul,
                ))
                % modul;
            let pair = map.get(&babystep);
            if pair.is_some() {
                let final_value = (&m * pair.unwrap() + &i) % (modul - BigInt::one());
                let result = ShanksResult {
                    result: final_value,
                    map: sorted_map,
                };
                return Ok(result);
            }
            i.increment_assign();
        }
        Err(ArithmeticError::NoDiscreteLogarithmError(
            base.to_str_radix(10),
            element.to_str_radix(10),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::{
        Fast, Slow,
    };

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
                .unwrap()
                .result;
            assert_eq!(result, 134.into());
            let result = shanks_service
                .calculate(&11.into(), &3.into(), &29.into())
                .unwrap()
                .result;
            assert_eq!(result, 17.into());
            let result = shanks_service
                .calculate(&10.into(), &25.into(), &97.into())
                .unwrap()
                .result;
            assert_eq!(result, 22.into());
            let result = shanks_service
                .calculate(&3.into(), &4.into(), &7.into())
                .unwrap()
                .result;
            assert_eq!(result, 4.into());
            let result = shanks_service.calculate(&4.into(), &6.into(), &7.into());
            assert!(result.is_err());
            //Da Base nicht primitive Wurzel!
        });
    }
}
