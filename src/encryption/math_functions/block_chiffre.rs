use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};
use log::{debug, trace};
use num::Integer;

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
    debug!(
        "Erstelle Vektor von Blocksummen mit Blockgröße {} und Basis {}",
        block_size, g_base
    );
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
    debug!(
        "Erstelle String aus Vektor von Summen. Vektorgröße: {}",
        sums.len()
    );

    let mut result = String::new();
    for sum in sums {
        let string = helper_fun_sum_to_string(&sum, &big_i!(g_base));
        debug!("Chiffrierter Vector: {:?}", string);

        // Füllt jeden String vorne mit "0", um die maximale Länge zu erreichen
        let padded_string = format!("{}{}", "\u{0}".repeat(target_size - string.chars().count()), string);
        result.push_str(&padded_string);
    }

    result
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
    debug!(
        "Erstelle String aus Vektor von Summen. Vektorgröße: {}",
        sums.len()
    );
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

fn split_into_blocks(message: &str, block_size: usize, fill_block: bool) -> Vec<String> {
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
fn big_int_to_u32(value: &BigInt) -> u32 {
    let (_, remainder) = value.to_u32_digits();
    match remainder.first() {
        Some(&digit) => digit,
        None => panic!("Ungültiger u32 Wert: {}", value),
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::block_chiffre::{
        big_int_to_u32, create_string_from_blocks_decrypt, create_string_from_blocks_encrypt,
        encode_string_to_blocks, split_into_blocks, string_to_int_vec, sums_vec_to_string_vec,
        to_sum_vec, u32_to_c,
    };
    use crate::encryption::math_functions::number_theory::fast_exponentiation::FastExponentiation;
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    ///
    /// Test um zu prüfen, ob ein String aufgeteilt, manipuliert, zusammengesetzt und wieder umgekehrt werden kann.
    /// Dafür wird der String zerlegt, die Zahl verdoppelt und ein Ciphertext darauf erstellt.
    /// Dieser Cipher wird dann auch wieder zerlegt, die Zahl halbiert und ein Plaintext erstellt.
    /// Dieser Plaintext wird dann wieder zusammengesetzt und sollte dem ursprünglichen String entsprechen.
    ///
    #[test]
    #[ignore] //TODO: Fix this test
    fn test_loop_create_mult_decode_create_div_decode_1() {
        let mut failure_count = 0;

        for _ in 0..1 {
            let keygen_service = RsaKeygenService::new(256);
            let (public_key, private_key) = keygen_service.generate_keypair(1, 34, 55296, false); //TODO UseFast einbauen

            let message = "bbbbbbbbbbbbbbb  äääääääääääääää  !&    ";
            let _basis_length = 55296u32;

            let result = encode_string_to_blocks(message, public_key.get_block_size(), true, 55296)
                .iter()
                .map(|x| {
                    FastExponentiation::calculate(
                        x,
                        &public_key.get_e(),
                        &public_key.get_n(),
                        false,
                    )
                }) //TODO UseFast einbauen
                .collect::<Vec<BigInt>>();

            let encrypted_string =
                create_string_from_blocks_encrypt(result, public_key.get_block_size() + 1, 55296);

            let result = encode_string_to_blocks(
                &encrypted_string,
                private_key.get_block_size(),
                true,
                55296,
            ).iter()
            .map(|x| {
                FastExponentiation::calculate(x, &private_key.get_d(), &private_key.get_n(), false)
            }) //TODO UseFast einbauen
            .collect();

            let string = create_string_from_blocks_decrypt(result, 55296);

            if string.trim() != message.trim() {
                failure_count += 1;
            }
        }

        assert_eq!(failure_count, 0, "Fehlgeschlagene Tests: {}", failure_count);
    }

    ///
    /// Prüft, ob die Funktionen zum Zerteilen und Zusammensetzen eines String das Inverse voneinander sind.
    ///
    #[test]
    fn test_create_block_umkehrfunktion_create_string() {
        let m = "Da苉 ist eine Testnachricht";
        let block_size = 8;
        let _basis_length = 55296 as u32;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, true, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, true, 55296),
            55296,
        );
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 6;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, true, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, true, 55296),
            55296,
        );
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 47;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, true, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, true, 55296),
            55296,
        );
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 3;
        let encoded = create_string_from_blocks_encrypt(
            encode_string_to_blocks(m, block_size, true, 55296),
            block_size + 1,
            55296,
        );
        let decoded = create_string_from_blocks_decrypt(
            encode_string_to_blocks(&encoded, block_size + 1, true, 55296),
            55296,
        );
        assert_eq!(decoded.trim(), m);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Da苉 ist eine Testnachricht";
        let block_size = 7;
        let _basis_length = 55296 as u32;
        let result = encode_string_to_blocks(message, block_size, true, 55296);
        let expected_result = vec![
            big_i!(1943938337267550087026074257524),
            big_i!(914822981356602019800946507860),
            big_i!(2887304683313907978613082523752),
            big_i!(3258925137110102081877384560672),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decrypt_chiffre() {
        let sums = vec![
            big_i!(1943938337267550087026074257524),
            big_i!(914822981356602019800946507860),
            big_i!(2887304683313907978613082523752),
            big_i!(3258925137110102081877384560672),
        ];
        let result = create_string_from_blocks_decrypt(sums, 55296);
        let expected_result = "Da苉 ist eine Testnachricht".to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 4 aufgeteilt.
        let message = String::from("Da苉 ist eine Testnachricht");
        let block_size = 4;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(
            result,
            vec!["Da苉 ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
        );

        // Testfall 2: Ein String, der bereits eine Blockgröße hat, wird nicht verändert,
        // es kommt kein neuer leerer Block dazu.
        let message = String::from("123AB");
        let block_size = 5;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["123AB"]);

        // Testfall 3: Ein leerer String wird in Blöcke der Größe 3 aufgeteilt.
        let message = String::from("   ");
        let block_size = 3;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["   "]);

        // Testfall 4: Ein String wird in Blöcke der Größe 1 aufgeteilt.
        let message = String::from("abcdef");
        let block_size = 1;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_string_to_int_vec() {
        let message = "Da苉 ist eine Testnachricht ";
        let blocks = split_into_blocks(&message, 4, true);
        let expected = vec![
            vec!['D' as u32, 'a' as u32, '苉' as u32, ' ' as u32],
            vec!['i' as u32, 's' as u32, 't' as u32, ' ' as u32],
            vec!['e' as u32, 'i' as u32, 'n' as u32, 'e' as u32],
            vec![' ' as u32, 'T' as u32, 'e' as u32, 's' as u32],
            vec!['t' as u32, 'n' as u32, 'a' as u32, 'c' as u32],
            vec!['h' as u32, 'r' as u32, 'i' as u32, 'c' as u32],
            vec!['h' as u32, 't' as u32, ' ' as u32, ' ' as u32],
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

        let base = big_i!(55296);
        let result = to_sum_vec(digit_vectors, &base);

        let expected_result = vec![
            big_i!(11497444858239008),
            big_i!(17753298306195488),
            big_i!(17076964999090277),
            big_i!(5410678690363507),
            big_i!(19613115525224547),
            big_i!(17584219565365347),
            big_i!(17584225676623904),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_to_strings() {
        let sums = vec![
            big_i!(11497444858239008),
            big_i!(17753298306195488),
            big_i!(17076964999090277),
            big_i!(5410678690363507),
            big_i!(19613115525224547),
            big_i!(17584219565365347),
            big_i!(17584225676623904),
        ];

        let base = big_i!(55296);
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
    }

    #[test]
    fn test_ubig_to_u32() {
        let value = big_i!(12345);
        let result = big_int_to_u32(&value);
        assert_eq!(result, 12345);
    }
}
