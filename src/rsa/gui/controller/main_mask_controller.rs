use crate::rsa::gui::{model, view};

pub struct MainMaskController {
    model: model::crypto_operations::CryptoOperations,
    view: view::main_mask_view::MainMaskView
}

impl MainMaskController {
    pub fn generate_keys(&self) {
        self.model.generate_keys();
        self.view.display_public_keys();
    }

    pub fn display_alice_mask(&self) {
        // Implementierung zur Anzeige von Alice's Maske
    }

    pub fn display_bob_mask(&self) {
        // Implementierung zur Anzeige von Bob's Maske
    }
}