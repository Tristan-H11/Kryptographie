use ibig::ops::RemEuclid;
use ibig::{ubig, UBig};
use std::ops::Div;

///
/// Schnelle Exponentiation der Potenz und Reduzierung um einen Modul.
/// Alternativer Ansatz von Herrn Elsner zur schnellen Exponentiation durch Halbieren der Potenz.
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
pub fn fast_exponentiation(base: &UBig, exponent: &UBig, modul: &UBig) -> UBig {
    let zero: &UBig = &ubig!(0);
    let one: &UBig = &ubig!(1);

    // Sonderbedingungen der Exponentiation
    if modul == one {
        return zero.clone();
    }
    if exponent == zero {
        return one.clone();
    }
    if exponent == one {
        return base.rem_euclid(modul);
    }

    // Berechnung des Zwischenschrittes mit halbiertem Exponenten.
    let base_to_square = fast_exponentiation(base, &exponent.div(2), modul);

    return if exponent.rem_euclid(2) == 1 {
        // Ist der Exponent ungerade, wird die Basis erneut als Faktor herangezogen.
        (base_to_square.pow(2) * base).rem_euclid(modul)
    } else {
        // Ist der Exponent gerade, so wird nur quadriert.
        base_to_square.pow(2).rem_euclid(modul)
    };
}

pub fn expanded_euclidean_algorithm() {}
