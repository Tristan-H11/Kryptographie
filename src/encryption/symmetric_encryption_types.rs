use crate::encryption::encryption_types::{EncryptionScheme, Encryptor, Key};

/// Ein symmetrisches Verschlüsselungsschema.
pub trait SymmetricEncryptionScheme: EncryptionScheme {}

/// Ein symmetrischer Schlüssel für das symmetrische Verschlüsselungsschema.
pub trait SymmetricKey<T: SymmetricEncryptionScheme>: Key<T> {}

/// Ein Schlüssel zum Entschlüsseln für das symmetrische Verschlüsselungsschema.
pub trait SymmetricDecryptionKey<T: SymmetricEncryptionScheme>: SymmetricKey<T> {}

/// Ein Schlüssel zum Verschlüsseln für das symmetrische Verschlüsselungsschema.
pub trait SymmetricEncryptionKey<T: SymmetricEncryptionScheme>: SymmetricKey<T> {}

/// Ein Schlüsselpaar für das symmetrische Verschlüsselungsschema.
///
/// # Typen
/// * `EncryptionKey` - Der Typ des Schlüssels zum Verschlüsseln.
/// * `DecryptionKey` - Der Typ des Schlüssels zum Entschlüsseln.
/// * `Scheme` - Das symmetrische Verschlüsselungsschema.
///
/// # Methoden
/// * `encryption` - Gibt den Schlüssel zum Verschlüsseln zurück.
/// * `decryption` - Gibt den Schlüssel zum Entschlüsseln zurück.
pub trait SymmetricKeyPair<EncryptionKey, DecryptionKey, Scheme>
where
    EncryptionKey: SymmetricEncryptionKey<Scheme>,
    DecryptionKey: SymmetricDecryptionKey<Scheme>,
    Scheme: SymmetricEncryptionScheme,
{
    fn encryption(&self) -> EncryptionKey;
    fn decryption(&self) -> DecryptionKey;
}

/// Ein Schlüsselgenerator für das symmetrische Verschlüsselungsschema.
pub trait KeyGenerator<EncryptionKey, DecryptionKey, Scheme>
where
    EncryptionKey: SymmetricEncryptionKey<Scheme>,
    DecryptionKey: SymmetricDecryptionKey<Scheme>,
    Scheme: SymmetricEncryptionScheme,
{
    type KeyPair: SymmetricKeyPair<EncryptionKey, DecryptionKey, Scheme>;
    /// Generiert ein Schlüsselpaar für das symmetrische Verschlüsselungsschema.
    fn generate_keypair(&self) -> Self::KeyPair;
}

pub trait SymmetricEncryptor<T: SymmetricEncryptionScheme>: Encryptor<T> {
    /// Verschlüsselt den gegebenen Klartext mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `plaintext` - Der Klartext.
    /// * `key` - Der Schlüssel.
    ///
    /// # Rückgabe
    /// Der Geheimtext.
    fn encrypt(plaintext: Self::Input, key: &Self::Key) -> Self::Output;
}

pub trait SymmetricDecryptor<T: SymmetricEncryptionScheme>: Encryptor<T> {
    /// Entschlüsselt den gegebenen Geheimtext mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `ciphertext` - Der Geheimtext.
    /// * `key` - Der Schlüssel.
    ///
    /// # Rückgabe
    /// Der Klartext.
    fn decrypt(ciphertext: Self::Input, key: &Self::Key) -> Self::Output;
}