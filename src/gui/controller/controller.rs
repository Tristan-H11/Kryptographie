use crate::gui::controller::commands::*;
use crate::gui::model::model::{AliceModel, AppState, BobModel, View};
use druid::{Env, Event, EventCtx};
use std::fmt::format;

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
                self.encrypt_alice(&mut app_state.alice);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_alice(&mut app_state.alice);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_alice(&mut app_state.alice);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_alice(&mut app_state.alice, &mut app_state.bob);
            }
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_alice(&mut app_state.alice);
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
                self.encrypt_bob(&mut app_state.bob);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.sign_bob(&mut app_state.bob);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.decrypt_bob(&mut app_state.bob);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.send_message_bob(&mut app_state.bob, &mut app_state.alice);
            }
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.clear_bob(&mut app_state.bob);
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
        app_state.haupt_menu.public_key_alice = "1234567890".to_string();
        app_state.haupt_menu.public_key_bob = "0987654321".to_string();
    }

    //Alice Funktion
    fn encrypt_alice(&mut self, alice: &mut AliceModel) {
        alice.eingabe_klartext = String::from("Unyyb Obo");
        // todo -- Logik für Verschlüsselung für Alice
    }
    fn sign_alice(&mut self, alice: &mut AliceModel) {
        alice.eingabe_klartext = format!("{} Signatur", alice.eingabe_klartext);
        // todo -- Logik für Signierung für Alice
    }
    fn decrypt_alice(&mut self, alice: &mut AliceModel) {
        alice.eingabe_klartext = String::from("Hallo Bob");
        // todo -- Logik für Entschlüsselung für Alice
    }
    fn send_message_alice(&mut self, alice: &mut AliceModel, bob: &mut BobModel) {
        let message = &alice.eingabe_klartext;
        bob.eingabe_klartext = message.clone();
        alice.eingabe_klartext = String::new();
        // todo -- Logik zum Senden der Nachricht für Alice
    }
    fn clear_alice(&mut self, alice: &mut AliceModel) {
        alice.eingabe_klartext = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Alice
    }

    //Bob Funktion
    fn encrypt_bob(&mut self, bob: &mut BobModel) {
        bob.eingabe_klartext = String::from("Unyyb Nyvpr");
        // todo -- Logik für Verschlüsselung für Bob
    }
    fn sign_bob(&mut self, bob: &mut BobModel) {
        bob.eingabe_klartext = format!("{} Signatur", bob.eingabe_klartext);
        // todo -- Logik für Signierung für Bob
    }
    fn decrypt_bob(&mut self, bob: &mut BobModel) {
        bob.eingabe_klartext = String::from("Hallo Alice");
        // todo -- Logik für Entschlüsselung für Bob
    }
    fn send_message_bob(&mut self, bob: &mut BobModel, alice: &mut AliceModel) {
        let message = &bob.eingabe_klartext;
        alice.eingabe_klartext = message.clone();
        bob.eingabe_klartext = String::new();
        // todo -- Logik zum Senden der Nachricht für Bob
    }
    fn clear_bob(&mut self, bob: &mut BobModel) {
        bob.eingabe_klartext = String::new();
        // todo -- Logik zum Zurücksetzen der Eingabefelder und Labels für Bob
    }
}
