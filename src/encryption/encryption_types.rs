/// Ein Verschlüsselungsschema.
pub trait EncryptionScheme {}

/// Ein Schlüssel für das Verschlüsselungsschema.
pub trait Key<T: EncryptionScheme> {}

/// Ein Verschlüsselungsverfahren.
pub trait Encryptor<'a, T: EncryptionScheme> {
    type Input: ?Sized;
    type Output;
    type Key: Key<T>;
}

/// Ein Entschlüsselungsverfahren.
pub trait Decryptor<'a, T: EncryptionScheme> {
    type Input: ?Sized;
    type Output;
    type Key: Key<T>;
}
