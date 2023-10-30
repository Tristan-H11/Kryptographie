use crate::big_i;
use crate::encryption::rsa::keys::{PrivateKey, PublicKey};
use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;
use crate::gui::controller::commands::*;
use crate::gui::model::model::{AppState, View};
use bigdecimal::num_bigint::BigInt;
use druid::{Env, Event, EventCtx};
use log::{debug, error, info};

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
        if data.current_view == View::MainMenu {
            self.handle_haupt_menu_event(event, data, env);
        } else if data.current_view == View::Alice {
            self.handle_alice_event(event, data, env);
        } else if data.current_view == View::Bob {
            self.handle_bob_event(event, data, env);
        }

        child.event(ctx, event, data, env);
    }
}

impl Default for AppController {
    ///
    /// Erstellt eine neue Instanz des AppControllers mit 1er Werten für die Keys.
    ///
    fn default() -> Self {
        debug!("Erstelle Default-AppController");
        AppController {
            alice_private_key: PrivateKey::new(big_i!(1), big_i!(1)),
            alice_public_key: PublicKey::new(big_i!(1), big_i!(1)),

            bob_private_key: PrivateKey::new(big_i!(1), big_i!(1)),
            bob_public_key: PublicKey::new(big_i!(1), big_i!(1)),
        }
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
            Event::Command(cmd) if cmd.is(SWITCH_TO_MAIN_MENU) => {
                app_state.current_view = View::MainMenu;
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
            Event::Command(cmd) if cmd.is(SWITCH_TO_MAIN_MENU) => {
                app_state.current_view = View::MainMenu;
            }
            _ => (),
        }
    }

    ///
    /// Berechnet das Schlüsselpaar für Alice.
    ///
    fn calculate_keypair_alice(&mut self, app_state: &mut AppState) {
        info!("Berechne Schlüsselpaar für Alice");
        let (public_key_alice, private_key_alice) = self.calculate_keypair(app_state);

        self.alice_private_key = private_key_alice;
        self.alice_public_key = public_key_alice;

        app_state.main_menu.public_exponent_alice = self.alice_public_key.get_e_as_str();
        app_state.main_menu.modul_alice = self.alice_public_key.get_n_as_str();
        app_state.alice.private_exponent = self.alice_private_key.get_d_as_str();
    }

    ///
    /// Berechnet das Schlüsselpaar für Bob.
    ///
    fn calculate_keypair_bob(&mut self, app_state: &mut AppState) {
        info!("Berechne Schlüsselpaar für Bob");
        let (public_key_bob, private_key_bob) = self.calculate_keypair(app_state);

        self.bob_private_key = private_key_bob;
        self.bob_public_key = public_key_bob;

        app_state.main_menu.public_exponent_bob = self.bob_public_key.get_e_as_str();
        app_state.main_menu.modul_bob = self.bob_public_key.get_n_as_str();
        app_state.bob.private_exponent = self.bob_private_key.get_d_as_str();
    }

    ///
    /// Berechnet ein Schlüsselpaar
    ///
    fn calculate_keypair(&mut self, app_state: &mut AppState) -> (PublicKey, PrivateKey) {
        let modul_width = match app_state.main_menu.modul_width.parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                error!("Fehler beim Parsen der Modul-Breite. Es wird ein Default-Schlüssel mit 4096-bit erstellt.");
                4096
            }
        };
        let keygen_service = RsaKeygenService::new(modul_width);
        let miller_rabin_iterations = match app_state
            .main_menu
            .miller_rabin_iterations
            .parse::<usize>()
        {
            Ok(x) => x,
            Err(_) => {
                error!("Fehler beim Parsen der Miller-Rabin-Iterationen. Es wird ein Default-Wert von 100 Iterationen verwendet.");
                100
            }
        };

        keygen_service.generate_keypair(miller_rabin_iterations)
    }

    ///
    /// Parst die Basis-Länge aus dem GUI-String.
    /// Falls der String nicht geparst werden kann, wird der Default-Wert 55296 verwendet.
    ///
    fn parse_base(app_state: &mut AppState) -> u32 {
        let g_base = match app_state.main_menu.basis_length.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                error!("Fehler beim Parsen der Basis-Länge. Es wird der Default 55296 verwendet.");
                55296
            }
        };
        g_base
    }

    ///
    /// Verschlüsselt die Nachricht von Alice mit Bobs öffentlichem Schlüssel.
    ///
    fn encrypt_alice(&mut self, app_state: &mut AppState) {
        info!("Verschlüssle Nachricht von Alice");
        let plaintext = app_state.alice.plaintext.clone();
        let encrypted = self
            .bob_public_key
            .encrypt(&plaintext, Self::parse_base(app_state));
        app_state.alice.ciphertext = encrypted;
    }

    ///
    /// Signiert die Nachricht von Alice mit ihrem privaten Schlüssel.
    ///
    fn sign_alice(&mut self, _app_state: &mut AppState) {
        info!("Signiere Nachricht von Alice");
        let message = _app_state.alice.plaintext.clone();
        let signed = self.alice_private_key.sign(&message);
        _app_state.alice.signature = signed;
    }

    ///
    /// Verifiziert die Nachricht von Bob mit seinem öffentlichen Schlüssel.
    ///
    fn alice_verify_message_from_bob(&mut self, _app_state: &mut AppState) {
        info!("Verifiziere Nachricht von Bob");
        let message = _app_state.alice.plaintext.clone();
        let signature = _app_state.alice.signature.clone();
        let verified = self.bob_public_key.verify(&signature, &message);
        _app_state.alice.signature_status = verified;
    }

    ///
    /// Entschlüsselt die Nachricht von Bob mit Alices privatem Schlüssel.
    ///
    fn decrypt_alice(&mut self, app_state: &mut AppState) {
        info!("Entschlüssle Nachricht von Bob");
        let cipher_text = app_state.alice.ciphertext.clone();
        let decrypted = self
            .alice_private_key
            .decrypt(&cipher_text, Self::parse_base(app_state));
        app_state.alice.plaintext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Alice an Bob und löscht das Nachrichten-Feld.
    ///
    fn send_message_alice(&mut self, app_state: &mut AppState) {
        info!("Sende Nachricht von Alice an Bob");
        let cipher_text = &app_state.alice.ciphertext;
        app_state.bob.ciphertext = cipher_text.clone();
        let signature = &app_state.alice.signature;
        app_state.bob.signature = signature.clone();
        self.clear_alice(app_state);
    }

    ///
    /// Löscht die Inhalte von Alice.
    ///
    fn clear_alice(&mut self, app_state: &mut AppState) {
        info!("Lösche Felder von Alice");
        app_state.alice.plaintext = String::new();
        app_state.alice.ciphertext = String::new();
        app_state.alice.signature = String::new();
    }

    ///
    /// Verschlüsselt die Nachricht von Bob mit Alice öffentlichem Schlüssel.
    ///
    fn encrypt_bob(&mut self, app_state: &mut AppState) {
        info!("Verschlüssle Nachricht von Bob");
        let plaintext = app_state.bob.plaintext.clone();
        let encrypted = self
            .alice_public_key
            .encrypt(&plaintext, Self::parse_base(app_state));
        app_state.bob.ciphertext = encrypted;
    }

    ///
    /// Signiert die Nachricht von Bob mit seinem privaten Schlüssel.
    ///
    fn sign_bob(&mut self, _app_state: &mut AppState) {
        info!("Signiere Nachricht von Bob");
        let message = _app_state.bob.plaintext.clone();
        let signed = self.bob_private_key.sign(&message);
        _app_state.bob.signature = signed;
    }

    ///
    /// Verifiziert die Nachricht von Alice mit ihrem öffentlichen Schlüssel.
    ///
    fn bob_verify_message_from_alice(&mut self, _app_state: &mut AppState) {
        info!("Verifiziere Nachricht von Alice");
        let message = _app_state.bob.plaintext.clone();
        let signature = _app_state.bob.signature.clone();
        let verified = self.alice_public_key.verify(&signature, &message);
        _app_state.bob.signature_status = verified;
    }

    ///
    /// Entschlüsselt die Nachricht von Alice mit Bobs privatem Schlüssel.
    ///
    fn decrypt_bob(&mut self, app_state: &mut AppState) {
        info!("Entschlüssle Nachricht von Alice");
        let cipher_text = app_state.bob.ciphertext.clone();
        let decrypted = self
            .bob_private_key
            .decrypt(&cipher_text, Self::parse_base(app_state));
        app_state.bob.plaintext = decrypted;
    }

    ///
    /// Sendet die Nachricht von Bob an Alice und löscht das Nachrichten-Feld.
    ///
    fn send_message_bob(&mut self, app_state: &mut AppState) {
        info!("Sende Nachricht von Bob an Alice");
        let cipher_text = &app_state.bob.ciphertext;
        app_state.alice.ciphertext = cipher_text.clone();
        let signature = &app_state.bob.signature;
        app_state.alice.signature = signature.clone();
        self.clear_bob(app_state);
    }

    ///
    /// Löscht die Nachricht von Bob.
    ///
    fn clear_bob(&mut self, app_state: &mut AppState) {
        info!("Lösche Felder von Bob");
        app_state.bob.plaintext = String::new();
        app_state.bob.ciphertext = String::new();
        app_state.bob.signature = String::new();
    }
}
