use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey,
    PublicKey, SignatureKey, VerificationKey,
};
use crate::encryption::core::rsa::keys;
use crate::encryption::encryption_types::Key;
use crate::encryption::string_schemes::rsa::rsa_with_string_service::RsaWithStringScheme;
use keys::{RsaPrivateKey, RsaPublicKey};

#[derive(Clone, Debug)]
pub struct RsaWithStringPublicKey {
    pub rsa_public_key: RsaPublicKey,
    pub radix: u32,
}

impl Key<RsaWithStringScheme> for RsaWithStringPublicKey {}

impl AsymmetricKey<RsaWithStringScheme> for RsaWithStringPublicKey {}

impl PublicKey<RsaWithStringScheme> for RsaWithStringPublicKey {}

impl AsymmetricEncryptionKey<RsaWithStringScheme> for RsaWithStringPublicKey {}

impl VerificationKey<RsaWithStringScheme> for RsaWithStringPublicKey {}

#[derive(Clone, Debug)]
pub struct RsaWithStringPrivateKey {
    pub rsa_private_key: RsaPrivateKey,
    pub radix: u32,
}

impl Key<RsaWithStringScheme> for RsaWithStringPrivateKey {}

impl AsymmetricKey<RsaWithStringScheme> for RsaWithStringPrivateKey {}

impl PrivateKey<RsaWithStringScheme> for RsaWithStringPrivateKey {}

impl AsymmetricDecryptionKey<RsaWithStringScheme> for RsaWithStringPrivateKey {}

impl SignatureKey<RsaWithStringScheme> for RsaWithStringPrivateKey {}

#[derive(Clone, Debug)]
pub struct RsaWithStringKeyPair {
    pub public_key: RsaWithStringPublicKey,
    pub private_key: RsaWithStringPrivateKey,
}

impl AsymmetricKeyPair<RsaWithStringPublicKey, RsaWithStringPrivateKey, RsaWithStringScheme>
    for RsaWithStringKeyPair
{
    fn public(&self) -> RsaWithStringPublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> RsaWithStringPrivateKey {
        self.private_key.clone()
    }
}
