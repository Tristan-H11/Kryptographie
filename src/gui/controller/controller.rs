use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel};

pub struct Controller;

impl Controller {
    //Allgemeine Funktion
    pub(crate) fn calculate_public_key(haupt_menu: &mut HauptMenuModel) {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        haupt_menu.public_key_alice = "0123456789".to_string();
        haupt_menu.public_key_bob = "0987654321".to_string();
    }

    pub(crate) fn update_public_key(_haupt_menu: &mut HauptMenuModel) {
        // todo -- Logik zum Aktualisieren des öffentlichen Schlüssels
    }

    //Alice Funktion
    pub(crate) fn encrypt_alice(_alice: &mut AliceModel) {
        // todo -- Logik für Verschlüsselung für Alice
    }
    pub(crate) fn sign_alice(_alice: &mut AliceModel) {
        // todo -- Logik für Signierung für Alice
    }
    pub(crate) fn decrypt_alice(_alice: &mut AliceModel) {
        // todo -- Logik für Entschlüsselung für Alice
    }
    pub(crate) fn send_message_alice(_alice: &mut AliceModel) {
        // todo -- Logik zum Senden der Nachricht für Alice
    }
    pub(crate) fn clear_alice(_alice: &mut AliceModel) {
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Alice
    }

    //Bob Funktion
    pub(crate) fn encrypt_bob(_bob: &mut BobModel) {
        // todo -- Logik für Verschlüsselung für Bob
    }
    pub(crate) fn sign_bob(_bob: &mut BobModel) {
        // todo -- Logik für Signierung für Bob
    }
    pub(crate) fn decrypt_bob(_bob: &mut BobModel) {
        // todo -- Logik für Entschlüsselung für Bob
    }
    pub(crate) fn send_message_bob(_bob: &mut BobModel) {
        // todo -- Logik zum Senden der Nachricht für Bob
    }
    pub(crate) fn clear_bob(_bob: &mut BobModel) {
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Bob
    }
}
