use crate::encryption::encryption_service::EncryptionService;

/// Eine Implementierung des EncryptionService-Traits für RSA.
pub struct RsaEncryptionService {
    pub public_key: String,
    pub private_key: String,
}

impl RsaEncryptionService {
    ///
    /// Erstellt eine neue Instanz des RsaEncryptionService.
    ///
    /// # Argumente
    ///
    /// * `public_key` - Der öffentliche Schlüssel.
    /// * `private_key` - Der private Schlüssel.
    ///
    pub fn new(public_key: String, private_key: String) -> RsaEncryptionService {
        RsaEncryptionService {
            public_key,
            private_key,
        }
    }
}

impl EncryptionService for RsaEncryptionService {
    fn encrypt(&self, message: &String) -> String {
        todo!("Implementiere diese Funktion!")
    }

    fn decrypt(&self, message: &String) -> String {
        todo!("Implementiere diese Funktion!")
    }

    fn sign(&self, message: &String) -> String {
        todo!("Implementiere diese Funktion!")
    }

    fn verify(&self, signature: &String, message: &String) -> bool {
        todo!("Implementiere diese Funktion!")
    }
}
