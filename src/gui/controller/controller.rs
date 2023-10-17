use crate::encryption::encryption_service::EncryptionService;
use crate::encryption::keygen_service::KeyGenService;
use crate::encryption::rotate::rot_encryption_service::RotEncryptionService;
use crate::encryption::rotate::rot_keygen_service::RotKeygenService;
use crate::gui::controller::commands::*;
use crate::gui::model::model::{AppState, View};
use druid::{Env, Event, EventCtx};

pub struct AppController;

impl<W: druid::Widget<AppState>> druid::widget::Controller<AppState, W> for AppController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if data.current_view == View::HauptMenu {
            self.handle_haupt_menu_event(event, data, env);
        } else if data.current_view == View::Alice {
            self.handle_alice_event(event, data, env);
        } else if data.current_view == View::Bob {
            self.handle_bob_event(event, data, env);
        }

        child.event(ctx, event, data, env);
    }
}

impl AppController {
    ///
    /// Behandelt alle Events für das Hauptfenster
    ///
    fn handle_haupt_menu_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(CALCULATE_KEYPAIR_ALICE) => {
                self.calculate_keypair_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(CALCULATE_KEYPAIR_BOB) => {
                self.calculate_keypair_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(SWITCH_TO_ALICE) => {
                app_state.current_view = View::Alice;
            }
            Event::Command(cmd) if cmd.is(SWITCH_TO_BOB) => {
                app_state.current_view = View::Bob;
            }
            _ => (),
        }
    }

    fn handle_alice_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ENCRYPT) => {
                self.encrypt_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_alice(app_state);
            }
            Event::Command(cmd) if cmd.is(SWITCH_TO_HAUPTMENU) => {
                app_state.current_view = View::HauptMenu;
            }
            _ => (),
        }
    }

    fn handle_bob_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ENCRYPT) => {
                self.encrypt_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_bob(app_state);
            }
            Event::Command(cmd) if cmd.is(SWITCH_TO_HAUPTMENU) => {
                app_state.current_view = View::HauptMenu;
            }
            _ => (),
        }
    }

    ///
    /// Berechnet das Schlüsselpaar für Alice.
    ///
    fn calculate_keypair_alice(&mut self, app_state: &mut AppState) {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        // Das ist erstmal nur eine Dummy-Implementierung mittels Rot-Chiffre
        let keygen_service = RotKeygenService::new(app_state.haupt_menu.prime_number_one.clone());
        let (public_key_alice, private_key_alice) = keygen_service.generate_keypair();

        app_state.haupt_menu.public_key_alice = public_key_alice.clone();
        app_state.alice.private_key = private_key_alice.clone();
    }

    ///
    /// Berechnet das Schlüsselpaar für Bob.
    ///
    fn calculate_keypair_bob(&mut self, app_state: &mut AppState) {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        // Das ist erstmal nur eine Dummy-Implementierung mittels Rot-Chiffre
        let keygen_service = RotKeygenService::new(app_state.haupt_menu.prime_number_two.clone());
        let (public_key_bob, private_key_bob) = keygen_service.generate_keypair();

        app_state.haupt_menu.public_key_bob = public_key_bob.clone();
        app_state.bob.private_key = private_key_bob.clone();
    }

    ///
    /// Verschlüsselt die Nachricht von Alice mit Bobs öffentlichem Schlüssel.
    ///
    fn encrypt_alice(&mut self, app_state: &mut AppState) {
        let klartext = app_state.alice.message_klartext.clone();

        let encrypted = self
            .get_encryption_service_bob(app_state)
            .encrypt(&klartext);
        println!("Verschlüsselt: {}", &encrypted);
        app_state.alice.message_klartext = encrypted;
    }

    ///
    /// Signiert die Nachricht von Alice mit ihrem privaten Schlüssel.
    ///
    fn sign_alice(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Signierung für Alice
        println!("Signatur Alice");
    }

    ///
    /// Entschlüsselt die Nachricht von Bob mit Alices privatem Schlüssel.
    ///
    fn decrypt_alice(&mut self, app_state: &mut AppState) {
        let cipher_text = app_state.alice.message_klartext.clone();

        let decrypted = self
            .get_encryption_service_alice(app_state)
            .decrypt(&cipher_text);
        println!("Entschlüsselt: {}", &decrypted);
        app_state.alice.message_klartext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Alice an Bob und löscht das Nachrichten-Feld.
    ///
    fn send_message_alice(&mut self, app_state: &mut AppState) {
        let message = &app_state.alice.message_klartext;
        app_state.bob.message_chiffre = message.clone();
        self.clear_alice(app_state);
        // todo -- Logik zum Senden der Nachricht für Alice
    }

    ///
    /// Löscht die Nachricht von Alice.
    ///
    fn clear_alice(&mut self, app_state: &mut AppState) {
        app_state.alice.message_klartext = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Alice
    }

    ///
    /// Verschlüsselt die Nachricht von Bob mit Alice öffentlichem Schlüssel.
    ///
    fn encrypt_bob(&mut self, app_state: &mut AppState) {
        let klartext = app_state.bob.message_chiffre.clone();

        let encrypted = self
            .get_encryption_service_alice(app_state)
            .encrypt(&klartext);
        println!("Verschlüsselt: {}", &encrypted);
        app_state.bob.message_klartext = encrypted;
    }

    ///
    /// Signiert die Nachricht von Bob mit seinem privaten Schlüssel.
    ///
    fn sign_bob(&mut self, _app_state: &mut AppState) {
        // todo -- Logik für Signierung für Alice
        println!("Signatur Bob");
    }

    ///
    /// Entschlüsselt die Nachricht von Alice mit Bobs privatem Schlüssel.
    ///
    fn decrypt_bob(&mut self, app_state: &mut AppState) {
        let cipher_text = app_state.bob.message_klartext.clone();

        let decrypted = self
            .get_encryption_service_bob(app_state)
            .decrypt(&cipher_text);
        println!("Entschlüsselt: {}", &decrypted);
        app_state.bob.message_klartext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Bob an Alice und löscht das Nachrichten-Feld.
    ///
    fn send_message_bob(&mut self, app_state: &mut AppState) {
        let message = &app_state.bob.message_chiffre;
        app_state.alice.message_chiffre = message.clone();
        self.clear_bob(app_state);
        // todo -- Logik zum Senden der Nachricht für Bob
    }

    ///
    /// Löscht die Nachricht von Bob.
    ///
    fn clear_bob(&mut self, app_state: &mut AppState) {
        app_state.bob.message_klartext = String::new();
        app_state.bob.message_chiffre = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Bob
    }

    ///
    /// Erstellt den EncryptionService für Alice und gibt ihn zurück.
    ///
    fn get_encryption_service_alice(&mut self, app_state: &mut AppState) -> RotEncryptionService {
        let public_key = app_state.haupt_menu.public_key_alice.parse::<u8>().unwrap();
        RotEncryptionService::new(public_key)
    }

    ///
    /// Erstellt den EncryptionService für Bob und gibt ihn zurück.
    ///
    fn get_encryption_service_bob(&mut self, app_state: &mut AppState) -> RotEncryptionService {
        let public_key = app_state.haupt_menu.public_key_bob.parse::<u8>().unwrap();
        RotEncryptionService::new(public_key)
    }
}
