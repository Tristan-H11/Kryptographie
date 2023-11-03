use druid::widget::{Label, SizedBox};
use druid::{widget::Flex, WidgetExt};

use crate::gui::controller::commands::{
    CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::BobModel;
use crate::gui::view::traits::common_view_builder::{
    Alignment, CommonViewComponentsDefault, EntrySize, ViewBuilder,
};

pub struct BobViewBuilder;

impl ViewBuilder<BobModel> for BobViewBuilder {
    fn build_view() -> SizedBox<BobModel> {
        let common_components = CommonViewComponentsDefault::new();

        let cust_size_var_1 = EntrySize {
            width: 500.0,
            height: 50.0,
        };
        let cust_size_var_2 = EntrySize {
            width: 1200.0,
            height: 75.0,
        };

        // Entry-Felder und Labels
        let plaintext_entry = common_components.create_entry_static(
            "Klartext: ",
            "z.B. Hallo wie geht es dir?",
            false,
            BobModel::plaintext_msg,
            Some(cust_size_var_1),
            None,
            Some(Alignment::Left),
        );

        let ciphertext_entry = common_components.create_entry_static(
            "Geheimtext: ",
            "Wird automatisch berechnet",
            false,
            BobModel::ciphre_msg,
            Some(cust_size_var_1),
            None,
            Some(Alignment::Left),
        );

        let secret_exponent_entry = common_components.create_entry_static(
            "Geheimer Exponent: ",
            "Wird automatisch berechnet",
            true,
            BobModel::private_exponent,
            Some(cust_size_var_2),
            None,
            Some(Alignment::Left),
        );

        let signature_entry = common_components.create_entry_static(
            "Signatur: ",
            "Wird automatisch berechnet",
            true,
            BobModel::signature_msg,
            Some(cust_size_var_2),
            None,
            Some(Alignment::Left),
        );

        let signature_status_label = Label::dynamic(|data: &BobModel, _| {
            if data.signature_status {
                "Status: Gültig".to_string()
            } else {
                "Status: Ungültig".to_string()
            }
        });
        let signature_row = Flex::row()
            .with_child(signature_entry)
            .with_child(signature_status_label);

        // Buttons
        let encrypt_button = common_components.create_button_static(
            "Mit Alice PublicKey verschlüsseln",
            ENCRYPT,
            None,
            None,
            None,
        );
        let calc_sign_button =
            common_components.create_button_static("Signatur berechnen", SIGN, None, None, None);
        let check_sign_button =
            common_components.create_button_static("Signatur prüfen", VERIFY, None, None, None);
        let decrypt_button = common_components.create_button_static(
            "Mit eigenem PrivateKey entschlüsseln",
            DECRYPT,
            None,
            None,
            None,
        );
        let send_message_button = common_components.create_button_static(
            "Nachricht senden",
            SEND_MESSAGE,
            None,
            None,
            None,
        );
        let clear_button = common_components.create_button_static(
            "Alles außer privaten Schlüssel löschen",
            CLEAR,
            None,
            None,
            None,
        );
        let back_button = common_components.create_button_static(
            "Zurück zum Hauptmenü",
            SWITCH_TO_MAIN_MENU,
            None,
            None,
            None,
        );

        Flex::column()
            .with_flex_spacer(common_components.flex_space_default)
            .with_flex_child(plaintext_entry, 0.2)
            .with_flex_child(ciphertext_entry, 0.2)
            .with_flex_child(encrypt_button, 0.1)
            .with_flex_child(decrypt_button, 0.1)
            .with_flex_child(secret_exponent_entry, 0.2)
            .with_flex_child(calc_sign_button, 0.1)
            .with_flex_child(check_sign_button, 0.1)
            .with_flex_child(signature_row, 0.2)
            .with_flex_child(send_message_button, 0.1)
            .with_flex_child(clear_button, 0.1)
            .with_flex_child(back_button, 0.1)
            .expand_height()
    }
}
