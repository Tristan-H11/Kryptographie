use bigdecimal::num_bigint::BigInt;
use bigdecimal::{One, Zero};

use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::encryption::string_schemes::decimal_unicode_schemes::decimal_unicode_conversion_core::ToRadixString;
use crate::encryption::string_schemes::decimal_unicode_schemes::keys::DecimalUnicodeConversionSchemeKey;
use crate::encryption::symmetric_encryption_types::{
    SymmetricDecryptor, SymmetricEncryptionScheme, SymmetricEncryptor,
};

/// Ein Verschlüsselungsschema, das einen Klartext in Blöcke umwandelt und diese Blöcke in Dezimalzahlen umwandelt.
/// Dafür wird der übergebene Text als Unicode-Zeichenfolge bis zu einem gegebenen Radxi interpretiert, in Blöcke
/// der gegebenen Größe aufgeteilt und diese Blöcke durch g-adische Entwicklung in Dezimalzahlen umgewandelt.
pub struct ToDecimalBlockScheme {}

impl EncryptionScheme for ToDecimalBlockScheme {}

impl SymmetricEncryptionScheme for ToDecimalBlockScheme {}

impl<'a> Encryptor<ToDecimalBlockScheme> for ToDecimalBlockScheme {
    type Input = str;
    type Output = Vec<BigInt>;
    type Key = DecimalUnicodeConversionSchemeKey;
}

impl<'a> SymmetricEncryptor<ToDecimalBlockScheme> for ToDecimalBlockScheme {
    /// Interpretiert den String als eine Zeichenkette von Unicode-Zeichen bis zu einem gegebenen Radix, teilt diese
    /// Zeichenkette in Blöcke der gegebenen Größe auf und wandelt diese Blöcke in Dezimalzahlen um.
    ///
    /// # Arguments
    /// * `plaintext` - Der Klartext, der verschlüsselt werden soll.
    /// * `key` - Der Schlüssel, der die Basis und die Blockgröße enthält.
    ///
    /// # Returns
    /// Ein Vektor von Dezimalzahlen, die die verschlüsselten Blöcke repräsentieren.
    fn encrypt(plaintext: &Self::Input, key: &Self::Key) -> Self::Output {
        assert!(key.radix > 0, "Die Basis muss größer als 0 sein.");
        assert!(key.block_size > 0, "Die Blockgröße muss größer als 0 sein.");

        // Plaintext in Blöcke aufteilen
        let string_chunks = plaintext
            .chars()
            .collect::<Vec<char>>()
            .chunks(key.block_size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();

        // Einzelne Buchstaben in den Blöcken in u32 Dezimalzahlen umwandeln
        let encoded_chars_chunks = string_chunks
            .iter()
            .map(|chunk| chunk.chars().map(|c| c as u32).collect())
            .collect::<Vec<Vec<u32>>>();

        // G-Adische Summe über den einzelnen Blöcken bilden. Die Basis ist dabei der Radix aus dem Schlüssel.
        encoded_chars_chunks
            .iter()
            .map(|chunk| {
                // Hier wird reversed, weil die g-adische Entwicklung von rechts nach links gebildet wird.
                chunk
                    .iter()
                    .rev()
                    .fold((BigInt::zero(), BigInt::one()), |(acc, acc_base), &c| {
                        let product: BigInt = &acc_base * c;
                        (acc + product, acc_base * key.radix)
                    })
                    .0 // Nur das Ergebnis wird benötigt. Die entwickelte Basis kann verworfen werden.
            })
            .collect()
    }
}

impl Decryptor<ToDecimalBlockScheme> for ToDecimalBlockScheme {
    type Input = Vec<BigInt>;
    type Output = String;
    type Key = DecimalUnicodeConversionSchemeKey;
}

impl SymmetricDecryptor<ToDecimalBlockScheme> for ToDecimalBlockScheme {
    /// Wandelt die Dezimalzahlen in Blöcke von Unicode-Zeichen um und fügt diese Blöcke zu einer Zeichenfolge zusammen.
    ///
    /// # Arguments
    /// * `ciphertext` - Die Dezimalzahlen, die die verschlüsselten Blöcke repräsentieren.
    /// * `key` - Der Schlüssel, der die Basis und die Blockgröße enthält.
    ///
    /// # Returns
    /// Die entschlüsselte Zeichenfolge.
    fn decrypt(ciphertext: &Self::Input, key: &Self::Key) -> Self::Output {
        assert!(key.radix > 0, "Die Basis muss größer als 0 sein.");
        assert!(key.block_size > 0, "Die Blockgröße muss größer als 0 sein.");

        // Die Blöcke des ciphertextes als Dezimalzahl interpretieren und in g-adische Entwicklung umwandeln;
        // dabei ist key.radix die Basis der g-adischen Entwicklung.
        let decoded_chars_chunks = ciphertext
            .iter()
            .map(|decimal| decimal.to_radix_string(&key.radix).unwrap()) // TODO Fehlerbehandlung
            .collect::<Vec<String>>();

        // Die einzelnen Blöcke zu einer Zeichenfolge zusammenfügen
        decoded_chars_chunks.join("")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_happy_flow() {
        let m = "Da苉 ist eine Testnachricht";
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 55296,
            block_size: 8,
        };

        let ciphertext = ToDecimalBlockScheme::encrypt(m, &key);

        assert_eq!(
            ciphertext,
            vec![
                BigInt::from_str("107492014297546449612193802144047136").unwrap(),
                BigInt::from_str("159656113899559548508775364389320819").unwrap(),
                BigInt::from_str("183367115080887221772378868133959779").unwrap(),
                5750900.into()
            ]
        );

        let plaintext = ToDecimalBlockScheme::decrypt(&ciphertext, &key);
        assert_eq!(m, plaintext);
    }

    #[test]
    fn test_empty_string() {
        let m = "";
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 55296,
            block_size: 8,
        };

        let ciphertext = ToDecimalBlockScheme::encrypt(m, &key);

        assert_eq!(ciphertext, vec![]);

        let plaintext = ToDecimalBlockScheme::decrypt(&ciphertext, &key);
        assert_eq!(m, plaintext);
    }

    #[test]
    #[should_panic]
    fn test_invalid_radix_encrypt() {
        let m = "Da苉 ist eine Testnachricht";
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 0,
            block_size: 8,
        };

        ToDecimalBlockScheme::encrypt(m, &key);
    }

    #[test]
    #[should_panic]
    fn test_invalid_block_size_encrypt() {
        let m = "Da苉 ist eine Testnachricht";
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 55296,
            block_size: 0,
        };

        ToDecimalBlockScheme::encrypt(m, &key);
    }

    #[test]
    #[should_panic]
    fn test_invalid_radix_decrypt() {
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 0,
            block_size: 8,
        };

        ToDecimalBlockScheme::decrypt(&vec![], &key);
    }

    #[test]
    #[should_panic]
    fn test_invalid_block_size_decrypt() {
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 55296,
            block_size: 0,
        };

        ToDecimalBlockScheme::decrypt(&vec![], &key);
    }

    #[test]
    fn test_manipulated_block_remains_other_blocks_valid() {
        let m = "Da苉 ist eine Testnachricht";
        let key = DecimalUnicodeConversionSchemeKey {
            radix: 55296,
            block_size: 8,
        };

        let ciphertext = ToDecimalBlockScheme::encrypt(m, &key);

        let mut manipulated_ciphertext = ciphertext.clone();
        manipulated_ciphertext[0] = BigInt::from_str("123456789").unwrap();

        let plaintext = ToDecimalBlockScheme::decrypt(&manipulated_ciphertext, &key);
        assert_eq!(plaintext, "ࢸ贕eine Testnachricht");
        // Hier ist nur der erste Block manipuliert, die anderen Blöcke sind noch valide.
        // Das Zeichen ggf am Ende sieht nur aus, als wäre es dort falsch, es steht aber an der richtigen Stelle.
        // Das ist ein Zeichen, das von rechts nach links gelesen wird, also muss es rechts stehen, gehört aber zum ersten Block.
    }
}
