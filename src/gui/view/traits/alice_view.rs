use druid::widget::{Label, SizedBox};
use druid::{widget::Flex, WidgetExt};

use crate::gui::controller::commands::{
    CLEAR, DECRYPT, ENCRYPT, SEND_MESSAGE, SIGN, SWITCH_TO_MAIN_MENU, VERIFY,
};
use crate::gui::model::model::AliceModel;
use crate::gui::view::traits::common_view_builder::{
    Alignment, CommonViewComponentsDefault, EntrySize, ViewBuilder,
};

pub struct AliceViewBuilder;

impl ViewBuilder<AliceModel> for AliceViewBuilder {
    fn build_view() -> SizedBox<AliceModel> {
        let common_components = CommonViewComponentsDefault::new();

        let cust_size_var_1 = EntrySize {
            width: 800.0,
            height: 200.0,
        };
        let cust_size_var_2 = EntrySize {
            width: 1200.0,
            height: 50.0,
        };
        let cust_size_var_3 = EntrySize {
            width: 1090.0,
            height: 50.0,
        };

        let plaintext_entry = common_components.create_entry_static(
            "Klartext: ",
            "z.B. Hallo wie geht es dir?",
            false,
            AliceModel::plaintext_msg,
            Some(cust_size_var_1),
            None,
            Some(Alignment::Left),
        );

        let ciphertext_entry = common_components.create_entry_static(
            "Geheimtext: ",
            "Wird automatisch berechnet",
            false,
            AliceModel::ciphre_msg,
            Some(cust_size_var_1),
            None,
            Some(Alignment::Left),
        );
        let plainttext_chiffre_row = Flex::row()
            .with_child(plaintext_entry)
            .with_child(ciphertext_entry);

        let encrypt_button = common_components.create_button_static(
            "Mit Bobs PublicKey verschlüsseln",
            ENCRYPT,
            None,
            None,
            None,
        );
        let decrypt_button = common_components.create_button_static(
            "Mit eigenem PrivateKey entschlüsseln",
            DECRYPT,
            None,
            None,
            None,
        );
        let encrypt_decrypt_row = Flex::row()
            .with_child(encrypt_button)
            .with_child(decrypt_button);

        let secret_exponent_entry = common_components.create_entry_static(
            "Geheimer Exponent: ",
            "Wird automatisch berechnet",
            true,
            AliceModel::private_exponent,
            Some(cust_size_var_2),
            None,
            Some(Alignment::Center),
        );

        let signature_entry = common_components.create_entry_static(
            "Signatur: ",
            "Wird automatisch berechnet",
            true,
            AliceModel::signature_msg,
            Some(cust_size_var_3),
            None,
            Some(Alignment::Left),
        );

        let signature_status_label = Label::dynamic(|data: &AliceModel, _| {
            if data.signature_status {
                "Status: Gültig".to_string()
            } else {
                "Status: Ungültig".to_string()
            }
        });
        let signature_row = Flex::row()
            .with_child(signature_entry)
            .with_child(signature_status_label);

        let calc_sign_button =
            common_components.create_button_static("Signatur berechnen", SIGN, None, None, None);
        let check_sign_button =
            common_components.create_button_static("Signatur prüfen", VERIFY, None, None, None);

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
            .with_flex_child(plainttext_chiffre_row, 0.2)
            .with_flex_spacer(0.125)
            .with_flex_child(encrypt_decrypt_row, 0.2)
            .with_flex_child(secret_exponent_entry, 0.1)
            .with_flex_child(calc_sign_button, 0.1)
            .with_flex_child(check_sign_button, 0.1)
            .with_flex_child(signature_row, 0.2)
            .with_flex_child(send_message_button, 0.1)
            .with_flex_child(clear_button, 0.1)
            .with_flex_child(back_button, 0.1)
            .expand_height()
    }
}
