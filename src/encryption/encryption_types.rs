/// Ein Verschlüsselungsschema.
pub trait EncryptionScheme {}

/// Ein Schlüssel für das Verschlüsselungsschema.
pub trait Key<T: EncryptionScheme> {}

/// Ein Verschlüsselungsverfahren.
pub trait Encryptor<T: EncryptionScheme> {
    type Input;
    type Output;
    type Key: Key<T>;
}

/// Ein Entschlüsselungsverfahren.
pub trait Decryptor<T: EncryptionScheme> {
    type Input;
    type Output;
    type Key: Key<T>;
}
