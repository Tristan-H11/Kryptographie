use std::ops::MulAssign;

use bigdecimal::One;
use bigdecimal::num_bigint::BigUint;

///
/// Erstellt einen BigUint aus einem unsigned Integer.
///
#[macro_export]
macro_rules! big_u {
    ($x:expr) => {
        BigUint::from($x as u128)
    };
}

///
/// Erstellt einen BigInt aus einem signed Integer.
///
#[macro_export]
macro_rules! big_i {
    ($x:expr) => {
        BigInt::from($x)
    };
}



///
/// Berechnet den Logarithmus zu einer Basis.
///
/// # Argumente
///
/// * `x` - Die Zahl, zu der der Logarithmus berechnet werden soll.
/// * `base` - Die Basis des Logarithmus.
///
/// # Rückgabe
///
/// * `BigDecimal` - Der Logarithmus.
///
pub fn log_base_g(x: &BigUint, base: &BigUint) -> u32 {
    let mut count = 0;
    let mut current_value = BigUint::one();

    while &current_value < x {
        current_value.mul_assign(base);
        count += 1;
    }

    count
}
