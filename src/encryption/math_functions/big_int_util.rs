use bigdecimal::num_bigint::BigUint;
use bigdecimal::{One, Zero};

///
/// Erstellt einen BigUint aus einem unsigned Integer.
#[macro_export]
macro_rules! big_u {
    ($x:expr) => {
        BigUint::from($x)
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
/// Konvertiere ein Zeichen in einen u16 Code -- z.B. für Blockchiffre
///
pub(crate) fn c_to_u32(c: char) -> u32 {
    c as u32
}

///
/// Konvertiere ein u32 Code in ein Zeichen -- z.B. für Blockchiffre
///
pub(crate) fn u32_to_c(value: u32) -> char {
    match char::from_u32(value) {
        Some(x) => x,
        None => panic!("oben Ungültiger u32 Wert: {}", value),
    }
}

///
/// wandle eine ubig Zahl in einen u32 Wert um
///
pub(crate) fn ubig_to_u32(value: &BigUint) -> u32 {
    let value_str = format!("{}", value);
    match value_str.parse::<u32>(){
        Ok(x) => x,
        Err(_) => panic!("unten Ungültiger u32 Wert: {}", value),
    }
}
