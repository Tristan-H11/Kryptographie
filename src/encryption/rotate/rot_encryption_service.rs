use crate::encryption::encryption_service::EncryptionService;

/// Eine Implementierung des EncryptionService-Traits, welches
/// simple Dummy Daten mit dem Rotate Chiffre verwendet.
pub struct RotEncryptionService {
    message: String,
    rotation: u8
}
impl RotEncryptionService {
    ///
    /// Erstellt eine neue Instanz des RotEncryptionService.
    ///
    /// # Argumente
    ///
    /// * `input` - Der zu verschlüsselnde String.
    /// * `rotation` - Die Anzahl der zu verschiebenden Stellen.
    ///
    pub fn new(message: String, rotation: String) -> RotEncryptionService {
        let rotation = rotation.trim().parse::<u8>().unwrap();
        RotEncryptionService { message, rotation }
    }
}

impl EncryptionService for RotEncryptionService {
    fn encrypt(&self) -> String {
        rotate_forward(&self.message, &self.rotation)
    }

    fn decrypt(&self) -> String {
        rotate_backward(&self.message, &self.rotation)
    }

    fn sign(&self) -> String {
        format!("{} -- signiert!", &self.message)
    }

    fn verify(&self) -> String {
        panic!("Nicht implementiert!")
    }

    fn generate_keypair(&self) -> (String, String) {
        (self.rotation.to_string(), self.rotation.to_string())
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

