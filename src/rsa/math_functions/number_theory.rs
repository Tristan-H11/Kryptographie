use ibig::ops::RemEuclid;
use ibig::{rand, ubig, UBig};
use std::ops::Div;
use crate::rsa::math_functions::big_int_util::{is_even, is_one, is_zero};

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

/// Führt den Miller-Rabin-Primzahltest auf `maybe_prime` durch `repeats` Runden aus.
///
/// # Argumente
/// * `maybe_prime` - Die zu testende Zahl.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
///
/// # Rückgabe
/// `true`, wenn `maybe_prime` wahrscheinlich eine Primzahl ist, andernfalls `false`.
pub fn miller_rabin_test(maybe_prime: &UBig, repeats: u8) -> bool {
    let zero: UBig = ubig!(0);
    let one: UBig = ubig!(1);
    let two: UBig = ubig!(2);
    let three: UBig = ubig!(3);
    let four: UBig = ubig!(4);

    if maybe_prime == &zero || maybe_prime == &four {
        return false;
    }
    if maybe_prime <= &three {
        return true;
    }

    let mut d = maybe_prime - &one;
    while d.is_even() {
        d /= 2u32;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..repeats {
        let a = rng.gen_range(ubig!(2u32)..maybe_prime - &ubig!(2u32));
        let x = fast_exponentiation(&a, &d, maybe_prime);

        if &x == &ubig!(1u32) || &x == maybe_prime - &ubig!(1u32) {
            continue;
        }

        let mut i = 0;
        let mut prev_x = x.clone();
        while i < maybe_prime - &ubig!(1u32) {
            x = (&prev_x * &prev_x) % maybe_prime;
            if &x == &ubig!(1u32) {
                return false;
            }
            if &x == maybe_prime - &ubig!(1u32) {
                break;
            }
            prev_x = x.clone();
            i += 1u32;
        }

        if &x != maybe_prime - &ubig!(1u32) {
            return false;
        }
    }

    true
}