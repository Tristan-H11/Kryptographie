use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey,
    PublicKey, SignatureKey, VerificationKey,
};
use crate::encryption::rsa::rsa_scheme::RsaScheme;

use bigdecimal::num_bigint::BigInt;
use crate::encryption::encryption_types::Key;

#[derive(Clone, Debug)]
pub struct RsaPublicKey {
    pub e: BigInt,
    pub n: BigInt,
}

impl Key<RsaScheme> for RsaPublicKey {}

impl AsymmetricKey<RsaScheme> for RsaPublicKey {}

impl PublicKey<RsaScheme> for RsaPublicKey {}

impl AsymmetricEncryptionKey<RsaScheme> for RsaPublicKey {}

impl VerificationKey<RsaScheme> for RsaPublicKey {}

#[derive(Clone, Debug)]
pub struct RsaPrivateKey {
    pub d: BigInt,
    pub n: BigInt,
}

impl Key<RsaScheme> for RsaPrivateKey {}

impl AsymmetricKey<RsaScheme> for RsaPrivateKey {}

impl PrivateKey<RsaScheme> for RsaPrivateKey {}

impl AsymmetricDecryptionKey<RsaScheme> for RsaPrivateKey {}

impl SignatureKey<RsaScheme> for RsaPrivateKey {}

#[derive(Clone, Debug)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

impl AsymmetricKeyPair<RsaPublicKey, RsaPrivateKey, RsaScheme> for RsaKeyPair {
    fn public(&self) -> RsaPublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> RsaPrivateKey {
        self.private_key.clone()
    }
}
