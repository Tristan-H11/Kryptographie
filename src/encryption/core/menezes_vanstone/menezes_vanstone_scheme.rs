use std::str::FromStr;
use std::time::SystemTime;

use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::Zero;

use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor,
};
use crate::encryption::core::menezes_vanstone::keys::{
    MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;

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

pub struct MenezesVanstoneScheme {}

impl EncryptionScheme for MenezesVanstoneScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneScheme {}

// TODO: KeyGen für MenezesVanstoneScheme implementieren

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

        // Bestimmen von c1 und c2
        let (mut k, mut c1, mut c2);
        loop {
            // TODO Achtung! Dieser Wert ist vorerst fest, weil er in den Testfällen einen validen Wert darstellt, solange noch keine
            // Untergruppen bestimmt werden können.
            k = BigInt::from_str("165165550160996878069788804916299463847").unwrap(); // TODO random_generator.take(&1.into(), &order_of_subgroup.decrement(), &counter);
            let point = key.y.multiply(&k, &key.curve);
            (c1, c2) = (point.x, point.y);
            // Sind beide Werte ungleich 0, so ist das Paar (c1, c2) gültig
            if !c1.is_zero() && !c2.is_zero() {
                println!("k: {:?}", k);
                break;
            }
        }
        let a = key.generator.multiply(&k, &key.curve);
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

#[cfg(test)]
mod tests {
    use crate::math_core::ecc::finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
    use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_menezes_vanstone_encryption_decryption() {
        let curve = SecureFiniteFieldEllipticCurve::new(
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

        let private_key = MenezesVanstonePrivateKey { curve, x };

        let plaintext = MenezesVanstonePlaintext {
            first: 123.into(),
            second: 456.into(),
        };

        let service = NumberTheoryService::new(Fast);
        let ciphertext = MenezesVanstoneScheme::encrypt(&public_key, &plaintext, service);
        let decrypted_plaintext =
            MenezesVanstoneScheme::decrypt(&private_key, &ciphertext, service);
        assert_eq!(plaintext, decrypted_plaintext);
    }
}
