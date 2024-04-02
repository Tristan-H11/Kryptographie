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
use crate::encryption::symmetric_encryption_types::SymmetricEncryptor;
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;
use crate::math_core::traits::logarithm::Logarithm;

pub struct MenezesVanstoneStringScheme {}

impl EncryptionScheme for MenezesVanstoneStringScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneStringScheme {}

#[derive(Clone, Debug)]
pub struct MvStringCiphertext {
    pub ciphertext: String,
    pub points: Vec<FiniteFieldEllipticCurvePoint>
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
        if (block_size < 2) { // TODO Korrigieren
            block_size = 2;
        }
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };

        // Den Plaintext auffüllen, bis er eine gerade Anzahl von Blöcken erzeugen wird
        let diff = block_size * 2 - (plaintext.len() % (block_size * 2));
        let supplement = "\u{0000}".repeat(diff);
        let mut padded_plaintext = String::from(plaintext);
        padded_plaintext.push_str(&supplement);

        // Blockchiffre anwenden
        let message = ToDecimalBlockScheme::encrypt(&padded_plaintext, &decimal_unicode_key);

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

        let mut big_int_vec: Vec<BigInt> = Vec::new();
        for ciphertext in &ciphertext_list {
            big_int_vec.push(ciphertext.first.clone());
            big_int_vec.push(ciphertext.second.clone());
        }

        let mut ciphertext_string = FromDecimalBlockScheme::encrypt(&big_int_vec, &decimal_unicode_key);

        let points = ciphertext_list.iter().flat_map(|c| vec![c.point.clone()]).collect();

        MvStringCiphertext {
            ciphertext: ciphertext_string,
            points,
        }
        // Den Ciphertext wieder in einen String mappen
    }
}

impl<'a> Decryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = str;
    type Output = String;
    type Key = MenezesVanstoneStringPrivateKey;
}

impl AsymmetricDecryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        unimplemented!();

        // String in einen MenezesVanstoneCiphertext mappen

        let message = MenezesVanstoneCiphertext {
            point: FiniteFieldEllipticCurvePoint {
                x: Default::default(),
                y: Default::default(),
                is_infinite: false,
            },
            first: Default::default(),
            second: Default::default(),
        };

        let plaintext = MenezesVanstoneScheme::decrypt(&key.mv_key, &message, service);

        // MenezesVanstonePlaintext in einen String mappen
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::core::menezes_vanstone::keys::{MenezesVanstonePrivateKey, MenezesVanstonePublicKey};
    use crate::math_core::ecc::finite_field_elliptic_curve::FiniteFieldEllipticCurve;
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
    use super::*;

    #[test]
    fn testtest() {
            let curve = FiniteFieldEllipticCurve::new(2.into(), 738083.into());
            let generator = FiniteFieldEllipticCurvePoint::new(2.into(), 1.into());
            let y = FiniteFieldEllipticCurvePoint::new(3.into(), 10.into());
            let x = 7.into();

            let public_key = MenezesVanstonePublicKey {
                curve: curve.clone(),
                generator,
                y,
            };
            let public_key = MenezesVanstoneStringPublicKey {
                mv_key: public_key,
                radix: 140,
            };
            let private_key = MenezesVanstonePrivateKey { curve, x };

            let plaintext = "Das ist ein Test  ";
            let service = NumberTheoryService::new(Fast);
            let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &plaintext, service);
            println!("{:?}", ciphertext);
    }

    // #[test]
    // fn test_menezes_vanstone_encryption_decryption() {
    //     let curve = FiniteFieldEllipticCurve::new(3.into(), 9.into(), 11.into());
    //     let generator = FiniteFieldEllipticCurvePoint::new(2.into(), 1.into());
    //     let y = FiniteFieldEllipticCurvePoint::new(3.into(), 10.into());
    //     let x = 7.into();
    //
    //     let public_key = MenezesVanstonePublicKey {
    //         curve: curve.clone(),
    //         generator,
    //         y,
    //     };
    //     let private_key = MenezesVanstonePrivateKey { curve, x };
    //
    //     let plaintext = "Das ist ein Test  ";
    //
    //     let service = NumberTheoryService::new(Fast);
    //     let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &plaintext, service);
    //     let decrypted_plaintext =
    //         MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, service);
    //
    //     assert_eq!(plaintext, decrypted_plaintext);
    // }
}
