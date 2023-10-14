use std::sync::{Arc, Mutex};
use druid::{Data, Lens};
use crate::gui::controller::main_mask_controller::MainMaskController;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub(crate) current_mask: Mask,

    pub(crate) main_controller: Arc<Mutex<MainMaskController>>,


    // Main Mask View
    pub(crate) miller_rabin_input: String,
    pub(crate) open_key_result__e_a: String,
    pub(crate) length_p1_input: String,
    pub(crate) length_p2_input: String,
    pub(crate) open_key_result__e_b: String,

    // Alice Mask View
    pub(crate) secret_key_for_alice: String,


    // Bob Mask View
    pub(crate) secret_key_for_bob: String,
}

#[derive(Clone, Data, Lens, PartialEq, Eq)]
pub enum Mask {
    Main,
    Alice,
    Bob,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_mask: Mask::Main,
            main_controller: Arc::new(Mutex::new(MainMaskController::new())),

            miller_rabin_input: String::from(""),
            open_key_result__e_a: String::from(""),
            length_p1_input: String::from(""),
            length_p2_input: String::from(""),
            open_key_result__e_b: String::from(""),
            secret_key_for_alice: String::from(""),
            secret_key_for_bob: String::from(""),
        }
    }
}

#[derive(Clone)]
pub struct CryptoOperations;

impl CryptoOperations {
    pub fn new() -> Self {
        Self {}
    }
    pub(crate) fn generate_keys(&self) {
        // TODO Implementierung der Schlüsselgenerierung
    }

    fn encrypt(&self) {
        // TODO Implementierung der Verschlüsselung
    }

    fn sign_message(&self) {
        // TODO Implementierung der Signaturerstellung
    }

    fn decrypt_and_verify(&self) {
        // TODO Implementierung der Entschlüsselung und Signaturüberprüfung
    }
}