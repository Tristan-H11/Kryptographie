use ibig::ops::RemEuclid;
use ibig::{UBig, ubig};

///
/// Gibt zurück, ob die Zahl gerade ist.
///
pub fn is_even(x: &UBig) -> bool {
    return x.rem_euclid(2) == 0;
}

///
/// Gibt zurück, ob die Zahl ungerade ist.
///
pub fn is_uneven(x: &UBig) -> bool {
    return x.rem_euclid(2) == 1;
}

///
/// Gibt zurück, ob die Zahl 0 ist.
///
pub fn is_zero(x: &UBig) -> bool {
    x == ubig!(0)
}

///
/// Gibt zurück, ob die Zahl 1 ist.
///
pub fn is_one(x: &UBig) -> bool {
    x == ubig!(1)
}