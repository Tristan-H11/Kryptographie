use crate::rsa::math_functions::big_int_util::{
    decrement, is_even, is_one, is_zero, random_in_range,
};
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
    // Sonderbedingungen der Exponentiation
    if is_one(&modul) {
        return ubig!(0);
    }
    if is_zero(&exponent) {
        return ubig!(1);
    }
    if is_one(&exponent) {
        return base.rem_euclid(modul);
    }

    // Berechnung des Zwischenschrittes mit halbiertem Exponenten.
    let base_to_square = fast_exponentiation(base, &exponent.div(2), modul);

    return if is_even(&exponent) {
        // Ist der Exponent gerade, so wird nur quadriert.
        base_to_square.pow(2).rem_euclid(modul)
    } else {
        // Ist der Exponent ungerade, wird die Basis erneut als Faktor herangezogen.
        (base_to_square.pow(2) * base).rem_euclid(modul)
    };
}

pub fn expanded_euclidean_algorithm() {}

/// Führt den Miller-Rabin-Primzahltest auf `p` durch `repeats` Runden aus.
///
/// # Argumente
/// * `p` - Die zu testende Zahl >= 11.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
///
/// # Rückgabe
/// `true`, wenn `p` wahrscheinlich eine Primzahl ist, andernfalls `false`.
///
/// # Beispiel
/// ```
/// miller_rabin(89, 40) // => true
/// miller_rabin(221, 40) // => false
/// ```
pub fn miller_rabin(p: &UBig, repeats: usize) -> bool {
    for _ in 0..repeats {
        if !miller_rabin_single(p) {
            return false;
        }
    }
    true
}

/// Führt den Miller-Rabin-Primzahltest auf `p` aus.
///
/// # Argumente
/// * `p` - Die zu testende Zahl >= 11.
///
/// # Rückgabe
/// `true`, wenn `p` wahrscheinlich eine Primzahl ist, andernfalls `false`.
fn miller_rabin_single(p: &UBig) -> bool {
    let one = &ubig!(1);
    let two = &ubig!(2);

    let mut d = decrement(p);
    let mut r = ubig!(0);

    while is_even(&d) {
        d = d.div(two);
        r = r + one;
    }

    // Fun Fact:
    // Wenn man p = 221 (NICHT prim) setzt und das a manuell auf 174 setzt, kommt er
    // fälschlicherweise auf "prim" als Ergebnis.
    let a = &random_in_range(&d);
    let mut x = fast_exponentiation(a, &d, p);

    if is_one(&x) || &x == &decrement(p) {
        return true;
    }
    while &r > one {
        x = fast_exponentiation(&x, two, p);
        if is_one(&x) {
            return false;
        }
        if &x == &decrement(p) {
            return true;
        }
        r = decrement(&r);
    }
    return false;
}
