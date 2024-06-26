use crate::api::endpoints::mv::MvCipherTextBean;
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
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;
use crate::math_core::traits::logarithm::Logarithm;
use crate::shared::errors::MenezesVanstoneError;
use anyhow::{ensure, Context, Result};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::Zero;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct MenezesVanstoneStringScheme {}

impl EncryptionScheme for MenezesVanstoneStringScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneStringScheme {}

#[derive(Clone, Debug)]
pub struct MvStringCiphertext {
    pub ciphertext: String,
    pub points: Vec<FiniteFieldEllipticCurvePoint>,
}

impl From<MvCipherTextBean> for MvStringCiphertext {
    fn from(ciphertext: MvCipherTextBean) -> Self {
        MvStringCiphertext {
            ciphertext: ciphertext.encrypted_message,
            points: ciphertext.points.into_iter().map(Into::into).collect(),
        }
    }
}

impl MenezesVanstoneStringScheme {
    pub fn generate_keypair(
        n: i32,
        modul_width: u32,
        miller_rabin_iterations: u32,
        service_wrapper: &NumberTheoryWithPrngService,
        radix: u32,
    ) -> Result<MenezesVanstoneStringKeyPair> {
        ensure!(n != 0, MenezesVanstoneError::InvalidNValueError(n));
        ensure!(
            modul_width > 3,
            MenezesVanstoneError::InvalidModulusWidthError(modul_width)
        );
        ensure!(
            radix != 0,
            MenezesVanstoneError::InvalidNumberSystemBaseError(radix)
        );

        let key_pair = MenezesVanstoneScheme::generate_keypair(
            n,
            modul_width,
            miller_rabin_iterations,
            service_wrapper,
        )
        .context("Error while creating keypair for MenezesVanstone-Core. Error: {:#?}")?;

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

        Ok(MenezesVanstoneStringKeyPair {
            public_key,
            private_key,
        })
    }
}

impl<'a> Encryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = str;
    type Output = Result<MvStringCiphertext>;
    type Key = MenezesVanstoneStringPublicKey;
}

impl AsymmetricEncryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let radix = key.radix;
        let block_size = key.mv_key.curve.prime.log(&radix.into());

        ensure!(
            block_size > 0,
            "Die Blockgröße muss mindestens 1 sein, ist aber {}.",
            block_size
        );
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };

        // Blockchiffre anwenden
        let message = ToDecimalBlockScheme::encrypt(&plaintext, &decimal_unicode_key);

        // Die Zahlen in eine Liste von MenezesVanstonePlaintext mappen
        let plaintext_list: Vec<MenezesVanstonePlaintext> = message
            .chunks(2)
            .map(|chunk| MenezesVanstonePlaintext::from_chunk(chunk))
            .collect();

        // Jeden einzelnen Plaintext für sich verschlüsseln
        let ciphertext_list: Vec<MenezesVanstoneCiphertext> = plaintext_list
            .par_iter()
            .map(|plaintext| {
                MenezesVanstoneScheme::encrypt(&key.mv_key, plaintext, service).context(
                    "Verschlüsselung im MenezesVanstone-Kern fehlgeschlagen. Fehler: {:#?}",
                )
            })
            .collect::<Result<Vec<MenezesVanstoneCiphertext>>>()?;

        // Die Zahlen wieder in Strings konvertieren
        let big_int_vec: Vec<BigInt> = ciphertext_list
            .iter()
            .flat_map(|ciphertext| vec![ciphertext.first.clone(), ciphertext.second.clone()])
            .filter_map(|x| x)
            .collect();

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

        Ok(MvStringCiphertext {
            ciphertext: ciphertext_string,
            points,
        })
    }
}

impl<'a> Decryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = MvStringCiphertext;
    type Output = Result<String>;
    type Key = MenezesVanstoneStringPrivateKey;
}

impl AsymmetricDecryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let ciphertext_string = &ciphertext.ciphertext;
        let points = &ciphertext.points;
        let radix = key.radix;
        let block_size = key.mv_key.curve.prime.log(&radix.into()) + 1;

        // Blockchiffre anwenden
        let decimal_unicode_key = DecimalUnicodeConversionSchemeKey { radix, block_size };
        let big_int_vec = FromDecimalBlockScheme::decrypt(&ciphertext_string, &decimal_unicode_key);

        // Die Zahlen in eine Liste von MenezesVanstoneCiphertext mappen
        let ciphertext_list: Vec<MenezesVanstoneCiphertext> = big_int_vec
            .chunks(2)
            .zip(points.iter().cloned())
            .map(|(chunk, point)| MenezesVanstoneCiphertext {
                point,
                first: chunk.get(0).cloned(),
                second: chunk.get(1).cloned(),
            })
            .collect();

        // Jeden einzelnen Ciphertext für sich entschlüsseln
        let plaintext_list: Vec<MenezesVanstonePlaintext> = ciphertext_list
            .par_iter()
            .map(|ciphertext| {
                MenezesVanstoneScheme::decrypt(&key.mv_key, ciphertext, service)
                    .context("Entschlüsselung im MenezesVanstone-Kern fehlgeschlagen.")
            })
            .collect::<Result<Vec<MenezesVanstonePlaintext>>>()?;

        let big_int_vec: Vec<BigInt> = plaintext_list
            .iter()
            .flat_map(|plaintext| vec![plaintext.first.clone(), plaintext.second.clone()])
            .filter_map(|x| x)
            .collect();

        Ok(ToDecimalBlockScheme::decrypt(
            &big_int_vec,
            &decimal_unicode_key,
        ))
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::Rng;

    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;

    use super::*;

    #[test]
    #[ignore] // TODO Fix me: Dieser Test rennt in manchen Fällen in eine Endlosschleife.
    fn test_menezes_vanstone_encryption_decryption() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let radix = rand::thread_rng().gen_range(240..55296);
        let n = rand::thread_rng().gen_range(1..30);
        let modul_width = rand::thread_rng().gen_range(4..100);
        let random_seed = rand::thread_rng().gen_range(1..1000);
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneStringScheme::generate_keypair(n, modul_width, 40, &service, radix)
                .unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // Es soll ein zufälliger String erzeugt werden, der zwischen 0 und 400 Zeichen lang ist.
        let random_string_length = rand::thread_rng().gen_range(0..400);
        // siehe https://stackoverflow.com/a/54277357
        let plaintext: String = rand::thread_rng()
            .sample_iter(Uniform::new(char::from(0), char::from_u32(radix).unwrap())) // Sollte nicht panicen, weil radix immer innerhalb der Unicode-Zeichen liegt
            .take(random_string_length)
            .collect();

        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let ciphertext =
            MenezesVanstoneStringScheme::encrypt(&public_key, &plaintext, &service).unwrap();
        let decrypted_plaintext =
            MenezesVanstoneStringScheme::decrypt(&private_key, &ciphertext, &service).unwrap();
        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_encryption_decryption_as_integrated_test() {
        // Alice's Schlüsselpaar generieren
        let alice_n = 3;
        let alice_modul_width = 512;
        let alice_seed = 11;
        let miller_rabin_iterations = 40;
        let radix = 55296;
        let service = NumberTheoryWithPrngService::new(Fast, alice_seed);
        let alice_keypair = MenezesVanstoneStringScheme::generate_keypair(
            alice_n,
            alice_modul_width,
            miller_rabin_iterations,
            &service,
            radix,
        )
        .unwrap();

        let bob_n = 3;
        let bob_modul_width = 512;
        let bob_seed = 7;
        let service = NumberTheoryWithPrngService::new(Fast, bob_seed);
        let bob_keypair = MenezesVanstoneStringScheme::generate_keypair(
            bob_n,
            bob_modul_width,
            miller_rabin_iterations,
            &service,
            radix,
        )
        .unwrap();
        let bob_public_key = bob_keypair.public_key.clone();
        let bob_private_key = bob_keypair.private_key.clone();
        let plaintext_string = "Hallo mein Homieeeeeeeeeeeeeeeeee was geht ab ??? 3232 !\"!\"!\"!";
        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let ciphertext =
            MenezesVanstoneStringScheme::encrypt(&bob_public_key, &plaintext_string, &service)
                .unwrap();

        let decrypted_plaintext =
            MenezesVanstoneStringScheme::decrypt(&bob_private_key, &ciphertext, &service).unwrap();
        assert_eq!(plaintext_string, decrypted_plaintext);
    }
}
