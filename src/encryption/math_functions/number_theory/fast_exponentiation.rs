use num::{BigInt, One, Zero};
use num::traits::Euclid;

use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::parity::Parity;

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
pub struct FastExponentiation {}


impl FastExponentiation {
    ///
    /// Berechnet die schnelle Exponentiation der Potenz und Reduzierung um einen Modul.
    ///
    /// # Argumente
    ///
    /// * `base` - Die Basis, von welcher die Potenz berechnet werden soll.
    /// * `exponent`- Der Exponent zur Berechnung der Potenz.
    /// * `modul` - Der Modul, durch den reduziert werden soll.
    ///
    pub fn calculate(base: &BigInt, exponent: &BigInt, modul: &BigInt, use_fast: bool) -> BigInt {
        return if use_fast {
            FastExponentiation::fast(base, exponent, modul)
        } else {
            FastExponentiation::own(base, exponent, modul)
        };
    }

    fn fast(base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
        base.modpow(exponent, modul)
    }

    fn own(base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
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