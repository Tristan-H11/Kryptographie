use ibig::{ubig, UBig};
use rand::thread_rng;
use rand::distributions::{Bernoulli, Distribution};

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

///
/// Inkrementiert die übergebene Zahl.
///
pub fn increment(a: &UBig) -> UBig {
    a + ubig!(1)
}

///
/// Dekrementiert die übergebene Zahl.
///
pub fn decrement(a: &UBig) -> UBig {
    a - ubig!(1)
}

///
/// Gibt eine Zufallszahl im Bereich 2..high zurück.
///
pub fn random_in_range(high: &UBig) -> UBig {
    let high_len = high.bit_len();

    let mut rng = thread_rng();
    let bernoulli = Bernoulli::new(0.5).unwrap();
    let mut random_bool_iter = bernoulli.sample_iter(&mut rng).take(high_len - 2);

    let mut result = ubig!(2);
    for i in 2..high_len {
        if random_bool_iter.next().unwrap() {
            result.set_bit(i);
            if &result > high {
                result.clear_bit(i);
            }
        }
    }
    result
}