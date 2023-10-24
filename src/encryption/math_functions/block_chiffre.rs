use crate::encryption::math_functions::big_int_util::{char_to_u16, u16_to_char, ubig_to_u16};
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{One, Zero};


pub(crate) fn create_chiffre(message: &str, block_size: usize) -> Vec<BigUint> {
    let blocks = split_into_blocks(message, block_size);
    let int_vecs = string_to_int_vec(blocks);
    digits_from_vec_to_sum(int_vecs)
}

pub(crate) fn decode_chiffre(sums: Vec<BigUint>) -> String {
    let strings_vec = sums_to_strings(sums);
    join_strings(strings_vec)
}

//todo -- alles was nicht create_chiffre und decode_chiffre ist, muss private sein
///
/// Methode, um einen String in eine Menge von gleich großen Blöcken zu unterteilen.
/// Nicht-volle Blöcke werden mit Space (' ') aufgefüllt.
///
/// # Argumente
/// * `message` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
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
pub(crate) fn split_into_blocks(message: &str, block_size: usize) -> Vec<String> {
    message
        .chars()
        .collect::<Vec<char>>() //Erstelle einen Vektor für die Blöcke bestehend aus Zeichen
        .chunks(block_size) //Definiert die Blockgröße im Vector
        .map(|chunk| {
            // Durchlaufe alle chunks, im letzten muss du ggf. Leerzeichen auffüllen
            let mut block = chunk.iter().collect::<String>(); // .iter --> füge chars zu String zusammen
            while block.len() < block_size {
                block.push(' '); // Fügt Leerzeichen hinzu, um den letzten Block zu füllen
            }
            block
        })
        .collect() // Fasst alle Blöcke im Vektor zusammen
}

///
/// Methode, um den Vector mit seinen Strings in einen Vector mit Integern zu überführen.
///
/// # Argumente
/// * `message` - Der zu überführende Vector mit seinen Strings.
///
/// # Rückgabe
/// * `Vec<Vec<u16>>` - Die codierte Darstellung des Strings als integer.
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
pub(crate) fn string_to_int_vec(blocks: Vec<String>) -> Vec<Vec<u16>> {
    blocks.into_iter().map(|block| {
        block.chars().map(char_to_u16).collect()
    }).collect()
}

///
/// Methode, um einen Vektor von Integern als g-adische Zahl zu interpretieren
/// und in eine Dezimalzahl zu überführen.
///
/// # Argumente
/// * `digits` - Der zu überführende Vec<Vec<u16>>.
///
/// # Rückgabe
/// * `BigUint` - Die Summe des g-adischen Systems als vec<u16> der Summen.
/// vec![
///             BigUint::from(19140715035688992u64),
///             BigUint::from(29555366483460128u64),
///             BigUint::from(28429423626551397u64),
///             BigUint::from(9007560038613107u64),
///             BigUint::from(32651569751195747u64),
///             BigUint::from(29273887211061347u64),
///             BigUint::from(29273895796211744u64),
///         ];
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
pub(crate) fn digits_from_vec_to_sum(digit_vectors: Vec<Vec<u16>>) -> Vec<BigUint> {
    digit_vectors.into_iter().map(|digits| helper_fun_sum_for_digits(&digits)).collect()
}

pub(crate) fn helper_fun_sum_for_digits(digits: &Vec<u16>) -> BigUint {
    let g_base = BigUint::from(2u32.pow(16));
    let mut sum = BigUint::zero();
    let mut base = BigUint::one();
    for &digit in digits.iter().rev() {
        sum += &base * BigUint::from(digit);
        base *= &g_base;
    }
    sum
}

///
/// Methode, um eine Dezimalzahl in einen String (g-adisch) zu überführen.
///
/// # Argumente
/// * `sum` - Die zu überführende Summe als vec der Summen.
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
pub(crate) fn sums_to_strings(sums: Vec<BigUint>) -> Vec<String> {
    sums.into_iter().map(|sum| helper_fun_sum_to_string(&sum)).collect()
}
pub(crate) fn helper_fun_sum_to_string(sum: &BigUint) -> String {
    let mut temp_sum = sum.clone();
    let mut result = String::new();
    let base = BigUint::from(2u32.pow(16));
    let zero = BigUint::zero();

    while temp_sum > zero {
        let remainder = ubig_to_u16(&(&temp_sum % &base));
        result.push(u16_to_char(remainder));
        temp_sum = temp_sum / &base;
    }
    result.chars().rev().collect()
}

///
/// Erzeuge einen String aus dem Vector von Strings
///
pub(crate) fn join_strings(strings: Vec<String>) -> String {
    strings.join("")
}

