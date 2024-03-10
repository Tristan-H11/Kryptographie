use crate::encryption::asymmetric_encryption_types::AsymmetricEncryptionScheme;
use crate::encryption::encryption_types::EncryptionScheme;

pub struct MenezesVanstoneScheme{}

impl EncryptionScheme for MenezesVanstoneScheme {}

impl AsymmetricEncryptionScheme for MenezesVanstoneScheme{}