use crate::api::endpoints::mv::{MvKeyPair, MvPrivateKey, MvPublicKey};
use num::BigInt;

use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey,
    PublicKey, SignatureKey, VerificationKey,
};
use crate::encryption::core::menezes_vanstone::menezes_vanstone_scheme::MenezesVanstoneScheme;
use crate::encryption::encryption_types::Key;
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;

#[derive(Clone, Debug)]
pub struct MenezesVanstonePublicKey {
    pub curve: SecureFiniteFieldEllipticCurve,
    pub y: FiniteFieldEllipticCurvePoint,
}

impl From<MvPublicKey> for MenezesVanstonePublicKey {
    /// Mapped die Bean in das Domain-Modell
    fn from(mv_public_key: MvPublicKey) -> Self {
        let curve = SecureFiniteFieldEllipticCurve::from(mv_public_key.curve);
        let y = FiniteFieldEllipticCurvePoint::from(mv_public_key.y);
        MenezesVanstonePublicKey { curve, y }
    }
}

impl Key<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl AsymmetricKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl PublicKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl AsymmetricEncryptionKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}
impl VerificationKey<MenezesVanstoneScheme> for MenezesVanstonePublicKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstonePrivateKey {
    pub curve: SecureFiniteFieldEllipticCurve,
    pub x: BigInt,
}

impl From<MvPrivateKey> for MenezesVanstonePrivateKey {
    /// Mapped die Bean in das Domain-Modell
    fn from(mv_private_key: MvPrivateKey) -> Self {
        let curve = SecureFiniteFieldEllipticCurve::from(mv_private_key.curve);
        let x = mv_private_key.x.parse().unwrap();
        MenezesVanstonePrivateKey { curve, x }
    }
}

impl Key<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl AsymmetricKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl PrivateKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl AsymmetricDecryptionKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

impl SignatureKey<MenezesVanstoneScheme> for MenezesVanstonePrivateKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstoneKeyPair {
    pub public_key: MenezesVanstonePublicKey,
    pub private_key: MenezesVanstonePrivateKey,
}

impl From<MvKeyPair> for MenezesVanstoneKeyPair {
    /// Mapped die Bean in das Domain-Modell
    fn from(mv_key_pair: MvKeyPair) -> Self {
        let public_key = MenezesVanstonePublicKey::from(mv_key_pair.public_key);
        let private_key = MenezesVanstonePrivateKey::from(mv_key_pair.private_key);
        MenezesVanstoneKeyPair {
            public_key,
            private_key,
        }
    }
}

impl AsymmetricKeyPair<MenezesVanstonePublicKey, MenezesVanstonePrivateKey, MenezesVanstoneScheme>
    for MenezesVanstoneKeyPair
{
    fn public(&self) -> MenezesVanstonePublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> MenezesVanstonePrivateKey {
        self.private_key.clone()
    }
}
