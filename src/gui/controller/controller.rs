use druid::{Env, Event};
use crate::gui::gui::UPDATE_PUBLIC_KEY;
use crate::gui::gui::CALCULATE_PUBLIC_KEY;
use crate::gui::model::model::{AppState, View};
use crate::gui::gui::{ENCRYPT, SIGN, DECRYPT, SEND_MESSAGE, CLEAR};

pub struct Controller;

impl Controller {

    // Übergeordnete Handler Ebene
    pub fn handle_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match app_state.current_view {
            View::HauptMenu => self.handle_haupt_menu_event(event, app_state, _env),
            View::Alice => self.handle_alice_event(event, app_state, _env),
            View::Bob => self.handle_bob_event(event, app_state, _env),
        }
    }

    // Masken handler
    fn handle_haupt_menu_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(CALCULATE_PUBLIC_KEY) => {
                self.calculate_public_key(app_state);
            },
            Event::Command(cmd) if cmd.is(UPDATE_PUBLIC_KEY) => {
                if let Some(public_key) = cmd.get(UPDATE_PUBLIC_KEY) {
                    app_state.haupt_menu.ausgabe_oeff_schluessel = public_key.clone();
                }
            },
            _ => (),
        }
    }
    fn handle_alice_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ENCRYPT) => {
                self.encrypt_alice(app_state);
            },
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_alice(app_state);
            },
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_alice(app_state);
            },
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_alice(app_state);
            },
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_alice(app_state);
            },
            _ => (),
        }
    }
    fn handle_bob_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ENCRYPT) => {
                self.encrypt_bob(app_state);
            },
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_bob(app_state);
            },
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_bob(app_state);
            },
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_bob(app_state);
            },
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_bob(app_state);
            },
            _ => (),
        }
    }

    //Allgemeine Funktion
    fn calculate_public_key(&mut self, app_state: &mut AppState) {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        app_state.haupt_menu.public_key_alice = "1234567890".to_string();
        app_state.haupt_menu.public_key_bob = "0987654321".to_string();
    }

    //Alice Funktion
    fn encrypt_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Verschlüsselung für Alice
    }
    fn sign_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Signierung für Alice
    }
    fn decrypt_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Entschlüsselung für Alice
    }
    fn send_message_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik zum Senden der Nachricht für Alice
    }
    fn clear_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Alice
    }


    //Bob Funktion
    fn encrypt_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Verschlüsselung für Bob
    }
    fn sign_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Signierung für Bob
    }
    fn decrypt_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Entschlüsselung für Bob
    }
    fn send_message_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik zum Senden der Nachricht für Bob
    }
    fn clear_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Bob
    }
}