use std::ops::{Div, MulAssign};
use bigdecimal::num_bigint::{BigInt, BigUint};
use bigdecimal::{BigDecimal, One, Zero};
use bigdecimal::num_traits::real::Real;

///
/// Erstellt einen BigUint aus einem unsigned Integer.
#[macro_export]
macro_rules! big_u {
    ($x:expr) => {
        BigUint::from($x as u128)
    };
}

///
/// Gibt zurück, ob die Zahl gerade ist.
///
pub fn is_even(x: &BigUint) -> bool {
    !is_uneven(x)
}

///
/// Gibt zurück, ob die Zahl ungerade ist.
///
pub fn is_uneven(x: &BigUint) -> bool {
    // Ist das letzte Bit eine 1, so ist die Zahl ungerade.
    return x.bit(0);
}

///
/// Gibt zurück, ob die Zahl 0 ist.
///
pub fn is_zero(x: &BigUint) -> bool {
    x == &BigUint::zero()
}

///
/// Gibt zurück, ob die Zahl 1 ist.
///
pub fn is_one(x: &BigUint) -> bool {
    x == &BigUint::one()
}

///
/// Gibt zurück, ob a teilt b.
/// Also b % a == 0
///
pub fn divides(a: &BigUint, b: &BigUint) -> bool {
    return b % a == BigUint::zero();
}

///
/// Gibt zurück, ob a teilt nicht b.
/// Also b % a != 0
///
pub fn not_divides(a: &BigUint, b: &BigUint) -> bool {
    return b % a != BigUint::zero();
}

///
/// Inkrementiert die übergebene Zahl.
///
pub fn increment(a: &BigUint) -> BigUint {
    a + BigUint::one()
}

///
/// Dekrementiert die übergebene Zahl.
///
pub fn decrement(a: &BigUint) -> BigUint {
    a - BigUint::one()
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



