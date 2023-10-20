use ibig::{ubig, UBig};
use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;

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
pub(crate) fn ubig_to_u32(value: &UBig) -> u32 {
    //todo -- einschränken, dass die ubig zahl nicht über den werteraum von u32 geht
    let value_str = format!("{}", value);
    value_str.parse::<u32>().unwrap()
}
