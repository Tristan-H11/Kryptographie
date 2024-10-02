use anyhow::Context;
use anyhow::{ensure, Result};
use std::cmp::max;

use crate::api::endpoints::mv::MvSignatureBean;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;
use log::debug;

use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor, Signer, Verifier,
};
use crate::encryption::core::menezes_vanstone::keys::{
    MenezesVanstoneKeyPair, MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::encryption::string_schemes::decimal_unicode_schemes::from_decimal_block_scheme::FromDecimalBlockScheme;
use crate::encryption::string_schemes::decimal_unicode_schemes::keys::DecimalUnicodeConversionSchemeKey;
use crate::encryption::symmetric_encryption_types::SymmetricDecryptor;
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceTrait;
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;
use crate::math_core::traits::increment::Increment;
use crate::math_core::traits::logarithm::Logarithm;
use crate::shared::errors::MenezesVanstoneError;
use crate::shared::hashing::sha256;

#[derive(Clone, Debug, PartialEq)]
pub struct MenezesVanstonePlaintext {
    pub first: Option<BigInt>,
    pub second: Option<BigInt>,
}

impl MenezesVanstonePlaintext {
    /// Erstellt eine Plaintext-Instanz aus einem BigInt-Wert
    pub fn single(value: BigInt) -> Self {
        MenezesVanstonePlaintext {
            first: Some(value),
            second: None,
        }
    }

    /// Erstellt eine Plaintext-Instanz aus zwei BigInt-Werten
    pub fn double(value1: BigInt, value2: BigInt) -> Self {
        MenezesVanstonePlaintext {
            first: Some(value1),
            second: Some(value2),
        }
    }

    /// Erstellt eine Plaintext-Instanz aus einem BigInt-Chunk
    /// Panics, wenn der Chunk nicht die Länge 1 oder 2 hat.
    pub fn from_chunk(chunk: &[BigInt]) -> Self {
        match chunk.len() {
            1 => MenezesVanstonePlaintext::single(chunk[0].clone()),
            2 => MenezesVanstonePlaintext::double(chunk[0].clone(), chunk[1].clone()),
            _ => panic!("Chunk size must be 1 or 2, but was {}", chunk.len()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenezesVanstoneCiphertext {
    pub point: FiniteFieldEllipticCurvePoint,
    pub first: Option<BigInt>,
    pub second: Option<BigInt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenezesVanstoneSignature {
    pub r: BigInt,
    pub s: BigInt,
}

impl From<MvSignatureBean> for MenezesVanstoneSignature {
    /// Mapped die Bean in das Domain-Modell
    fn from(signature: MvSignatureBean) -> Self {
        // TODO: Sauber ausarbeiten!
        let r: BigInt = signature.r.parse().unwrap();
        let s: BigInt = signature.s.parse().unwrap();

        // TODO Präsi: Einkommentieren, wenn der String manipuliert werden soll.
        // let radix = 55296;
        //
        // // Die größere der beiden Blockgrößen, damit sicher beide Werte enthalten sein werden.
        // let block_size = max(r.log(&radix.into()) + 1, s.log(&radix.into()) + 1);
        // let key = DecimalUnicodeConversionSchemeKey { block_size, radix };
        //
        // let blocks = FromDecimalBlockScheme::decrypt(&signature.string_representation, &key);
        //
        // assert!(blocks.len() > 1);
        //
        // let r = blocks[0].clone();
        // let s = blocks[1].clone();

        MenezesVanstoneSignature { r, s }
    }
}

pub struct MenezesVanstoneScheme {}

impl EncryptionScheme for MenezesVanstoneScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneScheme {}

impl MenezesVanstoneScheme {
    pub fn generate_keypair(
        n: i32,
        modul_width: u32,
        miller_rabin_iterations: u32,
        service_wrapper: &NumberTheoryWithPrngService,
    ) -> Result<MenezesVanstoneKeyPair> {
        ensure!(n != 0, MenezesVanstoneError::InvalidNValueError(n));
        ensure!(
            modul_width > 3,
            MenezesVanstoneError::InvalidModulusWidthError(modul_width)
        );

        let curve = SecureFiniteFieldEllipticCurve::new(
            n.into(),
            modul_width,
            miller_rabin_iterations,
            service_wrapper,
        )
        .context("Failed to create secure elliptic curve")?;

        let order_of_subgroup = &curve.order_of_subgroup;
        let (mut x, mut y);
        loop {
            x = service_wrapper
                .take_random_number_in_range(&1.into(), &order_of_subgroup.decrement());
            y = curve
                .generator
                .multiply(&x, &curve, &service_wrapper.number_theory_service)
                .context("Failed to calculate key-component y")?;
            if !y.x.is_zero() && !y.y.is_zero() {
                break;
            }
        }

        let public_key = MenezesVanstonePublicKey {
            curve: curve.clone(),
            y,
        };

        let private_key = MenezesVanstonePrivateKey { curve, x };

        Ok(MenezesVanstoneKeyPair {
            public_key,
            private_key,
        })
    }
}

impl Encryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstonePlaintext;
    type Output = Result<MenezesVanstoneCiphertext>;
    type Key = MenezesVanstonePublicKey;
}

impl AsymmetricEncryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let m1 = plaintext.first.clone();
        let m2 = plaintext.second.clone();
        let prime = &key.curve.prime;

        let curve = &key.curve;

        // Bestimmen von c1 und c2
        let (mut k, mut c1, mut c2);
        loop {
            // Dadurch, dass ein Wert < |H| gewählt wird, ist garantiert, dass der Punkt k*g niemals
            // im Unendlichen liegen wird.
            k = service
                .take_random_number_in_range(&1.into(), &curve.order_of_subgroup.decrement());
            let point = key
                .y
                .multiply(&k, curve, &service.number_theory_service)
                .context("Failed to calculate Point (c1, c2)")?;

            ensure!(
                !point.is_infinite,
                "Calculated point is infinite, but cannot be since k < |H|. With k = {}",
                k
            );

            (c1, c2) = (point.x, point.y);
            // Sind beide Werte ungleich 0, so ist das Paar (c1, c2) gültig
            if !c1.is_zero() && !c2.is_zero() {
                debug!(
                    "MV: Verschlüsselung mit k = {}, c1 = {} und c2 = {}",
                    k, c1, c2
                );
                break;
            }
        }
        let a = key
            .curve
            .generator
            .multiply(&k, curve, &service.number_theory_service)
            .context("Failed to calculate Point a")?;
        let b1 = m1.and_then(|m| Some((c1 * m) % prime));
        let b2 = m2.and_then(|m| Some((c2 * m) % prime));

        Ok(MenezesVanstoneCiphertext {
            point: a,
            first: b1,
            second: b2,
        })
    }
}

impl Decryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstoneCiphertext;
    type Output = Result<MenezesVanstonePlaintext>;
    type Key = MenezesVanstonePrivateKey;
}

impl AsymmetricDecryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let a = &ciphertext.point;
        let b1 = ciphertext.first.clone();
        let b2 = ciphertext.second.clone();
        let prime = &key.curve.prime;

        let point = a
            .multiply(&key.x, &key.curve, &service.number_theory_service)
            .context("Failed to calculate Point (c1, c2)")?;
        let (c1, c2) = (point.x, point.y);
        let c1_inverse = service
            .number_theory_service
            .modulo_inverse(&c1, prime)
            .context("Failed to find modulo inverse for c1 during decryption")?;
        let c2_inverse = service
            .number_theory_service
            .modulo_inverse(&c2, prime)
            .context("Failed to find modulo inverse for c2 during decryption")?;

        debug!(
            "MV: Entschlüsselung mit c1 = {}, c2 = {}, b1 = {:?} und b2 = {:?}",
            c1, c2, b1, b2
        );

        let m1 = b1.and_then(|b| Some((b * c1_inverse) % prime));
        let m2 = b2.and_then(|b| Some((b * c2_inverse) % prime));

        Ok(MenezesVanstonePlaintext {
            first: m1,
            second: m2,
        })
    }
}

impl<'a> Signer<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = str;
    type Output = Result<MenezesVanstoneSignature>;
    type Key = MenezesVanstonePrivateKey;

    fn sign(
        key: &Self::Key,
        message: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let curve = &key.curve;
        let q = &curve.order_of_subgroup;

        let hashed_message = sha256(message);

        // Schleife, bis r und s jeweils ungleich 0 sind.
        loop {
            let k = &service.take_random_number_in_range(&1.into(), &q.decrement());
            let point = curve
                .generator
                .multiply(k, curve, &service.number_theory_service)
                .context("Failed to calculate Point (c1, c2)")?;

            ensure!(
                !point.is_infinite,
                "Calculated point is infinite, but cannot be since k < |H|. With k = {}",
                k
            );

            let r = point.x.rem_euclid(q);
            if r.is_zero() {
                continue;
            }
            let inverse_k = service.number_theory_service.modulo_inverse(k, q).unwrap();
            let s = (inverse_k * (&hashed_message + &key.x * &r)).rem_euclid(q);
            if s.is_zero() {
                continue;
            }
            debug!("MV: Signatur mit k = {}, r = {} und s = {}", k, r, s);
            return Ok(MenezesVanstoneSignature { r, s });
        }
    }
}

impl<'a> Verifier<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Signature = MenezesVanstoneSignature;
    type Message = str;
    type Output = Result<bool>;
    type Key = MenezesVanstonePublicKey;

    fn verify(
        key: &Self::Key,
        signature: &Self::Signature,
        message: &Self::Message,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output {
        let curve = &key.curve;
        let r = &signature.r;
        let s = &signature.s;
        let q = &curve.order_of_subgroup;

        let hashed_message = sha256(message);
        let w = &service.number_theory_service.modulo_inverse(s, q).unwrap();
        let u1 = (hashed_message * w).rem_euclid(q);
        let u2 = (r * w).rem_euclid(q);

        let first_point = curve
            .generator
            .multiply(&u1, curve, &service.number_theory_service)
            .context("Failed to calculate first point")?;
        let second_point = key
            .y
            .multiply(&u2, curve, &service.number_theory_service)
            .context("Failed to calculate second point")?;
        let point = first_point
            .add(&second_point, curve, &service.number_theory_service)
            .context("Failed to calculate verification point")?;

        let v = point.x.rem_euclid(q);

        debug!(
            "MV: Verifikation mit r = {}, s = {}, u1 = {}, u2 = {}, v = {}",
            r, s, u1, u2, v
        );

        Ok(v == *r)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;

    use super::*;

    #[test]
    fn test_menezes_vanstone_encryption_decryption() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = 7; //rand::thread_rng().gen_range(1..30);
        let modul_width = 128; //rand::thread_rng().gen_range(4..256);
        let random_seed = 300; //rand::thread_rng().gen_range(1..1000);
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, &service).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // 3 und 5 sind definitiv kleiner als jeder generierte 4Bit Modul
        let plaintext = MenezesVanstonePlaintext {
            first: Some(3.into()),
            second: Some(5.into()),
        };

        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, &service).unwrap();
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, &service).unwrap();
        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    #[ignore] // TODO: Läuft manchmal unendlich lange
    fn test_encryption_decryption_fails_when_message_greater_prime() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = rand::thread_rng().gen_range(1..30);
        let modul_width = rand::thread_rng().gen_range(4..100);
        let random_seed = rand::thread_rng().gen_range(1..1000);
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, &service).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let value_bigger_prime: BigInt = BigInt::from(2).pow(modul_width) + 1;
        let plaintext = MenezesVanstonePlaintext {
            first: Some(value_bigger_prime.clone()),
            second: Some(value_bigger_prime),
        };

        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, &service).unwrap();
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, &service).unwrap();
        assert_ne!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_sign_verify_happyflow() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = 5; //rand::thread_rng().gen_range(1..30);
        let modul_width = 16; //rand::thread_rng().gen_range(4..16);
        let random_seed = 73; //rand::thread_rng().gen_range(1..1000);
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, &service).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        assert!(public_key.curve.has_point(&public_key.curve.generator));
        let service = NumberTheoryWithPrngService::new(Fast, 13);

        let message = "Hello World!";
        let signature = MenezesVanstoneScheme::sign(&private_key, message, &service).unwrap();
        let is_verified =
            MenezesVanstoneScheme::verify(&public_key, &signature, message, &service).unwrap();
        assert!(is_verified);
    }

    #[test]
    fn test_invalid_n_value_error() {
        // Testet, ob ein Fehler zurückgegeben wird, wenn n = 0 ist
        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let result = MenezesVanstoneScheme::generate_keypair(0, 128, 40, &service);
        match result {
            Err(err) => match err.downcast_ref::<MenezesVanstoneError>() {
                Some(&MenezesVanstoneError::InvalidNValueError(_)) => assert!(true),
                _ => assert!(false, "Expected InvalidNValueError"),
            },
            _ => assert!(false, "Expected an error"),
        }
    }

    #[test]
    fn test_invalid_modulus_width_error() {
        // Testet, ob ein Fehler zurückgegeben wird, wenn die Breite des Moduls <= 3 ist
        let service = NumberTheoryWithPrngService::new(Fast, 13);
        let result = MenezesVanstoneScheme::generate_keypair(5, 3, 40, &service);
        match result {
            Err(err) => match err.downcast_ref::<MenezesVanstoneError>() {
                Some(&MenezesVanstoneError::InvalidModulusWidthError(_)) => assert!(true),
                _ => assert!(false, "Expected InvalidModulusWidthError"),
            },
            _ => assert!(false, "Expected an error"),
        }
    }

    #[test]
    fn test_sign_verify_invalid_signature() {
        // Testet, ob Signaturprüfung fehlschlägt, wenn die Signatur ungültig ist
        let n = 5;
        let modul_width = 16;
        let random_seed = 73;
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, &service).unwrap();
        let public_key = key_pair.public_key;
        let message = "Hello My Friend!";

        // Manipulation der Signatur
        let invalid_signature = MenezesVanstoneSignature {
            r: BigInt::from(12345),
            s: BigInt::from(67890),
        };
        let is_verified = MenezesVanstoneScheme::verify(
            &public_key,
            &invalid_signature,
            message,
            &NumberTheoryWithPrngService::new(Fast, 13),
        )
        .unwrap();
        assert!(!is_verified);
    }

    #[test]
    fn test_verify_invalid_message() {
        // Testet, ob Signaturprüfung fehlschlägt, wenn die Nachricht nicht übereinstimmt
        let n = 5;
        let modul_width = 16;
        let random_seed = 73;
        let service = NumberTheoryWithPrngService::new(Fast, random_seed);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, &service).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let service = NumberTheoryWithPrngService::new(Fast, 13);

        let message = "Hello my Friend!";
        let signature = MenezesVanstoneScheme::sign(&private_key, message, &service).unwrap();

        let another_message = "I hate you the most!";

        let is_verified =
            MenezesVanstoneScheme::verify(&public_key, &signature, another_message, &service)
                .unwrap();
        assert!(!is_verified);
    }
}
