/// Ein Trait, gegen welchen die kryptographische Operationen aufrufen werden können.
pub trait EncryptionService {
    ///
    /// Verschlüsselt einen übergebenen String.
    ///
    fn encrypt(&self, message: &String) -> String;

    ///
    /// Entschlüsselt einen übergebenen String.
    ///
    fn decrypt(&self, message: &String) -> String;

    ///
    /// Signiert einen übergebenen String.
    ///
    fn sign(&self, message: &String) -> String;

    ///
    /// Überprüft die Signatur eines übergebenen Strings.
    ///
    fn verify(&self, message: &String) -> String;
}
