use num::traits::Euclid;
use num::{BigInt, One, Zero};

use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::parity::Parity;

/// Implementiert den Schnellexponentiationsalgorithmus.
pub struct FastExponentiation {}

impl FastExponentiation {
    /// Berechnet die Schnellexponentiation für eine Basis `base`, einen Exponent `exponent`
    /// und einen Modulus `modul`.
    ///
    /// # Argumente
    ///
    /// * `base` - Die Basis.
    /// * `exponent` - Der Exponent.
    /// * `modul` - Der Modulus.
    /// * `use_fast` - Gibt an, ob die schnelle Implementierung verwendet werden soll.
    ///
    /// # Rückgabewert
    ///
    /// * Das Ergebnis der Schnellexponentiation.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let base = BigInt::from(2);
    /// let exponent = BigInt::from(3);
    /// let modulus = BigInt::from(5);
    ///
    /// let result = FastExponentiation::calculate(&base, &exponent, &modulus, true);
    ///
    /// assert_eq!(result, BigInt::from(3));
    /// ```
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
