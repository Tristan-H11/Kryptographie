use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN,
    SWITCH_TO_ALICE, SWITCH_TO_BOB, SWITCH_TO_HAUPTMENU,
};
use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel};
use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, Widget, WidgetExt,
};

pub(crate) fn build_haupt_menu() -> impl Widget<HauptMenuModel> {
    // Entry-Felder
    let p1_entry = Flex::row()
        .with_child(Label::new("Breite von Primzahl 1: "))
        .with_child(TextBox::new().lens(HauptMenuModel::prime_number_one));

    let p2_entry = Flex::row()
        .with_child(Label::new("Breite von Primzahl 2: "))
        .with_child(TextBox::new().lens(HauptMenuModel::prime_number_two));

    let miller_rabin_entry = Flex::row()
        .with_child(Label::new("Miller-Rabin Iterationen: "))
        .with_child(TextBox::new().lens(HauptMenuModel::miller_rabin_iterations));

    // Button
    let calculate_keypair_alice = Button::new("Schlüsselpaar Alice Berechnen").on_click(
        |ctx, _data: &mut HauptMenuModel, _env| {
            ctx.submit_command(CALCULATE_KEYPAIR_ALICE);
        },
    );

    let calculate_keypair_bob = Button::new("Schlüsselpaar Bob Berechnen").on_click(
        |ctx, _data: &mut HauptMenuModel, _env| {
            ctx.submit_command(CALCULATE_KEYPAIR_BOB);
        },
    );

    let open_alice_button = Button::new("Alice-Ansicht öffnen").on_click(|_ctx, _data, _env| {
        _ctx.submit_command(SWITCH_TO_ALICE);
    });
    let open_bob_button = Button::new("Bob-Ansicht öffnen").on_click(|_ctx, _data, _env| {
        _ctx.submit_command(SWITCH_TO_BOB);
    });

    // Label
    let public_key_alice_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Alice: {}", &data.public_key_alice)
    });

    let public_key_bob_label = Label::new(|data: &HauptMenuModel, _env: &Env| -> String {
        format!("Öffentlicher Schlüssel Bob: {}", &data.public_key_bob)
    });

    Flex::column()
        .with_child(p1_entry)
        .with_default_spacer()
        .with_child(p2_entry)
        .with_default_spacer()
        .with_child(miller_rabin_entry)
        .with_default_spacer()
        .with_child(calculate_keypair_alice)
        .with_default_spacer()
        .with_child(calculate_keypair_bob)
        .with_default_spacer()
        .with_child(public_key_alice_label)
        .with_default_spacer()
        .with_child(public_key_bob_label)
        .with_default_spacer()
        .with_child(open_alice_button)
        .with_default_spacer()
        .with_child(open_bob_button)
        .padding(druid::Insets::uniform(10.0))
}

pub(crate) fn build_alice_view() -> impl Widget<AliceModel> {
    // Label
    let secret_key_label = Label::new(|data: &AliceModel, _env: &Env| {
        format!("Geheimschlüssel: {}", data.private_key)
    });

    // Entry-Felder und Labels
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Nachricht: "))
        .with_child(TextBox::new().lens(AliceModel::message));

    let signature_row = Flex::row()
        .with_child(
            Flex::column().with_child(
                TextBox::new()
                    .with_placeholder("Signatur")
                    .lens(AliceModel::signature),
            ),
        )
        .with_default_spacer()
        .with_child(Label::new(|data: &AliceModel, _env: &Env| {
            if data.signature_status {
                "Gültig".to_string()
            } else {
                "Ungültig".to_string()
            }
        }));

    // Buttons
    let encrypt_button = Button::new("Mit Bobs PublicKey verschlüsseln").on_click(
        |_ctx, _data: &mut AliceModel, _env| {
            _ctx.submit_command(ENCRYPT);
        },
    );
    let sign_button = Button::new("Signieren").on_click(|_ctx, _data: &mut AliceModel, _env| {
        _ctx.submit_command(SIGN);
    });
    let decrypt_button = Button::new("Mit eigenem PrivateKey entschlüsseln").on_click(
        |_ctx, _data: &mut AliceModel, _env| {
            _ctx.submit_command(DECRYPT);
        },
    );
    let send_message_button =
        Button::new("Nachricht senden").on_click(|_ctx, _data: &mut AliceModel, _env| {
            _ctx.submit_command(SEND_MESSAGE);
        });
    let clear_button =
        Button::new("Nachricht löschen").on_click(|_ctx, _data: &mut AliceModel, _env| {
            _ctx.submit_command(CLEAR);
        });
    let back_button =
        Button::new("Zurück zum Hauptmenü").on_click(|_ctx, _data: &mut AliceModel, _env| {
            _ctx.submit_command(SWITCH_TO_HAUPTMENU);
        });

    Flex::column()
        .with_child(secret_key_label)
        .with_default_spacer()
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(encrypt_button)
        .with_default_spacer()
        .with_child(sign_button)
        .with_default_spacer()
        .with_child(decrypt_button)
        .with_default_spacer()
        .with_child(send_message_button)
        .with_default_spacer()
        .with_child(signature_row)
        .with_default_spacer()
        .with_child(clear_button)
        .with_default_spacer()
        .with_child(back_button)
        .padding(druid::Insets::uniform(10.0))
}

pub(crate) fn build_bob_view() -> impl Widget<BobModel> {
    // Label
    let secret_key_label =
        Label::new(|data: &BobModel, _env: &Env| format!("Geheimschlüssel: {}", data.private_key));

    // Entry
    let plaintext_entry = Flex::row()
        .with_child(Label::new("Nachricht: "))
        .with_child(TextBox::new().lens(BobModel::message));

    let signature_row = Flex::row()
        .with_child(
            Flex::column().with_child(
                TextBox::new()
                    .with_placeholder("Signatur")
                    .lens(BobModel::signature),
            ),
        )
        .with_default_spacer()
        .with_child(Label::new(|data: &BobModel, _env: &Env| {
            if data.signature_status {
                "Gültig".to_string()
            } else {
                "Ungültig".to_string()
            }
        }));

    // Buttons
    let encrypt_button = Button::new("Mit Alice PublicKey verschlüsseln").on_click(
        |_ctx, _data: &mut BobModel, _env| {
            _ctx.submit_command(ENCRYPT);
        },
    );
    let sign_button = Button::new("Signieren").on_click(|_ctx, _data: &mut BobModel, _env| {
        _ctx.submit_command(SIGN);
    });
    let decrypt_button = Button::new("Mit eigenem PrivateKey entschlüsseln").on_click(
        |_ctx, _data: &mut BobModel, _env| {
            _ctx.submit_command(DECRYPT);
        },
    );
    let send_message_button =
        Button::new("Nachricht senden").on_click(|_ctx, _data: &mut BobModel, _env| {
            _ctx.submit_command(SEND_MESSAGE);
        });
    let clear_button =
        Button::new("Nachricht löschen").on_click(|_ctx, _data: &mut BobModel, _env| {
            _ctx.submit_command(CLEAR);
        });
    let back_button =
        Button::new("Zurück zum Hauptmenü").on_click(|_ctx, _data: &mut BobModel, _env| {
            _ctx.submit_command(SWITCH_TO_HAUPTMENU);
        });

    Flex::column()
        .with_child(secret_key_label)
        .with_default_spacer()
        .with_child(plaintext_entry)
        .with_default_spacer()
        .with_child(encrypt_button)
        .with_default_spacer()
        .with_child(sign_button)
        .with_default_spacer()
        .with_child(decrypt_button)
        .with_default_spacer()
        .with_child(send_message_button)
        .with_default_spacer()
        .with_child(signature_row)
        .with_default_spacer()
        .with_child(clear_button)
        .with_default_spacer()
        .with_child(back_button)
        .padding(druid::Insets::uniform(10.0))
}
