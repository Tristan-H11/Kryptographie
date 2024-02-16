use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey,
    PublicKey,
};
use crate::encryption::el_gamal::el_gamal_scheme::ElGamalScheme;
use crate::encryption::encryption_types::Key;
use bigdecimal::num_bigint::BigInt;

/// Ein öffentlicher ElGamal-Schlüssel für das ElGamal-Kryptosystem in primen Restklassengruppen.
/// Besteht aus dem Modulus p, dem Generator g und dem öffentlichen Wert y.
#[derive(Clone, Debug)]
pub struct ElGamalPublicKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

impl Key<ElGamalScheme> for ElGamalPublicKey {}

impl AsymmetricKey<ElGamalScheme> for ElGamalPublicKey {}

impl PublicKey<ElGamalScheme> for ElGamalPublicKey {}

impl AsymmetricEncryptionKey<ElGamalScheme> for ElGamalPublicKey {}

// TODO: Ist dieser Schlüssel auf für die Verifikation von Signaturen notwendig?

/// Ein privater ElGamal-Schlüssel für das ElGamal-Kryptosystem in primen Restklassengruppen.
/// Besteht aus dem Modulus p und dem Geheimwert x.
#[derive(Clone, Debug)]
pub struct ElGamalPrivateKey {
    pub p: BigInt,
    pub x: BigInt,
}

impl Key<ElGamalScheme> for ElGamalPrivateKey {}

impl AsymmetricKey<ElGamalScheme> for ElGamalPrivateKey {}

impl PrivateKey<ElGamalScheme> for ElGamalPrivateKey {}

impl AsymmetricDecryptionKey<ElGamalScheme> for ElGamalPrivateKey {}

// TODO: Siehe oben. Ist dieser Schlüssel auf für die Signierung von Nachrichten notwendig?

#[derive(Clone, Debug)]
pub struct ElGamalKeyPair {
    pub public_key: ElGamalPublicKey,
    pub private_key: ElGamalPrivateKey,
}

impl AsymmetricKeyPair<ElGamalPublicKey, ElGamalPrivateKey, ElGamalScheme> for ElGamalKeyPair {
    fn public(&self) -> ElGamalPublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> ElGamalPrivateKey {
        self.private_key.clone()
    }
}
