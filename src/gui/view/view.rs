use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN,
    SWITCH_TO_ALICE, SWITCH_TO_BOB, SWITCH_TO_HAUPTMENU, VERIFY,
};
use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel};
use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, Widget, WidgetExt,
};
pub(crate) fn build_haupt_menu() -> impl Widget<HauptMenuModel> {
    let fixed_width_entryLabel = 200.0;
    let fixed_width_textbox = 400.0;
    let fixed_width_button = fixed_width_entryLabel + fixed_width_textbox;
    let fixed_width_label = fixed_width_entryLabel + fixed_width_textbox;
    let spacer_size = 40.0;

    // Entry-Felder
    let p1_entry = Flex::row()
        .with_child(Label::new("Breite von Primzahl 1: ").fix_width(fixed_width_entryLabel))
        .with_child(
            TextBox::new()
                .with_placeholder("z.B. 5003")
                .fix_width(fixed_width_textbox)
                .lens(HauptMenuModel::prime_number_one),
        );

    let p2_entry = Flex::row()
        .with_child(Label::new("Breite von Primzahl 2: ").fix_width(fixed_width_entryLabel))
        .with_child(
            TextBox::new()
                .with_placeholder("z.B. 5009")
                .fix_width(fixed_width_textbox)
                .lens(HauptMenuModel::prime_number_two),
        );

    let miller_rabin_entry = Flex::row()
        .with_child(Label::new("Miller-Rabin Iterationen: ").fix_width(fixed_width_entryLabel))
        .with_child(
            TextBox::new()
                .with_placeholder("z.B. 61, sollte laut Herr Elsner mit max 100 reichen")
                .fix_width(fixed_width_textbox)
                .lens(HauptMenuModel::miller_rabin_iterations),
        );

    // Button
    let calculate_keypair_alice = Button::new("Schlüsselpaar <public, private> Alice berechnen")
        .on_click(|ctx, _data: &mut HauptMenuModel, _env| {
            ctx.submit_command(CALCULATE_KEYPAIR_ALICE);
        })
        .fix_width(fixed_width_button);

    let calculate_keypair_bob = Button::new("Schlüsselpaar <public, private> Bob berechnen")
        .on_click(|ctx, _data: &mut HauptMenuModel, _env| {
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

    // Label
    let public_key_alice_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Alice: {}", &data.public_key_alice)
    })
    .fix_width(fixed_width_label);

    let public_key_bob_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Bob: {}", &data.public_key_bob)
    })
    .fix_width(fixed_width_label);

    Flex::column()
        .with_default_spacer()
        .with_default_spacer()
        .with_child(p1_entry)
        .with_default_spacer()
        .with_child(p2_entry)
        .with_default_spacer()
        .with_child(miller_rabin_entry)
        .with_spacer(spacer_size)
        .with_child(calculate_keypair_alice)
        .with_default_spacer()
        .with_child(calculate_keypair_bob)
        .with_spacer(spacer_size)
        .with_child(public_key_alice_label)
        .with_default_spacer()
        .with_child(public_key_bob_label)
        .with_spacer(spacer_size)
        .with_child(open_alice_button)
        .with_default_spacer()
        .with_child(open_bob_button)
}

pub(crate) fn build_alice_view() -> impl Widget<AliceModel> {
    let fixed_width_entryLabel = 200.0;
    let fixed_width_textbox = 400.0;
    let fixed_width_button = fixed_width_entryLabel + fixed_width_textbox;
    let fixed_width_label = fixed_width_entryLabel + fixed_width_textbox;
    let spacer_size = 40.0;

    // Label
    let secret_key_label = Label::new(|data: &AliceModel, _env: &Env| {
        format!("Geheimschlüssel: {}", data.private_key)
    })
    .fix_width(fixed_width_label);

    // Entry-Felder und Labels
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Nachricht Klartext: ").fix_width(fixed_width_entryLabel))
        .with_child(TextBox::new()
            .with_placeholder("z.B. Hallo wie geht es dir, mir geht es heute wunderbar, wusstest du, dass heute XXXXX")
            .fix_width(fixed_width_textbox)
            .lens(AliceModel::message_klartext));

    let ciffretext_entry = Flex::row()
        .with_child(Label::new("Nachricht Chiffre: ").fix_width(fixed_width_entryLabel))
        .with_child(
            TextBox::new()
                .with_placeholder(
                    "Erzeugt durch Berechnung. Z.B.: BHFISFBNDNSVNBIEASFNDJSVNDSJIFOINFDCDSI",
                )
                .fix_width(fixed_width_textbox)
                .lens(AliceModel::message_chiffre),
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
            .fix_width(fixed_width_entryLabel),
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
            _ctx.submit_command(SWITCH_TO_HAUPTMENU);
        })
        .fix_width(fixed_width_button);

    Flex::column()
        .with_default_spacer()
        .with_default_spacer()
        .with_child(secret_key_label)
        .with_spacer(spacer_size)
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(ciffretext_entry)
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
        .padding(druid::Insets::uniform(10.0))
}

pub(crate) fn build_bob_view() -> impl Widget<BobModel> {
    let fixed_width_entryLabel = 200.0;
    let fixed_width_textbox = 400.0;
    let fixed_width_button = fixed_width_entryLabel + fixed_width_textbox;
    let fixed_width_label = fixed_width_entryLabel + fixed_width_textbox;
    let spacer_size = 40.0;

    // Label
    let secret_key_label =
        Label::new(|data: &BobModel, _env: &Env| format!("Geheimschlüssel: {}", data.private_key))
            .fix_width(fixed_width_label);

    // Entry-Felder und Labels
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Nachricht Klartext: ").fix_width(fixed_width_entryLabel))
        .with_child(TextBox::new()
            .with_placeholder("z.B. Hallo wie geht es dir, mir geht es heute wunderbar, wusstest du, dass heute XXXXX")
            .fix_width(fixed_width_textbox)
            .lens(BobModel::message_klartext));

    let ciffretext_entry = Flex::row()
        .with_child(Label::new("Nachricht Chiffre: ").fix_width(fixed_width_entryLabel))
        .with_child(
            TextBox::new()
                .with_placeholder(
                    "Erzeugt durch Berechnung. Z.B.: BHFISFBNDNSVNBIEASFNDJSVNDSJIFOINFDCDSI",
                )
                .fix_width(fixed_width_textbox)
                .lens(BobModel::message_chiffre),
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
            .fix_width(fixed_width_entryLabel),
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
            _ctx.submit_command(SWITCH_TO_HAUPTMENU);
        })
        .fix_width(fixed_width_button);

    Flex::column()
        .with_default_spacer()
        .with_default_spacer()
        .with_child(secret_key_label)
        .with_spacer(spacer_size)
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(ciffretext_entry)
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
        .padding(druid::Insets::uniform(10.0))
}
