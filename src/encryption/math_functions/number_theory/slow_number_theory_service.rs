use std::io::{Error, ErrorKind};

use bigdecimal::{One, Zero};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;

use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::Slow;
use crate::encryption::math_functions::number_theory::primality_test::PrimalityTest;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::parity::Parity;

pub struct SlowNumberTheoryService;

impl SlowNumberTheoryService {
    pub fn new() -> SlowNumberTheoryService {
        SlowNumberTheoryService
    }
}

impl NumberTheoryServiceTrait for SlowNumberTheoryService {
    fn extended_euclid(&self, a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        //rotierendes Array, zur Berechnung und Speicherung der Faktoren `x` und `y`
        let mut xy = [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
        let mut m = b.clone();
        let mut n = a.clone();
        while !m.is_zero() {
            // Berechnet die Faktoren und speichert sie in einem rotierenden Array.
            let div = &n / &m;
            xy[0] = &xy[0] - (&div * &xy[2]);
            xy[1] = &xy[1] - (&div * &xy[3]);
            let tmp = &n % &m;
            n = m;
            m = tmp;
            xy.rotate_right(2);
        }
        (n, xy[0].clone(), xy[1].clone())
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

    fn modulo_inverse(&self, n: &BigInt, modul: &BigInt) -> Result<BigInt, Error> {
        let number_theory_service = SlowNumberTheoryService::new();
        let (ggt, _x, y) = number_theory_service.extended_euclid(modul, n);
        if !ggt.is_one() {
            let no_inverse_error =
                Error::new(ErrorKind::InvalidInput, "n hat keinen Inverse");
            return Err(no_inverse_error);
        }
        // Berechnet aus den letzten Faktoren das Inverse.
        return Ok((modul + y).rem_euclid(modul));
    }

    fn is_probably_prime(&self, p: &BigInt, repeats: u32, random_generator: &PseudoRandomNumberGenerator) -> bool {
        let primality_test = PrimalityTest::new(NumberTheoryService::new(Slow));
        primality_test.miller_rabin(p, repeats, random_generator)
    }
}