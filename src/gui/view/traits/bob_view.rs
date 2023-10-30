use druid::{
    Widget, widget::Flex, WidgetExt,
};

use crate::gui::controller::commands::{
    CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::BobModel;
use crate::gui::view::traits::common_view_builder::{CommonViewComponents, ViewBuilder};

// Trait für Bob View
pub struct BobViewBuilder;

impl ViewBuilder<BobModel> for BobViewBuilder {
    fn build_view() -> Box<dyn Widget<BobModel>> {
        let common_components = CommonViewComponents::new();

        // Entry-Felder und Labels
        let plaintext_entry = common_components.create_text_entry(
            "Klartext: ",
            "z.B. Hallo wie geht es dir?",
            BobModel::plaintext,
        );

        let ciphertext_entry = common_components.create_text_entry(
            "Geheimtext: ",
            "Erzeugt durch Berechnung.",
            BobModel::ciphertext,
        );

        let secret_exponent_entry = common_components.create_text_entry(
            "Geheimer Exponent: ",
            "wird automatisch berechnet",
            BobModel::private_exponent,
        );

        let signature_row =
            common_components.create_text_entry("Signatur: ", "Signatur", BobModel::signature);

        // Buttons
        let encrypt_button =
            common_components.create_button("Mit Alice PublicKey verschlüsseln", ENCRYPT);
        let calc_sign_button = common_components.create_button("Signatur berechnen", SIGN);
        let check_sign_button = common_components.create_button("Signatur prüfen", VERIFY);
        let decrypt_button =
            common_components.create_button("Mit eigenem PrivateKey entschlüsseln", DECRYPT);
        let send_message_button = common_components.create_button("Nachricht senden", SEND_MESSAGE);
        let clear_button =
            common_components.create_button("Alles außer privaten Schlüssel löschen", CLEAR);
        let back_button =
            common_components.create_button("Zurück zum Hauptmenü", SWITCH_TO_MAIN_MENU);

        // UI Struktur
        Box::new(
            Flex::column()
                .with_default_spacer()
                .with_default_spacer()
                .with_child(plaintext_entry)
                .with_default_spacer()
                .with_child(ciphertext_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(encrypt_button)
                .with_default_spacer()
                .with_child(decrypt_button)
                .with_spacer(common_components.spacer_size)
                .with_child(secret_exponent_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(calc_sign_button)
                .with_default_spacer()
                .with_child(check_sign_button)
                .with_default_spacer()
                .with_child(signature_row)
                .with_spacer(common_components.spacer_size)
                .with_child(send_message_button)
                .with_spacer(common_components.spacer_size)
                .with_child(clear_button)
                .with_spacer(common_components.spacer_size)
                .with_child(back_button)
                .padding(druid::Insets::uniform(10.0)),
        )
    }
}
