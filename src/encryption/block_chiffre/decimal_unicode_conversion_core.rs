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
    /// Eine Zeichenkette, die die g-adische Entwicklung der Dezimalzahl repräsentiert.
    fn to_radix_string(&self, radix: &u32) -> String;
}

impl ToRadixString for BigInt {
    fn to_radix_string(&self, radix: &u32) -> String {
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
