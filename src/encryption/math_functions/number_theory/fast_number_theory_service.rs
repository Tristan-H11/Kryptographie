use std::io::{Error, ErrorKind};

use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::One;
use num::Integer;
use crate::encryption::math_functions::number_theory::extended_euclid_result::ExtendedEuclidResult;

use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::encryption::math_functions::number_theory::primality_test::PrimalityTest;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;

#[derive(Clone, Copy)]
pub struct FastNumberTheoryService;

impl FastNumberTheoryService {
    pub fn new() -> FastNumberTheoryService {
        FastNumberTheoryService
    }
}

impl NumberTheoryServiceTrait for FastNumberTheoryService {
    fn extended_euclid(&self, a: &BigInt, b: &BigInt) -> ExtendedEuclidResult {
        let e = a.extended_gcd(b);
        ExtendedEuclidResult::new(e.gcd, e.x, e.y)
    }

    fn fast_exponentiation(&self, base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
        base.modpow(exponent, modul)
    }

    fn modulo_inverse(&self, n: &BigInt, modul: &BigInt) -> Result<BigInt, Error> {
        let number_theory_service = FastNumberTheoryService::new();
        let extended_euclid_result = number_theory_service.extended_euclid(modul, n);
        if !extended_euclid_result.ggt.is_one() {
            let no_inverse_error = Error::new(ErrorKind::InvalidInput, "n hat keinen Inverse");
            return Err(no_inverse_error);
        }
        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + extended_euclid_result.y).rem_euclid(modul));
    }

    fn is_probably_prime(
        &self,
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
    ) -> bool {
        let primality_test = PrimalityTest::new(NumberTheoryService::new(Fast));

        // Enthält noch einige weitere Tests, die für slow nicht vorgesehen sind.
        if PrimalityTest::fails_primitive_prime_checks(p) {
            return false;
        }
        // Sind die primitiven Tests bestanden, läuft miller_rabin an.
        primality_test.miller_rabin(p, repeats, random_generator)
    }
}
