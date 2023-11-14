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
            alice_private_key: PrivateKey::new(big_i!(351691), big_i!(8108339), 55296),
            alice_public_key: PublicKey::new(big_i!(1751731), big_i!(8108339), 55296),

            bob_private_key: PrivateKey::new(big_i!(351691), big_i!(8108339), 55296),
            bob_public_key: PublicKey::new(big_i!(1751731), big_i!(8108339), 55296),
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
                self.a_encrypt_a_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.a_decrypt_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                    self.a_sign_a_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(VERIFY) => {
                    self.a_verify_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                    self.a_send_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(CLEAR) => {
                self.a_clear_a_view(app_state);
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
                self.b_encrypt_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(DECRYPT) => {
                self.b_decode_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(SIGN) => {
                self.b_sign_b_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(VERIFY) => {
                self.b_verify_a_msg(app_state);
            }
            Event::Command(cmd) if cmd.is(SEND_MESSAGE) => {
                self.b_send_a_msg(app_state);
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
        let modul_width = match app_state.main_menu.modul_width.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                error!(
                    "Fehler beim Parsen der Modul-Breite. Es wird ein Default-Schlüssel \
                        mit 4096-bit erstellt."
                );
                app_state.main_menu.modul_width = String::from("4096");
                4096
            }
        };
        let keygen_service = RsaKeygenService::new(modul_width);
        let miller_rabin_iterations =
            match app_state.main_menu.miller_rabin_iterations.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    error!(
                    "Fehler beim Parsen der Miller-Rabin-Iterationen. Es wird ein Default-Wert \
                        von 100 Iterationen verwendet."
                );
                    app_state.main_menu.miller_rabin_iterations = String::from("100");
                    100
                }
            };

        let random_seed = match app_state.main_menu.random_seed.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                error!("Fehler beim Parsen des Random-Seeds. Es wird ein Default-Wert von 13 verwendet.");
                app_state.main_menu.random_seed = String::from("13");
                13
            }
        };

        let base = Self::parse_base(app_state);
        keygen_service.generate_keypair(miller_rabin_iterations, random_seed, base)
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
                app_state.main_menu.basis_length = String::from("55296");
                55296
            }
        };
        g_base
    }

    ///
    /// Verschlüsselt die Nachricht von Alice mit Bobs öffentlichem Schlüssel.
    ///
    fn a_encrypt_a_msg(&mut self, app_state: &mut AppState) {
        info!("Verschlüssle Nachricht von Alice");
        let plaintext = app_state.alice.plaintext_msg.clone();
        let encrypted = self
            .bob_public_key
            .encrypt(&plaintext, Self::parse_base(app_state));
        app_state.alice.ciphre_msg = encrypted;
    }

    ///
    /// Entschlüsselt die Nachricht von Bob mit Alices privatem Schlüssel.
    ///
    fn a_decrypt_b_msg(&mut self, app_state: &mut AppState) {
        info!("Entschlüssle Nachricht von Bob");
        let cipher_text = app_state.alice.ciphre_msg.clone();
        let decrypted = self
            .alice_private_key
            .decrypt(&cipher_text, Self::parse_base(app_state));
        app_state.alice.plaintext_msg = decrypted;
    }

    ///
    /// Signiert die Nachricht von Alice mit ihrem privaten Schlüssel.
    ///

    fn a_sign_a_msg(&mut self, _app_state: &mut AppState) {
        info!("Signiere Nachricht von Alice");
        if !_app_state.alice.ciphre_msg.is_empty() {
            let msg = _app_state.alice.ciphre_msg.clone();
            let signed = self.alice_private_key.sign(&msg);
            _app_state.alice.signature_msg = signed;
        } else {
            let msg = _app_state.alice.plaintext_msg.clone();
            let signed = self.alice_private_key.sign(&msg);
            _app_state.alice.signature_msg = signed;
        }
    }

    ///
    /// Sendet die Nachricht von Alice an Bob
    ///
    fn a_send_b_msg(&mut self, _app_state: &mut AppState) {
        info!("Sende Nachricht von Alice an Bob");
        if !_app_state.alice.ciphre_msg.is_empty() {
            let msg_to_send = &_app_state.alice.ciphre_msg;
            _app_state.bob.ciphre_msg = msg_to_send.clone();
            let signature = &_app_state.alice.signature_msg;
            _app_state.bob.signature_msg = signature.clone();
        } else {
            let msg_to_send = &_app_state.alice.plaintext_msg;
            _app_state.bob.plaintext_msg = msg_to_send.clone();
            let signature = &_app_state.alice.signature_msg;
            _app_state.bob.signature_msg = signature.clone();
        }
    }

    ///
    /// Verifiziert die Nachricht von Bob mit seinem öffentlichen Schlüssel.
    ///
    fn a_verify_b_msg(&mut self, _app_state: &mut AppState) {
        info!("Verifiziere Nachricht von Bob");
        if !_app_state.alice.ciphre_msg.is_empty() {
            let msg = _app_state.alice.ciphre_msg.clone();
            let signature = _app_state.alice.signature_msg.clone();
            let verified = self.bob_public_key.verify(&signature, &msg);
            _app_state.alice.signature_status = verified;
        } else {
            let msg = _app_state.alice.plaintext_msg.clone();
            let signature = _app_state.alice.signature_msg.clone();
            let verified = self.bob_public_key.verify(&signature, &msg);
            _app_state.alice.signature_status = verified;
        }
    }


    ///
    /// Löscht die Inhalte von Alice.
    ///
    fn a_clear_a_view(&mut self, app_state: &mut AppState) {
        info!("Lösche Felder von Alice");
        app_state.alice.plaintext_msg = String::new();
        app_state.alice.ciphre_msg = String::new();
        app_state.alice.signature_msg = String::new();
    }

    ///
    /// Verschlüsselt die Nachricht von Bob mit Alice öffentlichem Schlüssel.
    ///
    fn b_encrypt_b_msg(&mut self, app_state: &mut AppState) {
        info!("Verschlüssle Nachricht von Bob");
        let plaintext = app_state.bob.plaintext_msg.clone();
        let encrypted = self
            .alice_public_key
            .encrypt(&plaintext, Self::parse_base(app_state));
        app_state.bob.ciphre_msg = encrypted;
    }

    ///
    /// Entschlüsselt die Nachricht von Alice mit Bobs privatem Schlüssel.
    ///
    fn b_decode_b_msg(&mut self, app_state: &mut AppState) {
        info!("Entschlüssle Nachricht von Alice");
        let cipher_text = app_state.bob.ciphre_msg.clone();
        let decrypted = self
            .bob_private_key
            .decrypt(&cipher_text, Self::parse_base(app_state));
        app_state.bob.plaintext_msg = decrypted;
    }

    fn b_sign_b_msg(&mut self, _app_state: &mut AppState) {
        info!("Signiere Nachricht von Bob");
        let msg = if !_app_state.bob.ciphre_msg.is_empty() {
            _app_state.bob.ciphre_msg.clone()
        } else {
            _app_state.bob.plaintext_msg.clone()
        };
        let signed = self.bob_private_key.sign(&msg);
        _app_state.bob.signature_msg = signed;
    }

    fn b_verify_a_msg(&mut self, _app_state: &mut AppState) {
        info!("Verifiziere Nachricht von Alice");
        let (message, signature) = if !_app_state.bob.ciphre_msg.is_empty() {
            (_app_state.bob.ciphre_msg.clone(), _app_state.bob.signature_msg.clone())
        } else {
            (_app_state.bob.plaintext_msg.clone(), _app_state.bob.signature_msg.clone())
        };
        let verified = self.alice_public_key.verify(&signature, &message);
        _app_state.bob.signature_status = verified;
    }

    fn b_send_a_msg(&mut self, _app_state: &mut AppState) {
        info!("Sende Nachricht von Bob an Alice");
        if !_app_state.bob.ciphre_msg.is_empty() {
            let cipher_text = &_app_state.bob.ciphre_msg;
            _app_state.alice.ciphre_msg = cipher_text.clone();
        } else {
            let plain_msg = &_app_state.bob.plaintext_msg;
            _app_state.alice.plaintext_msg = plain_msg.clone();
        }
        let signature = &_app_state.bob.signature_msg;
        _app_state.alice.signature_msg = signature.clone();
    }

    ///
    /// Löscht die Nachricht von Bob.
    ///
    fn clear_bob(&mut self, app_state: &mut AppState) {
        info!("Lösche Felder von Bob");
        app_state.bob.plaintext_msg = String::new();
        app_state.bob.ciphre_msg = String::new();
        app_state.bob.signature_msg = String::new();
    }
}
