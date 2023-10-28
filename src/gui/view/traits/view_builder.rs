
use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, Widget, WidgetExt,
};
use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN,
    SWITCH_TO_ALICE, SWITCH_TO_BOB, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::{AliceModel, BobModel, MainMenuModel};
use crate::gui::view::traits::key_text_wrapper::{KeyTextWrapper, TextWrapper};

// trait für gemeinsame Merkmale von Ansichten
pub trait ViewBuilder<Model> {
    fn build_view() -> Box<dyn Widget<Model>>;
}

// trait für Hauptmenü
pub struct MainMenuViewBuilder;

impl ViewBuilder<MainMenuModel> for MainMenuViewBuilder {
    fn build_view() -> Box<dyn Widget<MainMenuModel>> {
        let fixed_width_entry_label = 200.0;
        let fixed_width_textbox = 400.0;
        let fixed_width_button = fixed_width_entry_label + fixed_width_textbox;
        let spacer_size = 40.0;
        let _spacer_empty_strings = "          ";

        // Entry-Felder
        let modul_width_entry = Flex::row()
            .with_child(Label::new("Breite des Moduls n: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("z.B. 4096")
                    .fix_width(fixed_width_textbox)
                    .lens(MainMenuModel::modul_width),
            );

        let miller_rabin_entry = Flex::row()
            .with_child(Label::new("Miller-Rabin Iterationen: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("z.B. 100")
                    .fix_width(fixed_width_textbox)
                    .lens(MainMenuModel::miller_rabin_iterations),
            );

        let basis_entry = Flex::row()
            .with_child(Label::new("Basis für Kalkulationen: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("default = 55296")
                    .fix_width(fixed_width_textbox)
                    .lens(MainMenuModel::basis_length),
            );

        // Button
        let calculate_keypair_alice = Button::new("Schlüsselpaar <public, private> Alice berechnen")
            .on_click(|ctx, _data: &mut MainMenuModel, _env| {
                ctx.submit_command(CALCULATE_KEYPAIR_ALICE);
            })
            .fix_width(fixed_width_button);

        let calculate_keypair_bob = Button::new("Schlüsselpaar <public, private> Bob berechnen")
            .on_click(|ctx, _data: &mut MainMenuModel, _env| {
                ctx.submit_command(CALCULATE_KEYPAIR_BOB);
            })
            .fix_width(fixed_width_button);

        let open_alice_button = Button::new("Alice-Ansicht öffnen")
            .on_click(|_ctx, _data, _env| {
                _ctx.submit_command(SWITCH_TO_ALICE);
            })
            .fix_width(fixed_width_button);

        let open_bob_button = Button::new("Bob-Ansicht öffnen")
            .on_click(|_ctx, _data, _env| {
                _ctx.submit_command(SWITCH_TO_BOB);
            })
            .fix_width(fixed_width_button);

        // big-text
        let public_exponent_alice_label = Label::new(|data: &MainMenuModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let key_text_wrapper = wrapper.key_text_wrapper(
                &format!("Öffentlicher Exponent Alice:{}",
                         &data.public_exponent_alice), 150);
            key_text_wrapper
        })
            .expand_width();


        let public_exponent_bob_label = Label::new(|data: &MainMenuModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let wrapped_text = wrapper.key_text_wrapper(&format!("{}", &data.public_exponent_bob), 150);
            format!("Öffentlicher Exponent Bob: \n{}", wrapped_text)
        })
            .expand_width();

        Box::new(
            Flex::column()
            .with_default_spacer()
            .with_default_spacer()
            .with_child(modul_width_entry)
            .with_default_spacer()
            .with_child(miller_rabin_entry)
            .with_default_spacer()
            .with_child(basis_entry)
            .with_spacer(spacer_size)
            .with_child(calculate_keypair_alice)
            .with_default_spacer()
            .with_child(calculate_keypair_bob)
            .with_spacer(spacer_size)
            .with_child(public_exponent_alice_label)
            .with_default_spacer()
            .with_child(public_exponent_bob_label)
            .with_spacer(spacer_size)
            .with_child(open_alice_button)
            .with_default_spacer()
            .with_child(open_bob_button))
    }
}

// trait für Alice View
pub struct AliceViewBuilder;

impl ViewBuilder<AliceModel> for AliceViewBuilder {
    fn build_view() -> Box<dyn Widget<AliceModel>> {
        let fixed_width_entry_label = 200.0;
        let fixed_width_textbox = 400.0;
        let fixed_width_button = fixed_width_entry_label + fixed_width_textbox;
        let spacer_size = 40.0;

        // Label
        let secret_exponent_label = Label::new(|data: &AliceModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let wrapped_text = wrapper.key_text_wrapper(&format!("Geheimer Exponent: {}", data.private_exponent), 75);
            format!("Geheimer Exponent: \n{}", wrapped_text)
        })
            .expand_width();


        // Entry-Felder und Labels
        let plaintext_entry = Flex::row()
            .with_child(Label::new("Klartext: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("z.B. Hallo wie geht es dir?")
                    .fix_width(fixed_width_textbox)
                    .lens(AliceModel::plaintext),
            );

        let ciphertext_entry = Flex::row()
            .with_child(Label::new("Geheimtext: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("Erzeugt durch Berechnung.")
                    .fix_width(fixed_width_textbox)
                    .lens(AliceModel::ciphertext),
            );

        let signature_row = Flex::row()
            .with_child(
                Flex::column().with_child(
                    TextBox::new()
                        .with_placeholder("Signatur")
                        .fix_width(fixed_width_textbox)
                        .lens(AliceModel::signature),
                ),
            )
            .with_default_spacer()
            .with_child(
                Label::new(|data: &AliceModel, _env: &Env| {
                    if data.signature_status {
                        "Gültig".to_string()
                    } else {
                        "Ungültig".to_string()
                    }
                })
                    .fix_width(fixed_width_entry_label),
            );

        // Buttons
        let encrypt_button = Button::new("Mit Bobs PublicKey verschlüsseln")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(ENCRYPT);
            })
            .fix_width(fixed_width_button);

        let calc_sign_button = Button::new("Signatur berechnen")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(SIGN);
            })
            .fix_width(fixed_width_button);

        let check_sign_button = Button::new("Signatur prüfen")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(VERIFY);
            })
            .fix_width(fixed_width_button);

        let decrypt_button = Button::new("Mit eigenem PrivateKey entschlüsseln")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(DECRYPT);
            })
            .fix_width(fixed_width_button);

        let send_message_button = Button::new("Nachricht senden")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(SEND_MESSAGE);
            })
            .fix_width(fixed_width_button);

        let clear_button = Button::new("Alles außer privaten Schlüssel löschen")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(CLEAR);
            })
            .fix_width(fixed_width_button);

        let back_button = Button::new("Zurück zum Hauptmenü")
            .on_click(|_ctx, _data: &mut AliceModel, _env| {
                _ctx.submit_command(SWITCH_TO_MAIN_MENU);
            })
            .fix_width(fixed_width_button);

        Box::new(
            Flex::column()
            .with_default_spacer()
            .with_default_spacer()
            .with_child(secret_exponent_label)
            .with_spacer(spacer_size)
            .with_child(plaintext_entry)
            .with_default_spacer()
            .with_child(ciphertext_entry)
            .with_spacer(spacer_size)
            .with_child(encrypt_button)
            .with_default_spacer()
            .with_child(decrypt_button)
            .with_spacer(spacer_size)
            .with_child(calc_sign_button)
            .with_default_spacer()
            .with_child(check_sign_button)
            .with_default_spacer()
            .with_child(signature_row)
            .with_spacer(spacer_size)
            .with_child(send_message_button)
            .with_spacer(spacer_size)
            .with_child(clear_button)
            .with_spacer(spacer_size)
            .with_child(back_button)
            .padding(druid::Insets::uniform(10.0)))
    }
}

// trait für Bob View
pub struct BobViewBuilder;

impl ViewBuilder<BobModel> for BobViewBuilder {
    fn build_view() -> Box<dyn Widget<BobModel>> {
        let fixed_width_entry_label = 200.0;
        let fixed_width_textbox = 400.0;
        let fixed_width_button = fixed_width_entry_label + fixed_width_textbox;
        let spacer_size = 40.0;

        // Label
        let secret_exponent_label = Label::new(|data: &BobModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let wrapped_text = wrapper.key_text_wrapper(&format!("Geheimer Exponent: {}", data.private_exponent), 75);
            format!("Geheimer Exponent: \n{}", wrapped_text)
        })
            .expand_width();


        // Entry-Felder und Labels
        let plaintext_entry = Flex::row()
            .with_child(Label::new("Klartext: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("z.B. Hallo wie geht es dir?")
                    .fix_width(fixed_width_textbox)
                    .lens(BobModel::plaintext),
            );

        let ciphertext_entry = Flex::row()
            .with_child(Label::new("Geheimtext: ").fix_width(fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder("Erzeugt durch Berechnung.")
                    .fix_width(fixed_width_textbox)
                    .lens(BobModel::ciphertext),
            );

        let signature_row = Flex::row()
            .with_child(
                Flex::column().with_child(
                    TextBox::new()
                        .with_placeholder("Signatur")
                        .fix_width(fixed_width_textbox)
                        .lens(BobModel::signature),
                ),
            )
            .with_default_spacer()
            .with_child(
                Label::new(|data: &BobModel, _env: &Env| {
                    if data.signature_status {
                        "Gültig".to_string()
                    } else {
                        "Ungültig".to_string()
                    }
                })
                    .fix_width(fixed_width_entry_label),
            );

        // Buttons
        let encrypt_button = Button::new("Mit Alice PublicKey verschlüsseln")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(ENCRYPT);
            })
            .fix_width(fixed_width_button);

        let calc_sign_button = Button::new("Signatur berechnen")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(SIGN);
            })
            .fix_width(fixed_width_button);

        let check_sign_button = Button::new("Signatur prüfen")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(VERIFY);
            })
            .fix_width(fixed_width_button);

        let decrypt_button = Button::new("Mit eigenem PrivateKey entschlüsseln")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(DECRYPT);
            })
            .fix_width(fixed_width_button);

        let send_message_button = Button::new("Nachricht senden")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(SEND_MESSAGE);
            })
            .fix_width(fixed_width_button);

        let clear_button = Button::new("Alles außer privaten Schlüssel löschen")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(CLEAR);
            })
            .fix_width(fixed_width_button);

        let back_button = Button::new("Zurück zum Hauptmenü")
            .on_click(|_ctx, _data: &mut BobModel, _env| {
                _ctx.submit_command(SWITCH_TO_MAIN_MENU);
            })
            .fix_width(fixed_width_button);

        Box::new(
            Flex::column()
            .with_default_spacer()
            .with_default_spacer()
            .with_child(secret_exponent_label)
            .with_spacer(spacer_size)
            .with_child(plaintext_entry)
            .with_default_spacer()
            .with_child(ciphertext_entry)
            .with_spacer(spacer_size)
            .with_child(encrypt_button)
            .with_default_spacer()
            .with_child(decrypt_button)
            .with_spacer(spacer_size)
            .with_child(calc_sign_button)
            .with_default_spacer()
            .with_child(check_sign_button)
            .with_default_spacer()
            .with_child(signature_row)
            .with_spacer(spacer_size)
            .with_child(send_message_button)
            .with_spacer(spacer_size)
            .with_child(clear_button)
            .with_spacer(spacer_size)
            .with_child(back_button)
            .padding(druid::Insets::uniform(10.0)))
    }
}


