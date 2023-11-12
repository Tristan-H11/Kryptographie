use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};
use log::{debug, trace};

use crate::big_i;

///
/// Methode, um einen String in eine Menge von gleich großen Blöcken in Dezimalform zu unterteilen.
///
/// # Argumente
/// * `m` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
/// * `fill_blocks` - Gibt an, ob die Blöcke mit Leerzeichen aufgefüllt werden sollen.
///
/// # Rückgabe
/// * `Vec<BigUint>` - Die codierte Darstellung des Strings als vec der Summen.
///
pub(crate) fn create_blocks_from_string_encrypt(
    m: &str,
    block_size: usize,
    fill_blocks: bool,
    g_base: u32,
) -> Vec<BigInt> {
    debug!("Erstelle Chiffre mit Blockgröße {} für {}", block_size, m);
    let b = split_into_blocks(m, block_size, fill_blocks);
    let i_vec = string_to_int_vec(b);
    to_sum_vec(i_vec, &big_i!(g_base))
}

///
/// Methode, um einen String in eine RSA Verschlüsselte Message in gleich große Blöcke zu splitten
/// und dann in deren Dezimalform umzuwandeln
///
/// # Argumente
/// * `m` - Der zu unterteilende String.
/// --> das m besteht aus 2 Teilen, max_block_size und rsa_encrypted_message; getrennt durch \u{FE8D}
/// * `fill_blocks` - Gibt an, ob die Blöcke mit Leerzeichen aufgefüllt werden sollen.
///
/// # Rückgabe
/// * `Vec<BigUint>` - Die codierte Darstellung des Strings als vec der Summen.
///
pub(crate) fn create_blocks_from_string_decrypt(
    m: &str,
    fill_blocks: bool,
    g_base: u32,
    block_size: usize,
) -> Vec<BigInt> {
    let message = m;
    debug!(
        "Erstelle Chiffre mit Blockgröße {} für {}",
        block_size, message
    );

    let b = split_into_blocks(message, block_size, fill_blocks);
    let i_vec = string_to_int_vec(b);
    to_sum_vec(i_vec, &big_i!(g_base))
}

///
/// Methode, um eine Menge von gleich großen Blöcken in Dezimalform, in einen String zu überführen.
///
/// # Argumente
/// * `sums` - Die zu überführenden Summen.
///
/// # Rückgabe
/// * `String` - Der decodierte String.
///
pub(crate) fn create_string_from_blocks_encrypt(
    sums: Vec<BigInt>,
    target_size: usize,
    g_base: u32,
) -> String {
    debug!(
        "Erstelle String aus Vektor von Summen: Anzahl der Vectorblöcke --> {}",
        sums.len()
    );
    let strings = sums_vec_to_string_vec(sums, &big_i!(g_base));
    debug!("Chiffrierter Vector: {:?}", strings);

    // Füllt jeden String vorne mit "0", um die maximale Länge zu erreichen
    // -- ziel ist es, eine einheitliche blocksize zu erhalten
    let strings: Vec<String> = strings
        .iter()
        .map(|s| {
            format!(
                "{}{}",
                "\u{0}".repeat(target_size - s.chars().count()),
                s
            )
        })
        .collect();
    strings.join("")
}

///
/// Methode, um eine Menge von gleich großen Blöcken in Dezimalform in einen String zu überführen.
///
/// # Argumente
/// * `sums` - Die zu überführenden Summen.
///
/// # Rückgabe
/// * `String` - Der decodierte String.
///
pub(crate) fn create_string_from_blocks_decrypt(sums: Vec<BigInt>, g_base: u32) -> String {
    debug!(
        "Erstelle String aus Vektor von Summen: Anzahl der Vectorblöcke --> {}",
        sums.len()
    );
    let strings = sums_vec_to_string_vec(sums, &big_i!(g_base));
    debug!("Chiffrierter Vector: {:?}", strings);

    let result = strings.join("");
    result.trim_end().to_string()
}

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um einen String in eine Menge von gleich großen Blöcken zu unterteilen.
/// Nicht-volle Blöcke werden mit Space (' ') aufgefüllt.
///
/// # Argumente
/// * `message` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
/// * `fill_block` - Gibt an, ob die Blöcke mit Leerzeichen aufgefüllt werden sollen.
///
/// # Rückgabe
/// * `Vec<String>` - Die Menge der Blöcke als Vector.
///
/// # Beispiel
/// Beispiel von Seite 20 IT-Sec Skript:
/// ```
/// split_into_blocks("Das ist eine Testnachricht", 4)
/// ["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
/// ```
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

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um den Vector mit seinen Strings in einen Vector mit Integern zu überführen.
///
/// # Argumente
/// * `b_vec` - Der zu überführende Vec<String>.
///
/// # Rückgabe
/// * `Vec<Vec<u32>>` - Die codierte Darstellung des Strings als integer.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
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

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um einen Vektor von Integern als g-adische Zahl zu interpretieren
/// und in eine Dezimalzahl zu überführen.
///
/// # Argumente
/// * `d_vec` - Der zu überführende Vec<Vec<u32>>.
///
/// # Rückgabe
/// * `Vec<BigUint>` - Die codierte Darstellung des Strings als vec der Summen.
/// vec![
///             big_u!(19140715035688992u64),
///             big_u!(29555366483460128u64),
///             big_u!(28429423626551397u64),
///             big_u!(9007560038613107u64),
///             big_u!(32651569751195747u64),
///             big_u!(29273887211061347u64),
///             big_u!(29273895796211744u64),
///         ];
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
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

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um eine Dezimalzahl in einen String (g-adisch) zu überführen.
///
/// # Argumente
/// * `sums` - Der zu überführende Vec<BigUint>.
/// * `base` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `String` - Vector der Strings.
///         let expected_result = vec![
///             "Das ".to_string(),
///             "ist ".to_string(),
///             "eine".to_string(),
///             " Tes".to_string(),
///             "tnac".to_string(),
///             "hric".to_string(),
///             "ht  ".to_string(),
///         ];
///
///
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

///
/// Konvertiere ein u32 Code in ein Zeichen -- z.B. für Blockchiffre
///
pub(crate) fn u32_to_c(value: u32) -> char {
    match char::from_u32(value) {
        Some(x) => x,
        None => panic!("oben Ungültiger u32 Wert: {}", value),
    }
}

///
/// wandle eine ubig Zahl in einen u32 Wert um
///
pub(crate) fn big_int_to_u32(value: &BigInt) -> u32 {
    let value_str = format!("{}", value);
    match value_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => panic!("unten Ungültiger u32 Wert: {}", value),
    }
}
