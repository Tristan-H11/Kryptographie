use std::time::SystemTime;

use crate::api::endpoints::mv::MvSignatureBean;
use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;

use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor, Signer, Verifier,
};
use crate::encryption::core::menezes_vanstone::keys::{
    MenezesVanstoneKeyPair, MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
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
        MenezesVanstoneSignature {
            r: signature.r.parse().unwrap(),
            s: signature.s.parse().unwrap(),
        }
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
    ) -> MenezesVanstoneKeyPair {
        assert_ne!(n, 0, "n darf nicht 0 sein, ist aber {}", n); // TODO error Handling
        assert!(
            modul_width > 3,
            "Die Modulbreite muss mindestens 4 Bit betragen, ist aber {}",
            modul_width
        ); // TODO error Handling

        let curve =
            SecureFiniteFieldEllipticCurve::new(n.into(), modul_width, miller_rabin_iterations);

        let prng = PseudoRandomNumberGenerator::new(random_seed, NumberTheoryService::new(Fast)); // TODO übergeben
        let counter = RelaxedCounter::new(1);
        let order_of_subgroup = &curve.order_of_subgroup;
        let (mut x, mut y);
        loop {
            x = prng.take(&1.into(), &order_of_subgroup.decrement(), &counter);
            y = curve.generator.multiply(&x, &curve);
            if !y.x.is_zero() && !y.y.is_zero() {
                break;
            }
        }

        let public_key = MenezesVanstonePublicKey {
            curve: curve.clone(),
            y,
        };

        let private_key = MenezesVanstonePrivateKey { curve, x };

        MenezesVanstoneKeyPair {
            public_key,
            private_key,
        }
    }
}

impl Encryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstonePlaintext;
    type Output = MenezesVanstoneCiphertext;
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

        // TODO Der Seed für die Generierung der Zufallszahl für das Verschlüsseln der Nachricht
        // wird vorerst aus der aktuellen Systemzeit generiert und auf 2^16 begrenzt.
        let random_seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u16;
        let random_generator = PseudoRandomNumberGenerator::new(random_seed as u32, service);
        let counter = RelaxedCounter::new(1);
        let curve = &key.curve;

        // Bestimmen von c1 und c2
        let (mut k, mut c1, mut c2);
        loop {
            // Dadurch, dass ein Wert < |H| gewählt wird, ist garantiert, dass der Punkt k*g niemals
            // im Unendlichen liegen wird.
            k = random_generator.take(&1.into(), &curve.order_of_subgroup.decrement(), &counter);
            let point = key.y.multiply(&k, curve);
            if point.is_infinite {
                panic!("Point is infinite")
            }
            (c1, c2) = (point.x, point.y);
            // Sind beide Werte ungleich 0, so ist das Paar (c1, c2) gültig
            if !c1.is_zero() && !c2.is_zero() {
                break;
            }
        }
        let a = key.curve.generator.multiply(&k, curve);
        let b1 = (c1 * m1) % prime;
        let b2 = (c2 * m2) % prime;

        MenezesVanstoneCiphertext {
            point: a,
            first: b1,
            second: b2,
        }
    }
}

impl Decryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstoneCiphertext;
    type Output = MenezesVanstonePlaintext;
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

        let point = a.multiply(&key.x, &key.curve);
        let (c1, c2) = (point.x, point.y);
        let m1 = (b1 * service.modulo_inverse(&c1, prime).unwrap()) % prime; //TODO Unwrap
        let m2 = (b2 * service.modulo_inverse(&c2, prime).unwrap()) % prime; //TODO Unwrap

        MenezesVanstonePlaintext {
            first: m1,
            second: m2,
        }
    }
}

impl<'a> Signer<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = str;
    type Output = MenezesVanstoneSignature;
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
            let point = curve.generator.multiply(k, curve);
            if point.is_infinite {
                panic!("Point is infinite")
            }
            let r = point.x.rem_euclid(q);
            if r.is_zero() {
                continue;
            }
            let inverse_k = service.modulo_inverse(k, q).unwrap();
            let s = (inverse_k * (&hashed_message + &key.x * &r)).rem_euclid(q);
            if s.is_zero() {
                continue;
            }
            return MenezesVanstoneSignature { r, s };
        }
    }
}

impl<'a> Verifier<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Signature = MenezesVanstoneSignature;
    type Message = str;
    type Output = bool;
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

        let first_point = curve.generator.multiply(&u1, curve);
        let second_point = key.y.multiply(&u2, curve);
        let point = first_point.add(&second_point, curve);

        let v = point.x.rem_euclid(q);
        v == *r
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
        let key_pair = MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed);

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        // 3 und 5 sind definitiv kleiner als jeder generierte 4Bit Modul
        let plaintext = MenezesVanstonePlaintext {
            first: 3.into(),
            second: 5.into(),
        };

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, service);
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, service);
        assert_eq!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_encryption_decryption_fails_when_message_greater_prime() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = rand::thread_rng().gen_range(1..30);
        let modul_width = rand::thread_rng().gen_range(4..256);
        let random_seed = rand::thread_rng().gen_range(1..1000);
        let key_pair = MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed);

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        let value_bigger_prime: BigInt = BigInt::from(2).pow(modul_width) + 1;
        let plaintext = MenezesVanstonePlaintext {
            first: value_bigger_prime.clone(),
            second: value_bigger_prime,
        };

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, service);
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, service);
        assert_ne!(plaintext, decrypted_plaintext);
    }

    #[test]
    fn test_sign_verify_happyflow() {
        // Die Parameter sollen hier für jeden Testlauf zufällig gewählt werden, damit flakiness
        // eher auffällt.
        let n = 5; //rand::thread_rng().gen_range(1..30);
        let modul_width = 16; //rand::thread_rng().gen_range(4..16);
        let random_seed = 73; //rand::thread_rng().gen_range(1..1000);
        let key_pair = MenezesVanstoneScheme::generate_keypair(n, modul_width, 40, random_seed);

        let public_key = key_pair.public_key;
        let private_key = key_pair.private_key;

        assert!(public_key.curve.has_point(&public_key.curve.generator));

        let message = "Hello World!";
        let signature =
            MenezesVanstoneScheme::sign(&private_key, message, NumberTheoryService::new(Fast));
        let is_verified = MenezesVanstoneScheme::verify(
            &public_key,
            &signature,
            message,
            NumberTheoryService::new(Fast),
        );
        assert!(is_verified);
    }
}
