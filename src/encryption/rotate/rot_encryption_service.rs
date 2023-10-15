use crate::encryption::encryption_service::EncryptionService;

/// Eine Implementierung des EncryptionService-Traits, welches
/// simple Dummy Daten mit dem Rotate Chiffre verwendet.
pub struct RotEncryptionService {
    rotation: u8,
}
impl RotEncryptionService {
    ///
    /// Erstellt eine neue Instanz des RotEncryptionService.
    ///
    /// # Argumente
    ///
    /// * `rotation` - Die Anzahl der zu verschiebenden Stellen.
    ///
    pub fn new(rotation: u8) -> RotEncryptionService {
        RotEncryptionService { rotation }
    }
}

impl EncryptionService for RotEncryptionService {
    fn encrypt(&self, message: &String) -> String {
        rotate_forward(message, &self.rotation)
    }

    fn decrypt(&self, message: &String) -> String {
        rotate_backward(message, &self.rotation)
    }

    fn sign(&self, message: &String) -> String {
        panic!("Nicht implementiert!")
    }

    fn verify(&self, message: &String) -> String {
        panic!("Nicht implementiert!")
    }

}

fn rotate_forward(text: &str, shift: &u8) -> String {
    text.chars()
        .map(|c| match c {
            'a'..='z' => ((c as u8 - b'a' + shift) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + shift) % 26 + b'A') as char,
            _ => c,
        })
        .collect()
}

fn rotate_backward(text: &str, shift: &u8) -> String {
    text.chars()
        .map(|c| match c {
            'a'..='z' => ((c as u8 - b'a' - shift) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' - shift) % 26 + b'A') as char,
            _ => c,
        })
        .collect()
}
