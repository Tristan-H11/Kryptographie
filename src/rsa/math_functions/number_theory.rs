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
        (base_to_square.pow(2) * base // TODO: Muss hier base_to_square hin?
        ).rem_euclid(modul)
    };
}

/// Berechnet das Inverse-Element in einem Restklassenring.
///
/// Das Inverse-Element einer Zahl `n` im Restklassenring modulo `modul` ist
/// eine andere Zahl `x`, so dass `(n * x) % modul = 1`
/// (das neutrale Element der Multiplikation).
///
/// # Argumente
///
/// * `n` - Die zu invertierende Zahl.
/// * `modul` - Die Modulo-Zahl, gegen die die Inversion durchgeführt wird.
///
/// # Rückgabe
///
/// Das Inverse-Element von `n` im Restklassenring modulo `modul`. Wenn keine
/// Inverse existiert (z. B. wenn `n` und `modul` nicht teilerfremd sind), wird
/// ein Fehler ausgelöst.
pub fn modulo_inverse(n: i128, modul: i128) -> i128 {
    let xy = [1, 0, 1];
    // Berechnet aus den letzten Faktoren das Inverse.
    return (modul + extended_euclidean_algorithm(modul, n, xy)) % modul;
}

/// Implementiert den erweiterten euklidischen Algorithmus zur Berechnung des Inversen.
///
/// Der erweiterte euklidische Algorithmus wird verwendet, um das Inverse-Element
/// einer Zahl in einem Restklassenring zu finden. Er arbeitet rekursiv und berechnet
/// die Faktoren `x` und `y` in der Bézout'schen Identität, so dass `x * n + y * modul = ggT(n, modul)`
///
/// # Argumente
///
/// * `n` - Die zu invertierende Zahl.
/// * `modul` - Die Modulo-Zahl, gegen die die Inversion durchgeführt wird.
/// * `xy` - Ein rotierendes Array, das die Berechnung der Faktoren `x` und `y` speichert.
///
/// # Rückgabe
///
/// Das Inverse-Element von `n` im Restklassenring modulo `modul`. Wenn keine
/// Inverse existiert (z. B. wenn `n` und `modul` nicht teilerfremd sind), wird
/// ein Fehler ausgelöst.
/// TODO Simon: Ungetestet!
fn extended_euclidean_algorithm(n: i128, modul: i128, mut xy: [i128; 3]) -> i128 {
    xy.rotate_left(1);
    if modul == 0 {
        if n != 1 {
            panic!("n hat kein Inverses");
        }
        return xy[0];
    } else {
        // Berechnet die Faktoren und speichert sie in einem rotierenden Array.
        let div: i128 = n / modul;
        xy[2] = xy[0] - (div * xy[1]);
        return extended_euclidean_algorithm(modul, n % modul, xy);
    }
}

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
