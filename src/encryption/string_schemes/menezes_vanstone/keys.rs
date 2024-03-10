use crate::encryption::asymmetric_encryption_types::{
    AsymmetricDecryptionKey, AsymmetricEncryptionKey, AsymmetricKey, AsymmetricKeyPair, PrivateKey,
    PublicKey,
};
use crate::encryption::core::menezes_vanstone::keys::{
    MenezesVanstonePrivateKey, MenezesVanstonePublicKey,
};
use crate::encryption::encryption_types::Key;
use crate::encryption::string_schemes::menezes_vanstone::menezes_vanstone_string_scheme::MenezesVanstoneStringScheme;

#[derive(Clone, Debug)]
pub struct MenezesVanstoneStringPublicKey {
    pub mv_key: MenezesVanstonePublicKey,
}

impl Key<MenezesVanstoneStringScheme> for MenezesVanstoneStringPublicKey {}
impl AsymmetricKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPublicKey {}
impl PublicKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPublicKey {}
impl AsymmetricEncryptionKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPublicKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstoneStringPrivateKey {
    pub mv_key: MenezesVanstonePrivateKey,
}

impl Key<MenezesVanstoneStringScheme> for MenezesVanstoneStringPrivateKey {}

impl AsymmetricKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPrivateKey {}

impl PrivateKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPrivateKey {}

impl AsymmetricDecryptionKey<MenezesVanstoneStringScheme> for MenezesVanstoneStringPrivateKey {}

#[derive(Clone, Debug)]
pub struct MenezesVanstoneStringKeyPair {
    pub public_key: MenezesVanstoneStringPublicKey,
    pub private_key: MenezesVanstoneStringPrivateKey,
}

impl
    AsymmetricKeyPair<
        MenezesVanstoneStringPublicKey,
        MenezesVanstoneStringPrivateKey,
        MenezesVanstoneStringScheme,
    > for MenezesVanstoneStringKeyPair
{
    fn public(&self) -> MenezesVanstoneStringPublicKey {
        self.public_key.clone()
    }

    fn private(&self) -> MenezesVanstoneStringPrivateKey {
        self.private_key.clone()
    }
}
