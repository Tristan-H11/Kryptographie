use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};
use log::{debug, trace};

use crate::big_i;

/// Diese Methode erzeugt einen Vektor mit BigInts, der aus einem String mit einer
/// bestimmten Blockgröße erstellt wurde.
///
/// # Argumente
/// * `m`: Der String, der in Blöcke unterteilt werden soll.
/// * `block_size`: Die Größe der Blöcke.
/// * `fill_blocks`: Gibt an, ob der letzte Block mit Leerzeichen aufgefüllt werden sollen.
/// * `g_base`: Die Basis, in der die Blöcke kodiert werden sollen.
///
/// # Rückgabe
/// * Ein Vektor mit BigInts, der den g-adisch entwickelten Text enthält.
pub(crate) fn encode_string_to_blocks(
    m: &str,
    block_size: usize,
    fill_blocks: bool,
    g_base: u32,
) -> Vec<BigInt> {
    debug!("Erstelle Vektor von Blocksummen mit Blockgröße {} und Basis {}", block_size, g_base);
    let b = split_into_blocks(m, block_size, fill_blocks);
    let i_vec = string_to_int_vec(b);
    to_sum_vec(i_vec, &big_i!(g_base))
}

/// Diese Methode erzeugt einen String aus einem Vektor mit BigInts, die bereits verschlüsselt
/// wurden. Die Blöcke werden dabei ggf mit führenden Nullen aufgefüllt, um eine einheitliche Länge
/// zu erreichen.
///
/// # Argumente
/// * `sums`: Der Vektor mit BigInts.
/// * `target_size`: Die gewünschte Länge der einzelnen Blöcke.
/// * `g_base`: Die Basis, in der die Blöcke kodiert wurden.
///
/// # Rückgabe
/// * Der resultierende String.
pub(crate) fn create_string_from_blocks_encrypt(
    sums: Vec<BigInt>,
    target_size: usize,
    g_base: u32,
) -> String {
    debug!("Erstelle String aus Vektor von Summen. Vektorgröße: {}", sums.len());
    let strings = sums_vec_to_string_vec(sums, &big_i!(g_base));
    debug!("Chiffrierter Vector: {:?}", strings);

    // Füllt jeden String vorne mit "0", um die maximale Länge zu erreichen
    let strings: Vec<String> = strings
        .iter()
        .map(|s| format!("{}{}", "\u{0}".repeat(target_size - s.chars().count()), s))
        .collect();
    strings.join("")
}

/// Methode, um eine Menge von gleich großen Blöcken in Dezimalform in einen String zu überführen.
/// Entfernt Leerzeichen am Ende.
///
/// # Argumente
/// * `sums` - Die zu überführenden Summen.
///
/// # Rückgabe
/// * Der resultierende String.
pub(crate) fn create_string_from_blocks_decrypt(sums: Vec<BigInt>, g_base: u32) -> String {
    debug!("Erstelle String aus Vektor von Summen. Vektorgröße: {}", sums.len());
    let strings = sums_vec_to_string_vec(sums, &big_i!(g_base));
    debug!("Chiffrierter Vector: {:?}", strings);

    let result = strings.join("");
    result.trim_end().to_string()
}

/// Diese Methode teilt einen String in Blöcke mit einer bestimmten Größe auf.
///
/// # Argumente
/// * `message`: Der String, der in Blöcke unterteilt werden soll.
/// * `block_size`: Die Größe der Blöcke.
/// * `fill_block`: Gibt an, ob der letzte Block mit Leerzeichen aufgefüllt werden soll.
///
/// # Rückgabe
/// * Ein Vektor mit Strings, der die Blöcke enthält.

pub(crate) fn split_into_blocks(message: &str, block_size: usize, fill_block: bool) -> Vec<String> {
    debug!(
        "Erstelle Blöcke mit Blockgröße {} für '{}'",
        block_size, message
    );
    message
        .chars()
        .collect::<Vec<char>>()
        .chunks(block_size)
        .map(|c| {
            let mut b = c.iter().collect::<String>();
            if fill_block {
                while b.len() < block_size {
                    b.push(' '); // Fügt Leerzeichen hinzu, um den letzten Block zu füllen
                }
            }
            trace!("Erstellte Block '{}'", b);
            b
        })
        .collect()
}

/// Diese Methode konvertiert einen Vektor von Strings in einen Vektor von Integers.
/// Dabei wird jeder Char im String in einen Integer konvertiert und der Integer wird dem
/// resultierenden Vektor hinzugefügt.
///
/// # Argumente
/// * `b_vec`: Der Vektor von Strings, der konvertiert werden soll.
///
/// # Rückgabe
/// * Ein Vektor von Integers, der die konvertierten Strings enthält.
///
/// # Beispiel
/// ```rust
/// string_to_int_vec("["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]")
/// vec![
///             vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')],
///             vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')],
///             vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')],
///             vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')],
///             vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')],
///             vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')],
///             vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')],
///         ];
/// ```
///
pub(crate) fn string_to_int_vec(b_vec: Vec<String>) -> Vec<Vec<u32>> {
    debug!("Erstelle Integer Vektor aus String Vektor");
    b_vec
        .into_iter()
        .map(|b| {
            let vec = b.chars().map(|b| b as u32).collect();
            trace!("Erstelle Integer Vektor aus String Vektor: {:?}", vec);
            vec
        })
        .collect()
}

/// Diese Methode überführt einen Vektor von Vektoren in einen Vektor von den Summen der Vektoren.
/// Die Summen sind dabei in einem g-adischen System zu Basis base kodiert.
///
/// # Argumente
/// * `d_vec`: Der Vektor von Vektoren, der überführt werden soll.
/// * `base`: Die Basis, in der die Summen gebildet werden sollen.
///
/// # Rückgabe
/// * Ein Vektor von BigInts, der die Summen enthält.
pub(crate) fn to_sum_vec(d_vec: Vec<Vec<u32>>, base: &BigInt) -> Vec<BigInt> {
    debug!("Erstelle Summen Vektor aus Integer Vektor");
    d_vec
        .into_iter()
        .map(|d| helper_fun_sum_for_digits(&d, base))
        .collect()
}

fn helper_fun_sum_for_digits(i_vec: &Vec<u32>, g_base: &BigInt) -> BigInt {
    debug!("Erstelle Summe aus Integer Vektor");

    let (sum, _) = i_vec.iter().rev().fold(
        (BigInt::zero(), BigInt::one()),
        |(acc_sum, acc_base), &digit| {
            trace!("Addiere {} * {} zu Summe", acc_base, digit);
            (&acc_sum + &acc_base * big_i!(digit), acc_base * g_base)
        },
    );

    debug!("Summe: {}", sum);
    sum
}

/// Diese Methode überführt einen Vektor von BigInts in einen Vektor von Strings.
/// Dabei wird jeder BigInt in ein g-adisches System zu Basis base konvertiert.
///
/// # Argumente
/// * `sums`: Der Vektor von BigInts, der überführt werden soll.
/// * `base`: Die Basis, in der die Summen gebildet wurden.
///
/// # Rückgabe
/// * Ein Vektor von Strings, der die konvertierten Summen enthält.
pub(crate) fn sums_vec_to_string_vec(sums: Vec<BigInt>, base: &BigInt) -> Vec<String> {
    sums.into_iter()
        .map(|sum| helper_fun_sum_to_string(&sum, base))
        .collect()
}

fn helper_fun_sum_to_string(sum: &BigInt, base: &BigInt) -> String {
    let mut t_sum = sum.clone();
    let mut res = String::new();
    let zero = BigInt::zero();

    // Konvertiere die Summe in ein g-adisches System zu Basis base
    while t_sum > zero {
        let digit = &t_sum % base;
        trace!("{} % {} = {} ", t_sum, base, digit);
        t_sum = &t_sum / base;
        let char = u32_to_c(big_int_to_u32(&digit));
        trace!("--> {}\n", char);
        res.push(char);
    }
    res.chars().rev().collect()
}

/// Diese Methode konvertiert einen u32 Wert in einen char.
///
/// # Argumente
/// * `value`: Der u32 Wert, der konvertiert werden soll.
///
/// # Rückgabe
/// * Der resultierende char.
pub(crate) fn u32_to_c(value: u32) -> char {
    match char::from_u32(value) {
        Some(x) => x,
        None => panic!("oben Ungültiger u32 Wert: {}", value),
    }
}

/// Diese Methode konvertiert einen BigInt Wert in einen u32.
/// Panics, wenn der Wert nicht in einen u32 konvertiert werden kann.
///
/// # Argumente
/// * `value`: Der BigInt Wert, der konvertiert werden soll.
///
/// # Rückgabe
/// * Der resultierende u32 Wert.
pub(crate) fn big_int_to_u32(value: &BigInt) -> u32 {
    let value_str = format!("{}", value);
    match value_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => panic!("unten Ungültiger u32 Wert: {}", value),
    }
}
