use num::{BigInt, One, Zero};
use num::traits::Euclid;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::parity::Parity;
use crate::encryption::math_functions::traits::rapid_math_ops::RapidMathOps;

///
/// Berechnet die schnelle Exponentiation der Potenz und Reduzierung um einen Modul.
///
/// # Argumente
/// * `base` - Die Basis, von welcher die Potenz berechnet werden soll.
/// * `exponent`- Der Exponent zur Berechnung der Potenz.
/// * `modul` - Der Modul, durch den reduziert werden soll.
///
/// # Beispiel
/// ```
/// fast_exponentiation(95, 130, 7) // => '4'
/// ```
pub struct FastExponentiation {
    base: BigInt,
    exponent: BigInt,
    modul: BigInt,
}

impl RapidMathOps<BigInt> for FastExponentiation {
    fn fast(&self) -> BigInt {
        self.base.modpow(&self.exponent, &self.modul)
    }

    fn own(&self) -> BigInt {
        self.fast_exponentiation(&self.base, &self.exponent, &self.modul)
    }
}

impl FastExponentiation {
    ///
    /// Erstellt eine neue Instanz von FastExponentiation.
    ///
    /// # Argumente
    ///
    /// * `base` - Die Basis, von welcher die Potenz berechnet werden soll.
    /// * `exponent`- Der Exponent zur Berechnung der Potenz.
    /// * `modul` - Der Modul, durch den reduziert werden soll.
    ///
    pub fn new(base: BigInt, exponent: BigInt, modul: BigInt) -> Self {
        FastExponentiation {
            base,
            exponent,
            modul,
        }
    }

    ///
    /// Setzt die Basis der Potenz auf einen neuen Wert, um kein neues Objekt erstellen zu müssen.
    ///
    pub fn set_base(&mut self, base: BigInt) {
        self.base = base;
    }

    /// Schnelle Exponentiation der Potenz und Reduzierung um einen Modul.
    /// Alternativer Ansatz von Herrn Elsner zur schnellen Exponentiation durch Halbieren der Potenz.
    fn fast_exponentiation(base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
        // Sonderbedingungen der Exponentiation
        if modul.is_one() {
            return BigInt::zero();
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
}