use druid::{
    Widget,
    widget::Flex, WidgetExt,
};

use crate::gui::controller::commands::{
    CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::AliceModel;
use crate::gui::view::traits::common_view_builder::{CommonViewComponents, ViewBuilder};

// trait für Alice View
pub struct AliceViewBuilder;

impl ViewBuilder<AliceModel> for AliceViewBuilder {
    fn build_view() -> Box<dyn Widget<AliceModel>> {
        //verwende gemeinsame Komponenten s.o.
        let common_components = CommonViewComponents::new();

        let plaintext_entry = common_components.create_text_entry(
            "Klartext: ",
            "z.B. Hallo wie geht es dir?",
            AliceModel::plaintext,
        );

        let ciphertext_entry = common_components.create_text_entry(
            "Geheimtext: ",
            "Erzeugt durch Berechnung.",
            AliceModel::ciphertext,
        );

        let secret_exponent_entry = common_components.create_text_entry(
            "Geheimer Exponent: ",
            "wird automatisch berechnet",
            AliceModel::private_exponent,
        );

        let signature_row =
            common_components.create_text_entry("Signatur: ", "", AliceModel::signature);

        let encrypt_button =
            common_components.create_button("Mit Bobs PublicKey verschlüsseln", ENCRYPT);
        let calc_sign_button = common_components.create_button("Signatur berechnen", SIGN);
        let check_sign_button = common_components.create_button("Signatur prüfen", VERIFY);
        let decrypt_button =
            common_components.create_button("Mit eigenem PrivateKey entschlüsseln", DECRYPT);
        let send_message_button = common_components.create_button("Nachricht senden", SEND_MESSAGE);
        let clear_button =
            common_components.create_button("Alles außer privaten Schlüssel löschen", CLEAR);
        let back_button =
            common_components.create_button("Zurück zum Hauptmenü", SWITCH_TO_MAIN_MENU);

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
