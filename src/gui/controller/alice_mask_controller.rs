use std::sync::Arc;
use druid::{Env};
use crate::gui::{model, view};
use crate::gui::model::model::{AppState, CryptoOperations};

pub struct AliceMaskController {
    crypto_operations: CryptoOperations,
    view: view::alice_mask_view::AliceMaskView,
}

impl AliceMaskController {
    pub fn new(crypto_operations: CryptoOperations, view: view::alice_mask_view::AliceMaskView) -> Self {
        Self { crypto_operations, view }
    }

    pub fn display_alice_mask(&self, data: &AppState, _env: &Env) {
        // Implementierung zur Anzeige von Alice's Maske
    }
}
