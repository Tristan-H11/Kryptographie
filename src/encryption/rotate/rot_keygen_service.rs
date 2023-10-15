use crate::encryption::keygen_service::KeyGenService;

/// Eine Implementierung des KeygenService-Traits, welches eine i8 als Input nimmt und
/// in der Schlüsselgenerierung diese Zahl als public und private key setzt.
pub struct RotKeygenService {
    rotation: i8,
}

impl RotKeygenService {
    ///
    /// Erstellt eine neue Instanz des RotKeygenService.
    ///
    /// # Argumente
    ///
    /// * `input` - Die zu verschlüsselnde Zahl.
    ///
    pub fn new(rotation: String) -> RotKeygenService {
        let rotation = rotation.trim().parse::<i8>().unwrap();
        RotKeygenService { rotation }
    }
}

impl KeyGenService for RotKeygenService {
    fn generate_keypair(&self) -> (String, String) {
        (self.rotation.to_string(), self.rotation.to_string())
    }
}