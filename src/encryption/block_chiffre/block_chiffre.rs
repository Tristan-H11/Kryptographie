use crate::math_core::traits::logarithm::Logarithm;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, ToPrimitive, Zero};
use log::{debug, trace};
use num::Integer;

/// Bestimmt die Blockgröße für die Verschlüsselung und Entschlüsselung.
/// Die Blockgröße ist die Anzahl der Zeichen, die in einem Block verschlüsselt werden.
///
/// # Argumente
/// * `modulus`: Das Modulus des RSA-Schlüssels.
/// * `base`: Die Basis, in der die Blöcke kodiert werden sollen.
/// * `is_encryption`: Gibt an, ob die Blockgröße für die Verschlüsselung oder Entschlüsselung bestimmt werden soll.
///
/// # Rückgabe
/// * Die Blockgröße.
pub(crate) fn determine_block_size(modulus: &BigInt, base: &BigInt, is_encryption: bool) -> usize {
    // TODO Aufhübschen, wenn das Blockchiffre refactored wird.
    debug!("Bestimme Blockgröße");

    let block_size = match is_encryption {
        true => modulus.log(base),
        false => modulus.log(base) + 1,
    };

    debug!("Blockgröße bestimmt als: {}", block_size);
    block_size
}

/// Diese Methode erzeugt einen Vektor mit BigInts, der aus einem String mit einer bestimmten Blockgröße erstellt wurde.
///
/// # Argumente
/// * `m`: Der String, der in Blöcke unterteilt werden soll.
/// * `block_size`: Die Größe der Blöcke.
/// * `g_base`: Die Basis, in der die Blöcke kodiert werden sollen.
///
/// # Rückgabe
/// * Ein Vektor mit BigInts, der den g-adisch entwickelten Text enthält.
pub(crate) fn encode_string_to_blocks(m: &str, block_size: usize, g_base: u32) -> Vec<BigInt> {
    debug!(
        "Erstelle Vektor von Blocksummen mit Blockgröße {} und Basis {}",
        block_size, g_base
    );
    let b = split_into_blocks(m, block_size);
    let i_vec = string_to_int_vec(b);
    to_sum_vec(i_vec, &g_base.into())
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
    debug!(
        "Erstelle String aus Vektor von Summen. Vektorgröße: {}",
        sums.len()
    );

    let mut result = String::new();
    for sum in sums {
        let string = helper_fun_sum_to_string(&sum, &g_base.into());
        debug!("Chiffrierter Vector: {:?}", string);

        // Füllt jeden String vorne mit "0", um die maximale Länge zu erreichen
        let padded_string = format!(
            "{}{}",
            "\u{0}".repeat(target_size - string.chars().count()),
            string
        );
        result.push_str(&padded_string);
    }

    result
}

/// Methode, um eine Menge von gleich großen Blöcken in Dezimalform in einen String zu überführen.
///
/// # Argumente
/// * `sums` - Die zu überführenden Summen.
///
/// # Rückgabe
/// * Der resultierende String.
pub(crate) fn create_string_from_blocks_decrypt(sums: Vec<BigInt>, g_base: u32) -> String {
    debug!(
        "Erstelle String aus Vektor von Summen. Vektorgröße: {}",
        sums.len()
    );
    let strings = sums_vec_to_string_vec(sums, &g_base.into());
    debug!("Chiffrierter Vector: {:?}", strings);

    let result = strings.join("");
    result
}

/// Diese Methode teilt einen String in Blöcke mit einer bestimmten Größe auf.
///
/// # Argumente
/// * `message`: Der String, der in Blöcke unterteilt werden soll.
/// * `block_size`: Die Größe der Blöcke.
///
/// # Rückgabe
/// * Ein Vektor mit Strings, der die Blöcke enthält.
fn split_into_blocks(message: &str, block_size: usize) -> Vec<String> {
    debug!(
        "Erstelle Blöcke mit Blockgröße {} für '{}'",
        block_size, message
    );
    message
        .chars()
        .collect::<Vec<char>>()
        .chunks(block_size)
        .map(|c| {
            let b = c.iter().collect::<String>();
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
///     vec![char_to_u32('D'), char_to_u32('a'), char_to_u32('s'), char_to_u32(' ')],
///     vec![char_to_u32('i'), char_to_u32('s'), char_to_u32('t'), char_to_u32(' ')],
///     vec![char_to_u32('e'), char_to_u32('i'), char_to_u32('n'), char_to_u32('e')],
///     vec![char_to_u32(' '), char_to_u32('T'), char_to_u32('e'), char_to_u32('s')],
///     vec![char_to_u32('t'), char_to_u32('n'), char_to_u32('a'), char_to_u32('c')],
///     vec![char_to_u32('h'), char_to_u32('r'), char_to_u32('i'), char_to_u32('c')],
///     vec![char_to_u32('h'), char_to_u32('t'), char_to_u32(' '), char_to_u32(' ')],
/// ];
/// ```
///
fn string_to_int_vec(b_vec: Vec<String>) -> Vec<Vec<u32>> {
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
fn to_sum_vec(d_vec: Vec<Vec<u32>>, base: &BigInt) -> Vec<BigInt> {
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
            let product: BigInt = &acc_base * digit;
            (&acc_sum + product, acc_base * g_base)
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
fn sums_vec_to_string_vec(sums: Vec<BigInt>, base: &BigInt) -> Vec<String> {
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
        let (quotient, digit) = t_sum.div_rem(base);
        trace!("{} % {} = {} ", t_sum, base, digit);
        t_sum = quotient;
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
fn u32_to_c(value: u32) -> char {
    match char::from_u32(value) {
        Some(x) => x,
        None => panic!("oben Ungültiger u32 Wert: {}", value), //todo tristan
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
fn big_int_to_u32(value: &BigInt) -> u32 {
    match value.to_u32() {
        Some(digit) => digit,
        None => panic!("Ungültiger u32 Wert: {}", value), //todo tristan
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    ///
    /// Prüft, ob die Funktionen zum Zerteilen und Zusammensetzen eines String das Inverse voneinander sind.
    ///
    #[test]
    fn test_create_block_umkehrfunktion_create_string() {
        let m = "Da苉 ist eine Testnachricht";
        let block_size = 8;
        let _basis_length = 55296 as u32;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, 55296),
            55296,
        );
        assert_eq!(decoded, m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 6;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, 55296),
            55296,
        );
        assert_eq!(decoded, m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 47;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, 55296),
            55296,
        );
        assert_eq!(decoded, m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 3;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, 55296),
            55296,
        );
        assert_eq!(decoded, m);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Da苉 ist eine Testnachricht";
        let block_size = 7;
        let _basis_length = 55296;
        let result = encode_string_to_blocks(message, block_size, 55296);
        let expected_result: Vec<BigInt> = vec![
            BigInt::from(1943938337267550087026074257524u128),
            BigInt::from(914822981356602019800946507860u128),
            BigInt::from(2887304683313907978613082523752u128),
            BigInt::from(1065827572823258284148u128),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decrypt_chiffre() {
        let sums = vec![
            BigInt::from(1943938337267550087026074257524u128),
            BigInt::from(914822981356602019800946507860u128),
            BigInt::from(2887304683313907978613082523752u128),
            BigInt::from(3258925137110102081877384560672u128),
        ];
        let result = create_string_from_blocks_decrypt(sums, 55296);
        // Ja, die Leerzeichen sind so gewollt. Die sind in der Summe oben enthalten.
        let expected_result = "Da苉 ist eine Testnachricht  ".to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 4 aufgeteilt.
        let message = String::from("Da苉 ist eine Testnachricht");
        let block_size = 4;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(
            result,
            vec!["Da苉 ", "ist ", "eine", " Tes", "tnac", "hric", "ht"]
        );

        // Testfall 2: Ein String, der bereits eine Blockgröße hat, wird nicht verändert,
        // es kommt kein neuer leerer Block dazu.
        let message = String::from("123AB");
        let block_size = 5;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["123AB"]);

        // Testfall 3: Ein leerer String wird in Blöcke der Größe 3 aufgeteilt.
        let message = String::from("   ");
        let block_size = 3;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["   "]);

        // Testfall 4: Ein String wird in Blöcke der Größe 1 aufgeteilt.
        let message = String::from("abcdef");
        let block_size = 1;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_string_to_int_vec() {
        // Ja, das Leerzeichen am Ende ist gewollt.
        let message = "Da苉 ist eine Testnachricht ";
        let blocks = split_into_blocks(&message, 4);
        let expected = vec![
            vec!['D' as u32, 'a' as u32, '苉' as u32, ' ' as u32],
            vec!['i' as u32, 's' as u32, 't' as u32, ' ' as u32],
            vec!['e' as u32, 'i' as u32, 'n' as u32, 'e' as u32],
            vec![' ' as u32, 'T' as u32, 'e' as u32, 's' as u32],
            vec!['t' as u32, 'n' as u32, 'a' as u32, 'c' as u32],
            vec!['h' as u32, 'r' as u32, 'i' as u32, 'c' as u32],
            vec!['h' as u32, 't' as u32, ' ' as u32],
        ];
        let result = string_to_int_vec(blocks);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        let digit_vectors = vec![
            vec!['D' as u32, 'a' as u32, '苉' as u32, ' ' as u32],
            vec!['i' as u32, 's' as u32, 't' as u32, ' ' as u32],
            vec!['e' as u32, 'i' as u32, 'n' as u32, 'e' as u32],
            vec![' ' as u32, 'T' as u32, 'e' as u32, 's' as u32],
            vec!['t' as u32, 'n' as u32, 'a' as u32, 'c' as u32],
            vec!['h' as u32, 'r' as u32, 'i' as u32, 'c' as u32],
            vec!['h' as u32, 't' as u32, ' ' as u32, ' ' as u32],
        ];

        let base = 55296.into();
        let result = to_sum_vec(digit_vectors, &base);

        let expected_result = vec![
            BigInt::from(11497444858239008u64),
            BigInt::from(17753298306195488u64),
            BigInt::from(17076964999090277u64),
            BigInt::from(5410678690363507u64),
            BigInt::from(19613115525224547u64),
            BigInt::from(17584219565365347u64),
            BigInt::from(17584225676623904u64),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_to_strings() {
        let sums = vec![
            BigInt::from(11497444858239008u64),
            BigInt::from(17753298306195488u64),
            BigInt::from(17076964999090277u64),
            BigInt::from(5410678690363507u64),
            BigInt::from(19613115525224547u64),
            BigInt::from(17584219565365347u64),
            BigInt::from(17584225676623904u64),
        ];

        let base = 55296.into();
        let result = sums_vec_to_string_vec(sums, &base);

        let expected_result = vec![
            "Da苉 ".to_string(),
            "ist ".to_string(),
            "eine".to_string(),
            " Tes".to_string(),
            "tnac".to_string(),
            "hric".to_string(),
            "ht  ".to_string(),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_join_strings() {
        let input = vec![
            "Da苉 ".to_string(),
            "ist ".to_string(),
            "eine".to_string(),
            " Tes".to_string(),
            "tnac".to_string(),
            "hric".to_string(),
            "ht  ".to_string(),
        ];

        let result = input.join("");

        let expected_result = "Da苉 ist eine Testnachricht  ".to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_char_to_u32() {
        assert_eq!('a' as u32, 97); // Unicode
        assert_eq!('b' as u32, 98);
        assert_eq!('z' as u32, 122);
        assert_eq!('A' as u32, 65);
        assert_eq!('B' as u32, 66);
        assert_eq!('Z' as u32, 90);
        assert_eq!('0' as u32, 48);
        assert_eq!('1' as u32, 49);
        assert_eq!('9' as u32, 57);
    }

    #[test]
    fn test_u32_to_char() {
        assert_eq!(u32_to_c(97), 'a');
        assert_eq!(u32_to_c(122), 'z');
        assert_eq!(u32_to_c(65), 'A');
        assert_eq!(u32_to_c(90), 'Z');
        assert_eq!(u32_to_c(48), '0');
        assert_eq!(u32_to_c(57), '9');
        assert_eq!(u32_to_c(46), '.');
        assert_eq!(u32_to_c(44), ',');
        assert_eq!(u32_to_c(0), '\u{0}');
    }

    #[test]
    fn test_big_int_to_u32() {
        assert_eq!(big_int_to_u32(&12345.into()), 12345);
        assert_eq!(big_int_to_u32(&0.into()), 0);
    }

    #[test]
    #[should_panic]
    fn test_big_int_to_u32_panics() {
        big_int_to_u32(&BigInt::from_str("123456789012345678901234567890").unwrap());
    }
}
