use crate::encryption::encryption_service::EncryptionService;
use druid::Data;

/// Eine Implementierung des EncryptionService-Traits, welches
/// simple Dummy Daten mit dem Rotate Chiffre verwendet.
pub struct RotEncryptionService {
    pub public_key: u8,
    pub private_key: u8,
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
        RotEncryptionService {
            public_key: rotation,
            private_key: rotation,
        }
    }
}

impl EncryptionService for RotEncryptionService {
    fn encrypt(&self, message: &String) -> String {
        rotate_forward(message, &self.public_key)
    }

    fn decrypt(&self, message: &String) -> String {
        rotate_backward(message, &self.private_key)
    }

    fn sign(&self, _message: &String) -> String {
        panic!("Nicht implementiert!")
    }

    fn verify(&self, _message: &String) -> String {
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

impl Clone for RotEncryptionService {
    fn clone(&self) -> Self {
        RotEncryptionService {
            public_key: self.public_key,
            private_key: self.private_key,
        }
    }
}

impl Data for RotEncryptionService {
    fn same(&self, other: &Self) -> bool {
        self.public_key == other.public_key && self.private_key == other.private_key
    }
}
