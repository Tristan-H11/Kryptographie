pub trait Encryption {
    ///
    /// Verschlüsselt einen übergebenen String.
    ///
    fn encrypt(&self, message: &str) -> String;
}

pub trait Decryption {
    ///
    /// Entschlüsselt einen übergebenen String.
    ///
    fn decrypt(&self, message: &str) -> String;
}

pub trait Signing {
    ///
    /// Signiert einen übergebenen String.
    ///
    fn sign(&self, message: &str) -> String;
}

pub trait Verification {
    ///
    /// Überprüft die Signatur eines übergebenen Strings.
    ///
    fn verify(&self, signature: &str, message: &str) -> bool;
}
