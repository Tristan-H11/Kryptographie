use crate::math_core::number_theory::extended_euclid_result::ExtendedEuclidResult;
use anyhow::{ensure, Result};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::{One, Zero};

use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Slow;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::number_theory::primality_test::PrimalityTest;
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::divisible::Divisible;
use crate::math_core::traits::parity::Parity;
use crate::shared::errors::ArithmeticError;

#[derive(Clone, Copy, Debug)]
pub struct SlowNumberTheoryService;

impl SlowNumberTheoryService {
    pub fn new() -> SlowNumberTheoryService {
        SlowNumberTheoryService
    }
}

impl NumberTheoryServiceTrait for SlowNumberTheoryService {
    fn extended_euclid(&self, a: &BigInt, b: &BigInt) -> ExtendedEuclidResult {
        let mut m = b.clone();
        let mut n = a.clone();
        //rotierendes Array, zur Berechnung und Speicherung der Faktoren `x` und `y`
        let mut xy = [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
        while !m.is_zero() {
            //Berechnet die Faktoren und speichert sie in einem rotierenden Array.
            let div = &n / &m;
            xy[0] = &xy[0] - (&div * &xy[2]);
            xy[1] = &xy[1] - (&div * &xy[3]);
            let tmp = &n % &m;
            n = m;
            m = tmp;
            xy.rotate_right(2);
        }
        if n >= BigInt::zero() {
            ExtendedEuclidResult::new(n, xy[0].clone(), xy[1].clone())
        } else {
            ExtendedEuclidResult::new(
                BigInt::zero() - n,
                BigInt::zero() - xy[0].clone(),
                BigInt::zero() - xy[1].clone(),
            )
        }
    }

    fn fast_exponentiation(&self, base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
        if base.is_zero() && !exponent.is_zero() {
            return BigInt::zero();
        }
        if !base.is_zero() && exponent.is_zero() {
            return BigInt::one();
        }
        if modul.is_one() {
            return BigInt::zero();
        }
        if base.is_one() {
            return BigInt::one();
        }

        let mut result = BigInt::one();
        let mut base = base.clone();
        let mut exp = exponent.clone();

        while !exp.is_zero() {
            if exp.is_odd() {
                result = (result * &base).rem_euclid(modul);
            }
            base = (&base * &base).rem_euclid(modul);
            exp.half_assign();
        }
        result
    }

    fn modulo_inverse(&self, n: &BigInt, modul: &BigInt) -> Result<BigInt> {
        let number_theory_service = SlowNumberTheoryService::new();
        let extended_euclid_result = number_theory_service.extended_euclid(modul, n);

        ensure!(
            extended_euclid_result.ggt.is_one(),
            ArithmeticError::NoInverseError(n.to_string(), modul.to_string())
        );

        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + extended_euclid_result.y).rem_euclid(modul));
    }

    fn is_probably_prime(
        &self,
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
    ) -> bool {
        let primality_test = PrimalityTest::new(NumberTheoryService::new(Slow));

        if PrimalityTest::fails_primitive_prime_checks(p) {
            return false;
        }

        primality_test.miller_rabin(p, repeats, random_generator)
    }
}
