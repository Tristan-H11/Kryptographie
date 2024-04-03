use bigdecimal::num_bigint::BigInt;

use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor,
};
use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::{
    MenezesVanstoneCiphertext, MenezesVanstonePlaintext, MenezesVanstoneScheme,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::encryption::string_schemes::decimal_unicode_schemes::from_decimal_block_scheme::FromDecimalBlockScheme;
use crate::encryption::string_schemes::decimal_unicode_schemes::keys::DecimalUnicodeConversionSchemeKey;
use crate::encryption::string_schemes::decimal_unicode_schemes::to_decimal_block_scheme::ToDecimalBlockScheme;
use crate::encryption::string_schemes::menezes_vanstone::keys::{
    MenezesVanstoneStringPrivateKey, MenezesVanstoneStringPublicKey,
};
use crate::encryption::symmetric_encryption_types::{SymmetricDecryptor, SymmetricEncryptor};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::traits::logarithm::Logarithm;

pub struct MenezesVanstoneStringScheme {}

impl EncryptionScheme for MenezesVanstoneStringScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneStringScheme {}

#[derive(Clone, Debug)]
pub struct MvStringCiphertext {
    pub ciphertext: String,
    pub points: Vec<FiniteFieldEllipticCurvePoint>,
}

// TODO: KeyGen für MenezesVanstoneScheme implementieren

impl<'a> Encryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = str;
    type Output = MvStringCiphertext;
    type Key = MenezesVanstoneStringPublicKey;
}

impl AsymmetricEncryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        let radix = key.radix;
        let mut block_size = key.mv_key.curve.prime.log(&radix.into());
        if block_size < 2 {
            // TODO Korrigieren, BlockSize einheitlich irgendwo berechnen und ein Minimum festlegen
            block_size = 2;
        }
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };

        // TODO Hier ist das Padding. Das muss aber noch irgendwie wieder rausgerechnet werden.
        // TODO Funktioniert aber auch THEORETISCH(!) einwandfrei ohne. Überwiegend ungetestet!
        // TODO Damit es ohne Padding funktioniert, muss ggf ein letzter, nicht vorhandener Block
        // TODO mit \u0000 gefüllt werden.
        // Den Plaintext auffüllen, bis er eine gerade Anzahl von Blöcken erzeugen wird
        let diff = block_size * 2 - (plaintext.len() % (block_size * 2));
        let supplement = "\u{0000}".repeat(diff);
        let mut padded_plaintext = String::from(plaintext);
        padded_plaintext.push_str(&supplement);

        // Blockchiffre anwenden
        let message = ToDecimalBlockScheme::encrypt(&plaintext, &decimal_unicode_key);

        // Die Zahlen in eine Liste von MenezesVanstonePlaintext mappen
        let mut plaintext_list: Vec<MenezesVanstonePlaintext> = Vec::new();
        for chunk in message.chunks_exact(2) {
            let plaintext_chunk = MenezesVanstonePlaintext {
                first: chunk[0].clone(),
                second: chunk[1].clone(),
            };
            plaintext_list.push(plaintext_chunk);
        }

        // Jeden einzelnen Plaintext für sich verschlüsseln
        let mut ciphertext_list: Vec<MenezesVanstoneCiphertext> = Vec::new();
        for plaintext in plaintext_list {
            let ciphertext = MenezesVanstoneScheme::encrypt(&key.mv_key, &plaintext, service);
            ciphertext_list.push(ciphertext);
        }

        // Die Zahlen wieder in Strings konvertieren
        let mut big_int_vec: Vec<BigInt> = Vec::new();
        for ciphertext in &ciphertext_list {
            big_int_vec.push(ciphertext.first.clone());
            big_int_vec.push(ciphertext.second.clone());
        }
        let ciphertext_string = FromDecimalBlockScheme::encrypt(&big_int_vec, &decimal_unicode_key);

        // Die genutzten Punkte akkumulieren
        let points = ciphertext_list
            .iter()
            .flat_map(|c| vec![c.point.clone()])
            .collect();

        MvStringCiphertext {
            ciphertext: ciphertext_string,
            points,
        }
    }
}

impl<'a> Decryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = MvStringCiphertext;
    type Output = String;
    type Key = MenezesVanstoneStringPrivateKey;
}

impl AsymmetricDecryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        let ciphertext_string = &ciphertext.ciphertext;
        let points = &ciphertext.points;
        let radix = key.radix;
        let block_size = key.mv_key.curve.prime.log(&radix.into()) + 1;

        // Blockchiffre anwenden
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };
        let big_int_vec = FromDecimalBlockScheme::decrypt(&ciphertext_string, &decimal_unicode_key);

        // Wenn wir hier keine zusammenpassende Anzahl von Punkten und Tupeln haben,
        // dann ist die Nachricht nicht korrekt verschlüsselt worden.
        // Durch '*2' wird ebenfalls sichergestellt, dass es eine gerade Anzahl von Tupeln gibt.
        assert_eq!(points.len() * 2, big_int_vec.len(), "Ungültiger Ciphertext");

        // Die Zahlen in eine Liste von MenezesVanstoneCiphertext mappen
        let mut ciphertext_list: Vec<MenezesVanstoneCiphertext> = Vec::new();
        for i in 0..big_int_vec.len() / 2 {
            let ciphertext = MenezesVanstoneCiphertext {
                point: points[i].clone(),
                first: big_int_vec[i * 2].clone(),
                second: big_int_vec[i * 2 + 1].clone(),
            };
            ciphertext_list.push(ciphertext);
        }

        // Jeden einzelnen Ciphertext für sich entschlüsseln
        let mut plaintext_list: Vec<MenezesVanstonePlaintext> = Vec::new();
        for ciphertext in ciphertext_list {
            let plaintext = MenezesVanstoneScheme::decrypt(&key.mv_key, &ciphertext, service);
            plaintext_list.push(plaintext);
        }

        // Die Zahlen in einer flachen Liste sammeln und in Strings konvertieren
        let mut big_int_vec: Vec<BigInt> = Vec::new();
        for plaintext in &plaintext_list {
            big_int_vec.push(plaintext.first.clone());
            big_int_vec.push(plaintext.second.clone());
        }
        ToDecimalBlockScheme::decrypt(&big_int_vec, &decimal_unicode_key)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::str::FromStr;

    use crate::encryption::core::menezes_vanstone::keys::{
        MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
    };
    use crate::math_core::ecc::finite_field_elliptic_curve::FiniteFieldEllipticCurve;
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;

    use super::*;

    #[test]
    fn test_menezes_vanstone_encryption_decryption() {
        let curve = FiniteFieldEllipticCurve::new(
            5.into(),
            BigInt::from_str("259421157018863391010844302469063884861").unwrap(),
        );
        let generator = FiniteFieldEllipticCurvePoint::new(
            BigInt::from_str("152198469913648308717544634828661961231").unwrap(),
            BigInt::from_str("50296851635441247077790719368115682846").unwrap(),
        );
        let y = FiniteFieldEllipticCurvePoint::new(
            BigInt::from_str("26370934085012164485153092381593646122").unwrap(),
            BigInt::from_str("126290671313284822425335475919650022666").unwrap(),
        );
        let x = BigInt::from_str("12401522966815986254216934185370504355").unwrap();

        let public_key = MenezesVanstonePublicKey {
            curve: curve.clone(),
            generator,
            y,
        };

        // Der Radix soll hier für jeden Testlauf zufällig gewählt werden, damit die Tests
        // mehr abfangen können.
        let radix = rand::thread_rng().gen_range(240..55296);
        println!("Radix: {}", radix);
        let public_key = MenezesVanstoneStringPublicKey {
            mv_key: public_key,
            radix,
        };
        let private_key = MenezesVanstonePrivateKey { curve, x };
        let private_key = MenezesVanstoneStringPrivateKey {
            mv_key: private_key,
            radix,
        };

        let plaintext = "Das ist ein Test1234567890";

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &plaintext, service);
        println!("{:?}", ciphertext);
        let decrypted_plaintext =
            MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, service);
        println!("{:?}", decrypted_plaintext);
        assert_eq!(plaintext, decrypted_plaintext);
    }
}
