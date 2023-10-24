use bigdecimal::num_bigint::BigUint;
use bigdecimal::{One, Zero};
use rand::random;

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
/// Gibt eine Zufallszahl im Bereich 2..high zurück.
///
pub fn random_in_range(high: &BigUint) -> BigUint {
    // let high_len = high.bit_len();
    //
    // let mut rng = thread_rng();
    // let bernoulli = Bernoulli::new(0.5).unwrap();
    // let mut random_bool_iter = bernoulli.sample_iter(&mut rng).take(high_len - 2);
    //
    // let mut result = BigUint::from(2);
    // for i in 2..high_len {
    //     if random_bool_iter.next().unwrap() {
    //         result.set_bit(i);
    //         if &result > high {
    //             result.clear_bit(i);
    //         }
    //     }
    // }
    // result
    BigUint::from(30u8) // TODO Dummy-Wert
}

///
/// Gibt eine Zufallszahl im Bereich a..b zurück.
///
/// TODO: auf BigUint umwandeln.
/// eventuell schwierig weil decimal Zahl mit ganzer Zahl multipliziert werden muss

pub fn elsner_rand(a: f64, b: f64) -> f64 {
    let mut m: f64 = random::<u32>() as f64;

    // Für m brauchen wir die doppelte Präzision
    // Wie groß ist die Intervallbreite? Diese Anzahl von Dezimalstellen von (b-a+1) und
    // Dezimalstellen von n addieren und das die Präzision. + 10 mehr gegen rundungsfehler
    while (m.sqrt() == 0.0) {
        m = random::<u32>() as f64;
    }
    let n: f64 = random::<u32>() as f64;
    let s: f64 = a + ((n * m.sqrt() % 1.0) * (b - a + 1.0)).floor();
    return s;
}

///
/// Konvertiere ein Zeichen in einen u32 Code -- z.B. für Blockchiffre
///
pub(crate) fn char_to_u32(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32,
        'A'..='Z' => c as u32 - 'A' as u32 + 26,
        '0'..='9' => c as u32 - '0' as u32 + 52,
        '.' => 62,
        ',' => 63,
        ':' => 64,
        ';' => 65,
        '-' => 66,
        '!' => 67,
        '?' => 68,
        '"' => 69,
        '(' => 70,
        ')' => 71,
        '/' => 72,
        '\'' => 73,
        '*' => 74,
        '+' => 75,
        ' ' => 76,
        '&' => 77,
        '%' => 78,
        '$' => 79,
        '#' => 80,
        '@' => 81,
        '€' => 82,
        '§' => 83,
        '°' => 84,
        _ => panic!("Ungültiges Zeichen: {}", c),
    }
}

///
/// Konvertiere ein u32 Code in ein Zeichen -- z.B. für Blockchiffre
///
pub(crate) fn u32_to_char(value: u32) -> char {
    match value {
        0..=25 => ((value + 'a' as u32) as u8) as char,
        26..=51 => (((value - 26) + 'A' as u32) as u8) as char,
        52..=61 => (((value - 52) + '0' as u32) as u8) as char,
        62 => '.',
        63 => ',',
        64 => ':',
        65 => ';',
        66 => '-',
        67 => '!',
        68 => '?',
        69 => '"',
        70 => '(',
        71 => ')',
        72 => '/',
        73 => '\'',
        74 => '*',
        75 => '+',
        76 => ' ',
        77 => '&',
        78 => '%',
        79 => '$',
        80 => '#',
        81 => '@',
        82 => '€',
        83 => '§',
        84 => '°',
        _ => panic!("Ungültiger Wert: {}", value),
    }
}

///
/// wandle eine ubig Zahl in einen u32 Wert um
///
pub(crate) fn ubig_to_u32(value: &BigUint) -> u32 {
    //todo -- einschränken, dass die ubig zahl nicht über den werteraum von u32 geht
    let value_str = format!("{}", value);
    value_str.parse::<u32>().unwrap()
}
