/// Ein Trait, gegen welchen die kryptographische Operationen aufrufen werden können.
pub trait EncryptionService {
    ///
    /// Verschlüsselt einen übergebenen String.
    ///
    fn encrypt(&self) -> String;

    ///
    /// Entschlüsselt einen übergebenen String.
    ///
    fn decrypt(&self) -> String;

    ///
    /// Signiert einen übergebenen String.
    ///
    fn sign(&self) -> String;

    ///
    /// Überprüft die Signatur eines übergebenen Strings.
    ///
    fn verify(&self) -> String;

    ///
    /// Erstellt ein Schlüsselpaar.
    ///
    fn generate_keypair(&self) -> (String, String);
}
