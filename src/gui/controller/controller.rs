use bigdecimal::num_bigint::BigUint;
use crate::encryption::keygen_service::KeyGenService;
use crate::gui::controller::commands::*;
use crate::gui::model::model::{AppState, View};
use druid::{Env, Event, EventCtx};
use crate::encryption::encryption_services::{Decryption, Encryption, Signing, Verification};
use crate::encryption::rsa::keys::{PrivateKey, PublicKey};
use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

pub struct AppController {
    alice_private_key: PrivateKey,
    alice_public_key: PublicKey,

    bob_private_key: PrivateKey,
    bob_public_key: PublicKey,
}

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
            Event::Command(cmd) if cmd.is(VERIFY) => {
                self.alice_verify_message_from_bob(app_state);
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
            Event::Command(cmd) if cmd.is(VERIFY) => {
                self.bob_verify_message_from_alice(app_state);
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
        let (public_key_alice, private_key_alice) = self.calculate_keypair(app_state);

        self.alice_private_key = private_key_alice;
        self.alice_public_key = public_key_alice;

        app_state.haupt_menu.public_exponent_alice = self.alice_public_key.get_e();
        app_state.alice.private_key = self.alice_private_key.get_d();
    }

    ///
    /// Berechnet das Schlüsselpaar für Bob.
    ///
    fn calculate_keypair_bob(&mut self, app_state: &mut AppState) {
        let (public_key_alice, private_key_alice) = self.calculate_keypair(app_state);

        self.bob_private_key = private_key_alice;
        self.bob_public_key = public_key_alice;

        app_state.haupt_menu.public_key_bob = self.bob_public_key.get_e();
        app_state.bob.private_key = self.bob_private_key.get_d();
    }

    ///
    /// Berechnet ein Schlüsselpaar
    ///
    fn calculate_keypair(&mut self, app_state: &mut AppState) -> (PublicKey, PrivateKey) {
        let modul_width = app_state.haupt_menu.modul_width.parse::<usize>().unwrap();
        let keygen_service = RsaKeygenService::new(modul_width);
        keygen_service.generate_keypair()
    }

    ///
    /// Verschlüsselt die Nachricht von Alice mit Bobs öffentlichem Schlüssel.
    ///
    fn encrypt_alice(&mut self, app_state: &mut AppState) {
        let message = app_state.alice.message_klartext.clone();
        let encrypted = self.bob_public_key.encrypt(&message);
        app_state.alice.message_chiffre = encrypted;
    }

    ///
    /// Signiert die Nachricht von Alice mit ihrem privaten Schlüssel.
    ///
    fn sign_alice(&mut self, _app_state: &mut AppState) {
        let message = _app_state.alice.message_klartext.clone();
        let signed = self.alice_private_key.sign(&message);
        _app_state.alice.signature = signed;
    }

    ///
    /// Verifiziert die Nachricht von Bob mit seinem öffentlichen Schlüssel.
    ///
    fn alice_verify_message_from_bob(&mut self, _app_state: &mut AppState) {
        let message = _app_state.alice.message_klartext.clone();
        let signature = _app_state.alice.signature.clone();
        let verified = self.bob_public_key.verify(&signature, &message);
        _app_state.alice.signature_status = verified;
    }

    ///
    /// Entschlüsselt die Nachricht von Bob mit Alices privatem Schlüssel.
    ///
    fn decrypt_alice(&mut self, app_state: &mut AppState) {
        let cipher_text = app_state.alice.message_chiffre.clone();
        let decrypted = self.alice_private_key.decrypt(&cipher_text);
        app_state.alice.message_klartext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Alice an Bob und löscht das Nachrichten-Feld.
    ///
    fn send_message_alice(&mut self, app_state: &mut AppState) {
        let cipher_text = &app_state.alice.message_chiffre;
        app_state.bob.message_chiffre = cipher_text.clone();
        let signature = &app_state.alice.signature;
        app_state.bob.signature = signature.clone();
        self.clear_alice(app_state);
    }

    ///
    /// Löscht die Inhalte von Alice.
    ///
    fn clear_alice(&mut self, app_state: &mut AppState) {
        app_state.alice.message_klartext = String::new();
        app_state.alice.message_chiffre = String::new();
        app_state.alice.signature = String::new();
    }

    ///
    /// Verschlüsselt die Nachricht von Bob mit Alice öffentlichem Schlüssel.
    ///
    fn encrypt_bob(&mut self, app_state: &mut AppState) {
        let message = app_state.bob.message_klartext.clone();
        let encrypted = self.alice_public_key.encrypt(&message);
        app_state.bob.message_chiffre = encrypted;
    }

    ///
    /// Signiert die Nachricht von Bob mit seinem privaten Schlüssel.
    ///
    fn sign_bob(&mut self, _app_state: &mut AppState) {
        let message = _app_state.bob.message_klartext.clone();
        let signed = self.bob_private_key.sign(&message);
        _app_state.bob.signature = signed;
    }

    ///
    /// Verifiziert die Nachricht von Alice mit ihrem öffentlichen Schlüssel.
    ///
    fn bob_verify_message_from_alice(&mut self, _app_state: &mut AppState) {
        let message = _app_state.bob.message_klartext.clone();
        let signature = _app_state.bob.signature.clone();
        let verified = self.alice_public_key.verify(&signature, &message);
        _app_state.bob.signature_status = verified;
    }

    ///
    /// Entschlüsselt die Nachricht von Alice mit Bobs privatem Schlüssel.
    ///
    fn decrypt_bob(&mut self, app_state: &mut AppState) {
        let cipher_text = app_state.bob.message_chiffre.clone();
        let decrypted = self.bob_private_key.decrypt(&cipher_text);
        app_state.bob.message_klartext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Bob an Alice und löscht das Nachrichten-Feld.
    ///
    fn send_message_bob(&mut self, app_state: &mut AppState) {
        let cipher_text = &app_state.bob.message_chiffre;
        app_state.alice.message_chiffre = cipher_text.clone();
        let signature = &app_state.bob.signature;
        app_state.alice.signature = signature.clone();
        self.clear_bob(app_state);
    }

    ///
    /// Löscht die Nachricht von Bob.
    ///
    fn clear_bob(&mut self, app_state: &mut AppState) {
        app_state.bob.message_klartext = String::new();
        app_state.bob.message_chiffre = String::new();
        app_state.bob.signature = String::new();
    }
}
