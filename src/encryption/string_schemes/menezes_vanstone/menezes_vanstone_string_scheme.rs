use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor,
};

use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::{
    MenezesVanstoneCiphertext, MenezesVanstonePlaintext, MenezesVanstoneScheme,
};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::encryption::string_schemes::menezes_vanstone::keys::{
    MenezesVanstoneStringPrivateKey, MenezesVanstoneStringPublicKey,
};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;

pub struct MenezesVanstoneStringScheme {}

impl EncryptionScheme for MenezesVanstoneStringScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneStringScheme {}

// TODO: KeyGen für MenezesVanstoneScheme implementieren

impl<'a> Encryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    type Input = str;
    type Output = String;
    type Key = MenezesVanstoneStringPublicKey;
}

impl AsymmetricEncryptor<MenezesVanstoneStringScheme> for MenezesVanstoneStringScheme {
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: NumberTheoryService,
    ) -> Self::Output {
        unimplemented!();

        // Den String auf MenezesVanstonePlaintext mappen

        let message = MenezesVanstonePlaintext {
            first: Default::default(),
            second: Default::default(),
        };

        let ciphertext = MenezesVanstoneScheme::encrypt(&key.mv_key, &message, service);

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
