use crate::encryption::block_chiffre::from_decimal_block_scheme::FromDecimalBlockScheme;
use crate::encryption::block_chiffre::to_decimal_block_scheme::ToDecimalBlockScheme;
use crate::encryption::encryption_types::Key;
use crate::encryption::symmetric_encryption_types::{
    SymmetricDecryptionKey, SymmetricEncryptionKey, SymmetricKey,
};

// Ein Schlüssel, welcher für Verschlüsselungsschemata verwendet wird,
// die zwischen Blöcken in Dezimalform und Klartext zu einer Basis radix konvertieren.
pub struct DecimalUnicodeConversionSchemeKey {
    pub block_size: usize,
    pub radix: u32,
}

// Typisierung für ToDecimalBlockScheme
impl Key<ToDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricKey<ToDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricEncryptionKey<ToDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricDecryptionKey<ToDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

// Typisierung für FromDecimalBlockScheme
impl Key<FromDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricKey<FromDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricEncryptionKey<FromDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}

impl SymmetricDecryptionKey<FromDecimalBlockScheme> for DecimalUnicodeConversionSchemeKey {}
