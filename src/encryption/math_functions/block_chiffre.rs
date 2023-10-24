use crate::encryption::math_functions::big_int_util::{c_to_u16, u16_to_c, ubig_to_u16};
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{One, Zero};


pub(crate) fn create_chiffre(m: &str, b_size: usize) -> Vec<BigUint> {
    let b = create_b_vec(m, b_size);
    let i_vec = s_to_i_vec(b);
    to_sum_vec(i_vec)
}

pub(crate) fn decode_chiffre(sums: Vec<BigUint>) -> String {
    let s_vec = sums_vec_to_s_vec(sums);
    decode_s_vec(s_vec)
}

//todo -- alles was nicht create_chiffre und decode_chiffre ist, muss private sein
//(dann fliegen die tests gegen die Wand)
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
pub(crate) fn create_b_vec(m: &str, b_size: usize) -> Vec<String> {
    m
        .chars()
        .collect::<Vec<char>>() //Erstelle einen Vektor für die Blöcke bestehend aus Zeichen
        .chunks(b_size) //Definiert die Blockgröße im Vector
        .map(|c| {
            // Durchlaufe alle chunks, im letzten muss du ggf. Leerzeichen auffüllen
            let mut b = c.iter().collect::<String>(); // .iter --> füge chars zu String zusammen
            while b.len() < b_size {
                b.push(' '); // Fügt Leerzeichen hinzu, um den letzten Block zu füllen
            }
            b
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
pub(crate) fn s_to_i_vec(b_vec: Vec<String>) -> Vec<Vec<u16>> {
    b_vec.into_iter().map(|b| {
        b.chars().map(c_to_u16).collect()
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
pub(crate) fn to_sum_vec(d_vec: Vec<Vec<u16>>) -> Vec<BigUint> {
    d_vec.into_iter().map(|d| helper_fun_sum_for_digits(&d)).collect()
}

pub(crate) fn helper_fun_sum_for_digits(i_vec: &Vec<u16>) -> BigUint {
    let g_base = BigUint::from(2u32.pow(16));
    let mut sum = BigUint::zero();
    let mut base = BigUint::one();
    for &digit in i_vec.iter().rev() {
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
pub(crate) fn sums_vec_to_s_vec(sums: Vec<BigUint>) -> Vec<String> {
    sums.into_iter().map(|sum| helper_fun_sum_to_string(&sum)).collect()
}
pub(crate) fn helper_fun_sum_to_string(sum: &BigUint) -> String {
    let mut t_sum = sum.clone();
    let mut res = String::new();
    let base = BigUint::from(2u32.pow(16));
    let z = BigUint::zero();

    while t_sum > z {
        let remainder = ubig_to_u16(&(&t_sum % &base));
        res.push(u16_to_c(remainder));
        t_sum = t_sum / &base;
    }
    res.chars().rev().collect()
}

///
/// Erzeuge einen String aus dem Vector von Strings
///
pub(crate) fn decode_s_vec(s: Vec<String>) -> String {
    s.join("")
}

