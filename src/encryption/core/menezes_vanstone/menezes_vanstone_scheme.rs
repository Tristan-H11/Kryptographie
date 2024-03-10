use bigdecimal::num_bigint::BigInt;
use crate::encryption::asymmetric_encryption_types::{AsymmetricDecryptor, AsymmetricEncryptionScheme, AsymmetricEncryptor};
use crate::encryption::core::menezes_vanstone::keys::{MenezesVanstonePrivateKey, MenezesVanstonePublicKey};
use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;

pub struct MenezesVanstonePlaintext {
    pub first: BigInt,
    pub second: BigInt,
}

pub struct MenezesVanstoneCiphertext {
    pub point: FiniteFieldEllipticCurvePoint,
    pub first: BigInt,
    pub second: BigInt,
}

pub struct MenezesVanstoneScheme{}

impl EncryptionScheme for MenezesVanstoneScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneScheme{}

// TODO: KeyGen für MenezesVanstoneScheme implementieren

impl Encryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstonePlaintext;
    type Output = MenezesVanstoneCiphertext;
    type Key = MenezesVanstonePublicKey;
}

impl AsymmetricEncryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    fn encrypt(key: &Self::Key, plaintext: &Self::Input, service: NumberTheoryService) -> Self::Output {
        todo!()
    }
}

impl Decryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    type Input = MenezesVanstoneCiphertext;
    type Output = MenezesVanstonePlaintext;
    type Key = MenezesVanstonePrivateKey;
}

impl AsymmetricDecryptor<MenezesVanstoneScheme> for MenezesVanstoneScheme {
    fn decrypt(key: &Self::Key, ciphertext: &Self::Input, service: NumberTheoryService) -> Self::Output {
        todo!()
    }
}