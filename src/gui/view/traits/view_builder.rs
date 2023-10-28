
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



//gemeinsam verwendete traits / strukturen
pub struct CommonViewComponents {
    pub fixed_width_entry_label: f64,
    pub fixed_width_textbox: f64,
    pub fixed_width_button: f64,
    pub spacer_size: f64,
}
impl CommonViewComponents {
    pub fn new() -> Self {
        Self {
            fixed_width_entry_label: 200.0,
            fixed_width_textbox: 400.0,
            fixed_width_button: 200.0 + 400.0,
            spacer_size: 40.0,
        }
    }

    pub fn create_text_entry<Model: druid::Data>(
        &self,
        label_text: &str,
        placeholder: &str,
        lens: impl druid::Lens<Model, String> + 'static,
    ) -> impl Widget<Model> {
        Flex::row()
            .with_child(Label::new(label_text).fix_width(self.fixed_width_entry_label))
            .with_child(
                TextBox::new()
                    .with_placeholder(placeholder)
                    .fix_width(self.fixed_width_textbox)
                    .lens(lens),
            )
    }

    pub fn create_button<Model: druid::Data>(
        &self,
        label_text: &str,
        command: impl Into<druid::Command> + Clone + 'static,
    ) -> impl Widget<Model> {
        let command_clone = command.clone();
        Button::new(label_text)
            .on_click(move |ctx, _data: &mut Model, _env| {
                ctx.submit_command(command_clone.clone().into());
            })
            .fix_width(self.fixed_width_button)
    }
}



// trait für Hauptmenü
pub struct MainMenuViewBuilder;

impl ViewBuilder<MainMenuModel> for MainMenuViewBuilder {
    fn build_view() -> Box<dyn Widget<MainMenuModel>> {
        let common_components = CommonViewComponents::new();  // Gemeinsame Komponenten instanziieren

        // Entry-Felder
        let modul_width_entry = common_components.create_text_entry(
            "Breite des Moduls n: ",
            "z.B. 4096",
            MainMenuModel::modul_width,
        );

        let miller_rabin_entry = common_components.create_text_entry(
            "Miller-Rabin Iterationen: ",
            "z.B. 100",
            MainMenuModel::miller_rabin_iterations,
        );

        let basis_entry = common_components.create_text_entry(
            "Basis für Kalkulationen: ",
            "default = 55296",
            MainMenuModel::basis_length,
        );

        // Button
        let calculate_keypair_alice = common_components.create_button(
            "Schlüsselpaar <public, private> Alice berechnen",
            CALCULATE_KEYPAIR_ALICE,
        );

        let calculate_keypair_bob = common_components.create_button(
            "Schlüsselpaar <public, private> Bob berechnen",
            CALCULATE_KEYPAIR_BOB,
        );

        let open_alice_button = common_components.create_button(
            "Alice-Ansicht öffnen",
            SWITCH_TO_ALICE,
        );

        let open_bob_button = common_components.create_button(
            "Bob-Ansicht öffnen",
            SWITCH_TO_BOB,
        );

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

        // UI Struktur
        Box::new(
            Flex::column()
                .with_default_spacer()
                .with_default_spacer()
                .with_child(modul_width_entry)
                .with_default_spacer()
                .with_child(miller_rabin_entry)
                .with_default_spacer()
                .with_child(basis_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(calculate_keypair_alice)
                .with_default_spacer()
                .with_child(calculate_keypair_bob)
                .with_spacer(common_components.spacer_size)
                .with_child(public_exponent_alice_label)
                .with_default_spacer()
                .with_child(public_exponent_bob_label)
                .with_spacer(common_components.spacer_size)
                .with_child(open_alice_button)
                .with_default_spacer()
                .with_child(open_bob_button))
    }
}





// trait für Alice View
pub struct AliceViewBuilder;

impl ViewBuilder<AliceModel> for AliceViewBuilder {
    fn build_view() -> Box<dyn Widget<AliceModel>> {
        //verwende gemeinsame Komponenten s.o.
        let common_components = CommonViewComponents::new();

        let secret_exponent_label = Label::new(|data: &AliceModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let wrapped_text = wrapper.key_text_wrapper(&format!("Geheimer Exponent: {}", data.private_exponent), 75);
            format!("Geheimer Exponent: \n{}", wrapped_text)
        })
            .expand_width();

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

        let signature_row = common_components.create_text_entry(
            "Signatur: ",
            "",
            AliceModel::signature,
        );

        let encrypt_button = common_components.create_button("Mit Bobs PublicKey verschlüsseln", ENCRYPT);
        let calc_sign_button = common_components.create_button("Signatur berechnen", SIGN);
        let check_sign_button = common_components.create_button("Signatur prüfen", VERIFY);
        let decrypt_button = common_components.create_button("Mit eigenem PrivateKey entschlüsseln", DECRYPT);
        let send_message_button = common_components.create_button("Nachricht senden", SEND_MESSAGE);
        let clear_button = common_components.create_button("Alles außer privaten Schlüssel löschen", CLEAR);
        let back_button = common_components.create_button("Zurück zum Hauptmenü", SWITCH_TO_MAIN_MENU);

        Box::new(
            Flex::column()
                .with_default_spacer()
                .with_default_spacer()
                .with_child(secret_exponent_label)
                .with_spacer(common_components.spacer_size)
                .with_child(plaintext_entry)
                .with_default_spacer()
                .with_child(ciphertext_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(encrypt_button)
                .with_default_spacer()
                .with_child(decrypt_button)
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
                .padding(druid::Insets::uniform(10.0))
        )
    }
}



// Trait für Bob View
pub struct BobViewBuilder;

impl ViewBuilder<BobModel> for BobViewBuilder {
    fn build_view() -> Box<dyn Widget<BobModel>> {
        let common_components = CommonViewComponents::new();

        // Label
        let secret_exponent_label = Label::new(|data: &BobModel, _env: &Env| -> String {
            let wrapper = KeyTextWrapper;
            let wrapped_text = wrapper.key_text_wrapper(&format!("Geheimer Exponent: {}", data.private_exponent), 75);
            format!("Geheimer Exponent: \n{}", wrapped_text)
        })
            .expand_width();

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

        let signature_row = common_components.create_text_entry(
            "Signatur: ",
            "Signatur",
            BobModel::signature,
        );

        // Buttons
        let encrypt_button = common_components.create_button("Mit Alice PublicKey verschlüsseln", ENCRYPT);
        let calc_sign_button = common_components.create_button("Signatur berechnen", SIGN);
        let check_sign_button = common_components.create_button("Signatur prüfen", VERIFY);
        let decrypt_button = common_components.create_button("Mit eigenem PrivateKey entschlüsseln", DECRYPT);
        let send_message_button = common_components.create_button("Nachricht senden", SEND_MESSAGE);
        let clear_button = common_components.create_button("Alles außer privaten Schlüssel löschen", CLEAR);
        let back_button = common_components.create_button("Zurück zum Hauptmenü", SWITCH_TO_MAIN_MENU);

        // UI Struktur
        Box::new(
            Flex::column()
                .with_default_spacer()
                .with_default_spacer()
                .with_child(secret_exponent_label)
                .with_spacer(common_components.spacer_size)
                .with_child(plaintext_entry)
                .with_default_spacer()
                .with_child(ciphertext_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(encrypt_button)
                .with_default_spacer()
                .with_child(decrypt_button)
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
                .padding(druid::Insets::uniform(10.0))
        )
    }
}

