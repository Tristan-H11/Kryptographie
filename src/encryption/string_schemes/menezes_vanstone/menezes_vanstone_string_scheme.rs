use bigdecimal::num_bigint::BigInt;
use bigdecimal::Zero;

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
    MenezesVanstoneStringKeyPair, MenezesVanstoneStringPrivateKey, MenezesVanstoneStringPublicKey,
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

impl MenezesVanstoneStringScheme {
    pub fn generate_keypair(
        n: i32,
        modul_width: u32,
        miller_rabin_iterations: u32,
        random_seed: u32,
        radix: u32,
    ) -> MenezesVanstoneStringKeyPair {
        assert_ne!(n, 0, "n darf nicht 0 sein, ist aber {}.", n);
        assert!(
            modul_width > 3,
            "Die Modulbreite muss mindestens 4 Bit betragen, ist aber {}.",
            modul_width
        );
        assert_ne!(
            radix, 0,
            "Die Basis des Zeichensatzes muss größer als 0 sein, ist aber {}.",
            radix
        );

        let key_pair = MenezesVanstoneScheme::generate_keypair(
            n,
            modul_width,
            miller_rabin_iterations,
            random_seed,
        );

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let public_key = MenezesVanstoneStringPublicKey {
            mv_key: public_key,
            radix,
        };

        let private_key = MenezesVanstoneStringPrivateKey {
            mv_key: private_key,
            radix,
        };

        MenezesVanstoneStringKeyPair {
            public_key,
            private_key,
        }
    }
}

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
        let block_size = key.mv_key.curve.prime.log(&radix.into());
        if block_size < 1 {
            panic!("Verhältnis von Basis und Modul ist zu klein.")
        }
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };

        // TODO Löschen? Erstmal abhängen lassen. Deadline: 20.04.2024
        // TODO Hier ist das Padding. Das muss aber noch irgendwie wieder rausgerechnet werden.
        // TODO Funktioniert aber auch THEORETISCH(!) einwandfrei ohne. Überwiegend ungetestet!
        // TODO Wenn die Nachricht genau uneven-blocks lang ist, wird ein letzter Block
        // TODO mit Wert 0 eingefügt.
        // Den Plaintext auffüllen, bis er eine gerade Anzahl von Blöcken erzeugen wird
        // let diff = block_size * 2 - (plaintext.len() % (block_size * 2));
        // let supplement = "\u{0000}".repeat(diff);
        // let mut padded_plaintext = String::from(plaintext);
        // if (plaintext.len() / block_size * 2) == 0 {
        //     padded_plaintext.push_str(&supplement);
        // }

        // Blockchiffre anwenden
        let message = ToDecimalBlockScheme::encrypt(&plaintext, &decimal_unicode_key);

        // Die Zahlen in eine Liste von MenezesVanstonePlaintext mappen
        let mut plaintext_list: Vec<MenezesVanstonePlaintext> = Vec::new();
        for chunk in message.chunks(2) {
            // Falls es den zweiten Block nicht gibt, soll eine 0 eingefügt werden.
            if chunk.len() < 2 {
                let plaintext_chunk = MenezesVanstonePlaintext {
                    first: chunk[0].clone(),
                    second: BigInt::zero(),
                };
                plaintext_list.push(plaintext_chunk);
            } else {
                let plaintext_chunk = MenezesVanstonePlaintext {
                    first: chunk[0].clone(),
                    second: chunk[1].clone(),
                };
                plaintext_list.push(plaintext_chunk);
            }
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

        let conversion_post_key = DecimalUnicodeConversionSchemeKey {
            radix,
            block_size: block_size + 1,
        };
        let ciphertext_string = FromDecimalBlockScheme::encrypt(&big_int_vec, &conversion_post_key);

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
        for i in (0..big_int_vec.len()).step_by(2) {
            // TODO Aufhübschen
            let first = big_int_vec[i].clone();
            let second = if i + 1 < big_int_vec.len() {
                big_int_vec[i + 1].clone()
            } else {
                BigInt::zero()
            };

            let ciphertext = MenezesVanstoneCiphertext {
                point: points[i / 2].clone(),
                first,
                second,
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
    use rand::distributions::{Uniform};
    use rand::Rng;

    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;

    use super::*;

    #[test]
    fn test_menezes_vanstone_encryption_decryption() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let radix = rand::thread_rng().gen_range(240..55296);
        let n = rand::thread_rng().gen_range(1..30);
        let modul_width = rand::thread_rng().gen_range(4..256);
        let random_seed = rand::thread_rng().gen_range(1..1000);
        let key_pair =
            MenezesVanstoneStringScheme::generate_keypair(n, modul_width, 40, random_seed, radix);

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // Es soll ein zufälliger String erzeugt werden, der zwischen 0 und 400 Zeichen lang ist.
        let random_string_length = rand::thread_rng().gen_range(0..400);
        // siehe https://stackoverflow.com/a/54277357
        let plaintext: String = rand::thread_rng()
            .sample_iter(Uniform::new(char::from(0), char::from_u32(radix).unwrap())) // Sollte nicht panicen, weil radix immer innerhalb der Unicode-Zeichen liegt
            .take(random_string_length)
            .collect();

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneStringScheme::encrypt(&public_key, &plaintext, service);
        let decrypted_plaintext =
            MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, service);
        assert_eq!(plaintext, decrypted_plaintext);
    }
}
