use crate::encryption::block_chiffre::to_decimal_block_scheme::ToDecimalBlockScheme;
use crate::encryption::encryption_types::Key;
use crate::encryption::symmetric_encryption_types::{
    SymmetricDecryptionKey, SymmetricEncryptionKey, SymmetricKey,
};

pub struct ToDecimalKey {
    pub block_size: usize,
    pub radix: u32,
}

impl Key<ToDecimalBlockScheme> for ToDecimalKey {}

impl SymmetricKey<ToDecimalBlockScheme> for ToDecimalKey {}

impl SymmetricEncryptionKey<ToDecimalBlockScheme> for ToDecimalKey {}

impl SymmetricDecryptionKey<ToDecimalBlockScheme> for ToDecimalKey {}
