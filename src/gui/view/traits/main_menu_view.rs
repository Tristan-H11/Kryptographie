use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, SWITCH_TO_ALICE, SWITCH_TO_BOB,
};
use crate::gui::model::model::MainMenuModel;
use crate::gui::view::traits::common_view_builder::{CommonViewComponents, ViewBuilder};
use crate::gui::view::traits::key_text_wrapper::{KeyTextWrapper, TextWrapper};
use druid::{
    widget::{Flex, Label},
    Env, Widget, WidgetExt,
};

// trait für Hauptmenü
pub struct MainMenuViewBuilder;

impl ViewBuilder<MainMenuModel> for MainMenuViewBuilder {
    fn build_view() -> Box<dyn Widget<MainMenuModel>> {
        let common_components = CommonViewComponents::new(); // Gemeinsame Komponenten instanziieren

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

        let open_alice_button =
            common_components.create_button("Alice-Ansicht öffnen", SWITCH_TO_ALICE);

        let open_bob_button = common_components.create_button("Bob-Ansicht öffnen", SWITCH_TO_BOB);

        let public_exponent_bob_entry = common_components.create_text_entry(
            "Öffentlicher Exponent Bob: ",
            "wird automatisch berechnet",
            MainMenuModel::public_exponent_bob,
        );

        let public_exponent_alice_entry = common_components.create_text_entry(
            "Öffentlicher Exponent Alice: ",
            "wird automatisch berechnet",
            MainMenuModel::public_exponent_alice,
        );

        let modul_alice_entry = common_components.create_text_entry(
            "Modul Alice: ",
            "wird automatisch berechnet",
            MainMenuModel::modul_alice,
        );

        let modul_bob_entry = common_components.create_text_entry(
            "Modul Bob: ",
            "wird automatisch berechnet",
            MainMenuModel::modul_bob,
        );


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
                .with_child(public_exponent_alice_entry)
                .with_default_spacer()
                .with_child(modul_alice_entry)
                .with_default_spacer()
                .with_child(public_exponent_bob_entry)
                .with_default_spacer()
                .with_child(modul_bob_entry)
                .with_spacer(common_components.spacer_size)
                .with_child(open_alice_button)
                .with_default_spacer()
                .with_child(open_bob_button),
        )
    }
}
