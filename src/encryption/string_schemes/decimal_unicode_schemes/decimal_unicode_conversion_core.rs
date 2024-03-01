use std::char::from_u32;

use bigdecimal::{Signed, ToPrimitive};
use num::BigInt;

pub trait ToRadixString {
    /// Wandelt die gegebene Dezimalzahl in eine Zeichenkette um, indem die g-adische Entwicklung der Summe gebildet wird.
    ///
    /// # Arguments
    /// * `decimal` - Die Dezimalzahl, die umgewandelt werden soll.
    /// * `radix` - Die Basis, in die die Dezimalzahl umgewandelt werden soll.
    ///
    /// # Returns
    /// Eine Zeichenkette, die die g-adische Entwicklung der Dezimalzahl in Unicode-Darstellung repräsentiert.
    fn to_radix_string(&self, radix: &u32) -> String;
}

impl ToRadixString for BigInt {
    fn to_radix_string(&self, radix: &u32) -> String {
        assert!(radix > &0, "Die Basis muss größer als 0 sein.");

        let mut decimal = self.clone();
        let mut result = String::new();

        while decimal.is_positive() {
            // Hier werden die u32-Operationen statt .div_rem(&BigInt) genutzt, weil diese schneller sind.
            let remainder = decimal.clone() % radix;
            decimal = decimal / radix;
            let char = from_u32(
                remainder
                    .to_u32()
                    .expect("Umwandlung in u32 fehlgeschlagen"),
            )
            .expect("Umwandlung in char fehlgeschlagen"); // TODO Fehlerbehandlung ggf später einbauen
            result.push(char);
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_to_radix_string() {
        let decimal = BigInt::from(123456789);
        let radix = 16;
        let expected = "\u{7}\u{5}\u{b}\u{c}\r\u{1}\u{5}";

        let result = decimal.to_radix_string(&radix);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_radix_string_zero() {
        let decimal = BigInt::from(0);
        let radix = 16;
        let expected = "";

        let result = decimal.to_radix_string(&radix);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_radix_string_decimal_is_radix() {
        let decimal = BigInt::from(16);
        let radix = 16;
        let expected = "\u{1}\u{0}";

        let result = decimal.to_radix_string(&radix);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_radix_big_numbers() {
        let decimal = BigInt::from_str("12345678987654321").unwrap();
        let radix = 55296;
        let expected = "IЇ秜咱";

        let result = decimal.to_radix_string(&radix);

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_to_radix_string_invalid_radix() {
        let decimal = BigInt::from(123456789);
        let radix = 0;

        decimal.to_radix_string(&radix);
    }
}
