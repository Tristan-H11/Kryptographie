use anyhow::Context;
use anyhow::{ensure, Result};

use crate::api::endpoints::mv::MvSignatureBean;
use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;
use rand::RngCore;

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
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Slow;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
use crate::shared::errors::MenezesVanstoneError;
use crate::shared::hashing::sha256;

#[derive(Clone, Debug, PartialEq)]
pub struct MenezesVanstonePlaintext {
    pub first: BigInt,
    pub second: BigInt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenezesVanstoneCiphertext {
    pub point: FiniteFieldEllipticCurvePoint,
    pub first: BigInt,
    pub second: BigInt,
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
        let key = DecimalUnicodeConversionSchemeKey {
            block_size: 4,
            radix: 55296,
        };
        let blocks = FromDecimalBlockScheme::decrypt(&signature.string_representation, &key);
        assert_eq!(blocks.len(), 2);
        let r_from_string = blocks[0].clone();
        let s_from_string = blocks[1].clone();

        let r = signature.r.parse().unwrap();
        let s = signature.s.parse().unwrap();

        assert_eq!(r_from_string, r);
        assert_eq!(s_from_string, s);

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
        random_seed: u32,
    ) -> Result<MenezesVanstoneKeyPair> {
        ensure!(n != 0, MenezesVanstoneError::InvalidNValueError(n));
        ensure!(
            modul_width > 3,
            MenezesVanstoneError::InvalidModulusWidthError(modul_width)
        );

        let curve =
            SecureFiniteFieldEllipticCurve::new(n.into(), modul_width, miller_rabin_iterations)
                .context("Failed to create secure elliptic curve")?;

        let prng = PseudoRandomNumberGenerator::new(random_seed, NumberTheoryService::new(Slow)); // TODO übergeben
        let counter = RelaxedCounter::new(1);
        let order_of_subgroup = &curve.order_of_subgroup;
        let (mut x, mut y);
        loop {
            x = prng.take(&1.into(), &order_of_subgroup.decrement(), &counter);
            y = curve
                .generator
                .multiply(&x, &curve)
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
        service: NumberTheoryService,
    ) -> Self::Output {
        let m1 = &plaintext.first;
        let m2 = &plaintext.second;
        let prime = &key.curve.prime;

        let mut rng = rand::thread_rng();
        let random_seed: u16 = rng.next_u32() as u16;

        let random_generator = PseudoRandomNumberGenerator::new(random_seed as u32, service);
        let counter = RelaxedCounter::new(1);
        let curve = &key.curve;

        // Bestimmen von c1 und c2
        let (mut k, mut c1, mut c2);
        loop {
            // Dadurch, dass ein Wert < |H| gewählt wird, ist garantiert, dass der Punkt k*g niemals
            // im Unendlichen liegen wird.
            k = random_generator.take(&1.into(), &curve.order_of_subgroup.decrement(), &counter);
            let point = key
                .y
                .multiply(&k, curve)
                .context("Failed to calculate Point (c1, c2)")?;

            ensure!(
                !point.is_infinite,
                "Calculated point is infinite, but cannot be since k < |H|. With k = {}",
                k
            );

            (c1, c2) = (point.x, point.y);
            // Sind beide Werte ungleich 0, so ist das Paar (c1, c2) gültig
            if !c1.is_zero() && !c2.is_zero() {
                break;
            }
        }
        let a = key
            .curve
            .generator
            .multiply(&k, curve)
            .context("Failed to calculate Point a")?;
        let b1 = (c1 * m1) % prime;
        let b2 = (c2 * m2) % prime;

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
        service: NumberTheoryService,
    ) -> Self::Output {
        let a = &ciphertext.point;
        let b1 = &ciphertext.first;
        let b2 = &ciphertext.second;
        let prime = &key.curve.prime;

        let point = a
            .multiply(&key.x, &key.curve)
            .context("Failed to calculate Point (c1, c2)")?;
        let (c1, c2) = (point.x, point.y);
        let c1_inverse = service
            .modulo_inverse(&c1, prime)
            .context("Failed to find modulo inverse for c1 during decryption")?;
        let c2_inverse = service
            .modulo_inverse(&c2, prime)
            .context("Failed to find modulo inverse for c2 during decryption")?;

        let m1 = (b1 * c1_inverse) % prime;
        let m2 = (b2 * c2_inverse) % prime;

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

    fn sign(key: &Self::Key, message: &Self::Input, service: NumberTheoryService) -> Self::Output {
        let prng = PseudoRandomNumberGenerator::new(17, service); // TODO übergeben
        let counter = RelaxedCounter::new(1);
        let curve = &key.curve;
        let q = &curve.order_of_subgroup;

        let hashed_message = sha256(message);

        // Schleife, bis r und s jeweils ungleich 0 sind.
        loop {
            let k = &prng.take(&1.into(), &q.decrement(), &counter);
            let point = curve
                .generator
                .multiply(k, curve)
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
            let inverse_k = service.modulo_inverse(k, q).unwrap();
            let s = (inverse_k * (&hashed_message + &key.x * &r)).rem_euclid(q);
            if s.is_zero() {
                continue;
            }
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
        service: NumberTheoryService,
    ) -> Self::Output {
        let curve = &key.curve;
        let r = &signature.r;
        let s = &signature.s;
        let q = &curve.order_of_subgroup;

        let hashed_message = sha256(message);
        let w = &service.modulo_inverse(s, q).unwrap();
        let u1 = (hashed_message * w).rem_euclid(q);
        let u2 = (r * w).rem_euclid(q);

        let first_point = curve
            .generator
            .multiply(&u1, curve)
            .context("Failed to calculate first point")?;
        let second_point = key
            .y
            .multiply(&u2, curve)
            .context("Failed to calculate second point")?;
        let point = first_point
            .add(&second_point, curve)
            .context("Failed to calculate verification point")?;

        let v = point.x.rem_euclid(q);
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
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // 3 und 5 sind definitiv kleiner als jeder generierte 4Bit Modul
        let plaintext = MenezesVanstonePlaintext {
            first: 3.into(),
            second: 5.into(),
        };

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, service).unwrap();
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, service).unwrap();
        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_encryption_decryption_fails_when_message_greater_prime() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = rand::thread_rng().gen_range(1..30);
        let modul_width = rand::thread_rng().gen_range(4..256);
        let random_seed = rand::thread_rng().gen_range(1..1000);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let value_bigger_prime: BigInt = BigInt::from(2).pow(modul_width) + 1;
        let plaintext = MenezesVanstonePlaintext {
            first: value_bigger_prime.clone(),
            second: value_bigger_prime,
        };

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, service).unwrap();
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, service).unwrap();
        assert_ne!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_sign_verify_happyflow() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = 5; //rand::thread_rng().gen_range(1..30);
        let modul_width = 16; //rand::thread_rng().gen_range(4..16);
        let random_seed = 73; //rand::thread_rng().gen_range(1..1000);
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        assert!(public_key.curve.has_point(&public_key.curve.generator));

        let message = "Hello World!";
        let signature =
            MenezesVanstoneScheme::sign(&private_key, message, NumberTheoryService::new(Fast))
                .unwrap();
        let is_verified = MenezesVanstoneScheme::verify(
            &public_key,
            &signature,
            message,
            NumberTheoryService::new(Fast),
        )
        .unwrap();
        assert!(is_verified);
    }

    #[test]
    fn test_invalid_n_value_error() {
        // Testet, ob ein Fehler zurückgegeben wird, wenn n = 0 ist
        let result = MenezesVanstoneScheme::generate_keypair(0, 128, 40, 123);
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
        let result = MenezesVanstoneScheme::generate_keypair(5, 3, 40, 123);
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
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed).unwrap();
        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;
        let message = "Hello My Friend!";
        let _signature =
            MenezesVanstoneScheme::sign(&private_key, message, NumberTheoryService::new(Fast))
                .unwrap();
        // Manipulation der Signatur
        let invalid_signature = MenezesVanstoneSignature {
            r: BigInt::from(12345),
            s: BigInt::from(67890),
        };
        let is_verified = MenezesVanstoneScheme::verify(
            &public_key,
            &invalid_signature,
            message,
            NumberTheoryService::new(Fast),
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
        let key_pair =
            MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed).unwrap();

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let message = "Hello my Friend!";
        let signature =
            MenezesVanstoneScheme::sign(&private_key, message, NumberTheoryService::new(Fast))
                .unwrap();

        let another_message = "I hate you the most!";

        let is_verified = MenezesVanstoneScheme::verify(
            &public_key,
            &signature,
            another_message,
            NumberTheoryService::new(Fast),
        )
        .unwrap();
        assert!(!is_verified);
    }
}
