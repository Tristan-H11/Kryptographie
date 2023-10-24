pub trait Encryption {
    ///
    /// Verschlüsselt einen übergebenen String.
    ///
    fn encrypt(&self, message: &String) -> String;
}

pub trait Decryption {
    ///
    /// Entschlüsselt einen übergebenen String.
    ///
    fn decrypt(&self, message: &String) -> String;
}

pub trait Signing {
    ///
    /// Signiert einen übergebenen String.
    ///
    fn sign(&self, message: &String) -> String;
}

pub trait Verification {
    ///
    /// Überprüft die Signatur eines übergebenen Strings.
    ///
    fn verify(&self, signature: &String, message: &String) -> bool;
}
