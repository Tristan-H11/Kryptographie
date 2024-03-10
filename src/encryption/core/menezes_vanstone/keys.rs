use num::BigInt;

use crate::encryption::asymmetric_encryption_types::{AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey, PublicKey};
use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::MenezesVanstoneScheme;
use crate::encryption::encryption_types::Key;
use crate::math_core::ecc::finite_field_elliptic_curve::FiniteFieldEllipticCurve;
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;

#[derive(Clone, Debug)]
pub struct MenezesVanstonePublicKey {
    pub curve: FiniteFieldEllipticCurve,
    pub generator: FiniteFieldEllipticCurvePoint,
    pub y: FiniteFieldEllipticCurvePoint,
}

impl Key<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl AsymmetricKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl PublicKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl AsymmetricEncryptionKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstonePrivateKey {
    pub curve: FiniteFieldEllipticCurve,
    pub x: BigInt,
}

impl Key<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl AsymmetricKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl PrivateKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl AsymmetricDecryptionKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstoneKeyPair {
    pub public_key: MenezesVanstonePublicKey,
    pub private_key: MenezesVanstonePrivateKey,
}

impl AsymmetricKeyPair<MenezesVanstonePublicKey, MenezesVanstonePrivateKey, MenezesVanstoneScheme> for MenezesVanstoneKeyPair {

    fn public(&self) -> MenezesVanstonePublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> MenezesVanstonePrivateKey {
        self.private_key.clone()
    }
}