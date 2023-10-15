pub trait KeyGenService {
    ///
    /// Erstellt ein Schlüsselpaar.
    ///
    fn generate_keypair(&self) -> (String, String);
}
