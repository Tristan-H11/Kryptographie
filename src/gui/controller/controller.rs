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
            Event::Command(cmd) if cmd.is(CALCULATE_PUBLIC_KEY) => {
                self.calculate_public_key(app_state);
            }
            Event::Command(cmd) if cmd.is(UPDATE_PUBLIC_KEY) => {
                if let Some(public_key) = cmd.get(UPDATE_PUBLIC_KEY) {
                    app_state.haupt_menu.ausgabe_oeff_schluessel = public_key.clone();
                }
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

    //Allgemeine Funktion
    fn calculate_public_key(&mut self, app_state: &mut AppState) {
        // TODO: Implementiere die Logik zur Berechnung der öffentlichen Schlüssel für Alice und Bob
        // Das ist erstmal nur eine Dummy-Implementierung mittels Rot-Chiffre

        // Alice
        let rot_keygen_service_alice =
            RotKeygenService::new(app_state.haupt_menu.eingabe_p1.clone());
        let (public_key_alice, private_key_alice) = rot_keygen_service_alice.generate_keypair();

        app_state.haupt_menu.public_key_alice = public_key_alice.clone();
        app_state.alice.anzeige_geheimer_schluessel = private_key_alice.clone();

        // Bob
        let rot_keygen_service_bob = RotKeygenService::new(app_state.haupt_menu.eingabe_p2.clone());
        let (public_key_bob, private_key_bob) = rot_keygen_service_bob.generate_keypair();

        app_state.haupt_menu.public_key_bob = public_key_bob.clone();
        app_state.bob.anzeige_geheimer_schluessel = private_key_bob.clone();
    }

    //Alice Funktion
    fn encrypt_alice(&mut self, app_state: &mut AppState) {
        let public_key = app_state.haupt_menu.public_key_alice.parse::<u8>().unwrap();
        let klartext = app_state.alice.eingabe_klartext.clone();
        let service = RotEncryptionService::new(public_key);

        let encrypted = service.encrypt(&klartext);
        app_state.alice.eingabe_klartext = encrypted;
    }

    fn sign_alice(&mut self, app_state: &mut AppState) {
        // todo -- Logik für Signierung für Alice
        println!("Signatur Alice");
    }
    fn decrypt_alice(&mut self, app_state: &mut AppState) {
        let private_key = app_state
            .alice
            .anzeige_geheimer_schluessel
            .parse::<u8>()
            .unwrap();
        let ciphertext = app_state.alice.eingabe_klartext.clone();
        let service = RotEncryptionService::new(private_key);

        let decrypted = service.decrypt(&ciphertext);
        app_state.alice.eingabe_klartext = decrypted;
    }
    fn send_message_alice(&mut self, app_state: &mut AppState) {
        let message = &app_state.alice.eingabe_klartext;
        app_state.bob.eingabe_klartext = message.clone();
        self.clear_alice(app_state);
        // todo -- Logik zum Senden der Nachricht für Alice
    }
    fn clear_alice(&mut self, app_state: &mut AppState) {
        app_state.alice.eingabe_klartext = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Alice
    }

    //Bob Funktion
    fn encrypt_bob(&mut self, app_state: &mut AppState) {
        let public_key = app_state.haupt_menu.public_key_bob.parse::<u8>().unwrap();
        let klartext = app_state.bob.eingabe_klartext.clone();
        let service = RotEncryptionService::new(public_key);

        let encrypted = service.encrypt(&klartext);
        app_state.bob.eingabe_klartext = encrypted;
        // todo -- Logik für Verschlüsselung für Bob
    }
    fn sign_bob(&mut self, app_state: &mut AppState) {
        // todo -- Logik für Signierung für Alice
        println!("Signatur Bob");
    }
    fn decrypt_bob(&mut self, app_state: &mut AppState) {
        let private_key = app_state
            .bob
            .anzeige_geheimer_schluessel
            .parse::<u8>()
            .unwrap();
        let ciphertext = app_state.bob.eingabe_klartext.clone();
        let service = RotEncryptionService::new(private_key);

        let decrypted = service.decrypt(&ciphertext);
        app_state.bob.eingabe_klartext = decrypted;
    }
    fn send_message_bob(&mut self, app_state: &mut AppState) {
        let message = &app_state.bob.eingabe_klartext;
        app_state.alice.eingabe_klartext = message.clone();
        self.clear_bob(app_state);
        // todo -- Logik zum Senden der Nachricht für Bob
    }
    fn clear_bob(&mut self, app_state: &mut AppState) {
        app_state.bob.eingabe_klartext = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Bob
    }
}
