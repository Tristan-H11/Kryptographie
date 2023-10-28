use std::io::{Error, ErrorKind};


use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::{One, Zero};

use crate::big_i;
use crate::encryption::math_functions::random_elsner::RandomElsner;
use crate::encryption::math_functions::traits::divisible::Divisible;
use crate::encryption::math_functions::traits::increment::Increment;
use crate::encryption::math_functions::traits::parity::Parity;

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
pub fn fast_exponentiation(base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
    // Sonderbedingungen der Exponentiation
    if modul.is_one() {
        return BigInt::zero();
    }
    if exponent.is_zero() {
        return BigInt::one();
    }
    if exponent.is_one() {
        return base.rem_euclid(modul);
    }

    // Berechnung des Zwischenschrittes mit halbiertem Exponenten.
    let base_to_square = fast_exponentiation(base, &exponent.half(), modul);
    return if exponent.is_even() {
        // Ist der Exponent gerade, so wird nur quadriert.
        base_to_square.pow(2).rem_euclid(modul)
    } else {
        // Ist der Exponent ungerade, wird die Basis erneut als Faktor herangezogen.
        (base_to_square.pow(2) * base).rem_euclid(modul)
    };
}

/// Berechnet das Inverse-Element in einem Restklassenring.
///
/// Das Inverse-Element einer Zahl `n` im Restklassenring modulo `modul` ist
/// eine andere Zahl `x`, so dass `(n * x) % modul = 1`
/// (das neutrale Element der Multiplikation).
///
/// # Argumente
/// * `n` - Die zu invertierende Zahl.
/// * `modul` - Die Modulo-Zahl, gegen die die Inversion durchgeführt wird.
///
/// # Rückgabe
/// * Result<inverse, Error>
/// Das Inverse-Element von `n` im Restklassenring modulo `modul`.
/// Wenn keine Inverse existiert (wenn `n` und `modul` nicht teilerfremd sind),
/// wird ein Error zurückgegeben.
pub fn modulo_inverse(n: &BigInt, modul: &BigInt) -> Result<BigInt, Error> {
    let (ggt, _x, y) = extended_euclid(modul, n);
    // Wenn ggT nicht 1, existiert kein Inverse. -> Error
    if !ggt.is_one() {
        let no_inverse_error = Error::new(ErrorKind::InvalidInput, format!("n hat keinen Inverse"));
        return Err(no_inverse_error);
    }
    // Berechnet aus den letzten Faktoren das Inverse.
    return Ok((modul + y).rem_euclid(modul));
}

/// Implementiert den erweiterten euklidischen Algorithmus.
///
/// Der erweiterte euklidische Algorithmus wird verwendet, um das Inverse-Element
/// einer Zahl in einem Restklassenring zu finden. Er arbeitet rekursiv und berechnet
/// die Faktoren `x` und `y` in der Bézout'schen Identität, so dass `x * n + y * modul = ggT(n, modul)`
///
/// # Argumente
/// * `n` - Die Zahl, welche mit dem Modul verechnet werden soll.
/// * `modul` - Die Modulo-Zahl, gegen die der Algorithmus durchgeführt wird.
///
/// # Rückgabe
/// * (ggT(n,modul),x,y)
/// Ein tripel aus dem groessten gemeinsamen Teiler einer Zahl `n` und dem `modul`,
/// sowie den zwei Faktoren `x` und `y`.
pub fn extended_euclid(n: &BigInt, modul: &BigInt) -> (BigInt, BigInt, BigInt) {
    //rotierendes Array, zur Berechnung und Speicherung der Faktoren `x` und `y`
    let xy = [
        BigInt::one(),
        BigInt::one(),
        BigInt::one(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::one(),
    ];
    return extended_euclidean_algorithm(&n, &modul, xy);
}

fn extended_euclidean_algorithm(
    n: &BigInt,
    modul: &BigInt,
    mut xy: [BigInt; 6],
) -> (BigInt, BigInt, BigInt) {
    xy.rotate_left(2);

    return if modul.is_zero() {
        (n.clone(), xy[0].clone(), xy[1].clone())
    } else {
        // Berechnet die Faktoren und speichert sie in einem rotierenden Array.
        let div = n / modul;
        xy[4] = &xy[0] - (&div * &xy[2]);
        xy[5] = &xy[1] - (&div * &xy[3]);
        extended_euclidean_algorithm(modul, &n.rem_euclid(modul), xy)
    };
}

/// Führt den Miller-Rabin-Primzahltest auf `p` mit `repeats` Runden aus.
///
/// # Argumente
/// * `p` - Die zu testende Zahl >= 11.
/// * `repeats` - Die Anzahl der Testrunden (Je mehr Runden, desto zuverlässiger).
///
/// # Rückgabe
/// `true`, wenn `p` wahrscheinlich eine Primzahl ist, andernfalls `false`.
///
/// Wahrscheinlichkeit: >= 1 - (1/4)^repeats
///
/// # Beispiel
/// ```
/// miller_rabin(11, 40) // => true
/// miller_rabin(2211, 40) // => false
/// ```
pub fn miller_rabin(p: &BigInt, repeats: usize) -> bool {
    let mut d = p.decrement();
    let mut s = BigInt::zero();

    while d.is_even() {
        d.half_assign();
        s.increment_assign();
    }

    let mut rand = RandomElsner::new(&big_i!(2), &p);

    for _ in 0..repeats {
        let mut a = rand.take();
        while p.is_divisible_by(&a) {
            a = rand.take();
        }
        if !miller_rabin_test(p, &s, &d, &a) {
            return false;
        }
    }

    return true;
}

fn miller_rabin_test(p: &BigInt, s: &BigInt, d: &BigInt, a: &BigInt) -> bool {
    let mut x = fast_exponentiation(a, d, p);

    if x.is_one() || x == p.decrement() {
        return true;
    }

    let mut r = BigInt::zero();

    while &r < s {
        x = fast_exponentiation(&x, &big_i!(2u8), p);
        if x == p.decrement() {
            return true;
        }
        r.increment_assign();
    }
    return false;
}
