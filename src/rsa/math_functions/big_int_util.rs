use ibig::{ubig, UBig};

// TODO: TESTEN

///
/// Gibt zurück, ob die Zahl gerade ist.
///
pub fn is_even(x: &UBig) -> bool {
    !is_uneven(x)
}

///
/// Gibt zurück, ob die Zahl ungerade ist.
///
pub fn is_uneven(x: &UBig) -> bool {
    // Ist das letzte Bit eine 1, so ist die Zahl ungerade.
    return x.bit(0);
}

///
/// Gibt zurück, ob die Zahl 0 ist.
///
pub fn is_zero(x: &UBig) -> bool {
    x == &ubig!(0)
}

///
/// Gibt zurück, ob die Zahl 1 ist.
///
pub fn is_one(x: &UBig) -> bool {
    x == &ubig!(1)
}

///
/// Gibt zurück, ob a teilt b.
/// Also b % a == 0
///
pub fn divides(a: &UBig, b: &UBig) -> bool {
    return b % a == ubig!(0);
}

///
/// Gibt zurück, ob a teilt nicht b.
/// Also b % a != 0
///
pub fn not_divides(a: &UBig, b: &UBig) -> bool {
    return b % a != ubig!(0);
}
