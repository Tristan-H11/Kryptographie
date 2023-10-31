use druid::{Widget, widget::Flex, WidgetExt};
use druid::widget::SizedBox;

use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, SWITCH_TO_ALICE, SWITCH_TO_BOB,
};
use crate::gui::model::model::MainMenuModel;
use crate::gui::view::traits::common_view_builder::{CommonViewComponentsDefault, ViewBuilder};

// trait für Hauptmenü
pub struct MainMenuViewBuilder;

impl ViewBuilder<MainMenuModel> for MainMenuViewBuilder {
    fn build_view() -> SizedBox<MainMenuModel> {
        let common_components = CommonViewComponentsDefault::new(); // Gemeinsame Komponenten instanziieren

        // Entry-Felder
        let modul_width_entry = common_components.create_text_entry_default(
            "Breite des Moduls n: ",
            "z.B. 4096",
            false,
            MainMenuModel::modul_width,
        );

        let miller_rabin_entry = common_components.create_text_entry_default(
            "Miller-Rabin Iterationen: ",
            "z.B. 100",
            false,
            MainMenuModel::miller_rabin_iterations,
        );

        let basis_entry = common_components.create_text_entry_default(
            "Basis für Kalkulationen: ",
            "Der Wert des höchsten Unicode-Zeichens exklusive, z.B. 55926",
            false,
            MainMenuModel::basis_length,
        );

        // Button
        let calculate_keypair_alice = common_components.create_button_default(
            "Schlüsselpaar <public, private> Alice berechnen",
            CALCULATE_KEYPAIR_ALICE,
        );

        let calculate_keypair_bob = common_components.create_button_default(
            "Schlüsselpaar <public, private> Bob berechnen",
            CALCULATE_KEYPAIR_BOB,
        );

        let open_alice_button =
            common_components.create_button_default("Alice-Ansicht öffnen", SWITCH_TO_ALICE);

        let open_bob_button = common_components.create_button_default("Bob-Ansicht öffnen", SWITCH_TO_BOB);

        let public_exponent_bob_entry = common_components.create_text_entry_default(
            "Öffentlicher Exponent Bob: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::public_exponent_bob,
        );

        let public_exponent_alice_entry = common_components.create_text_entry_default(
            "Öffentlicher Exponent Alice: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::public_exponent_alice,
        );

        let modul_alice_entry = common_components.create_text_entry_default(
            "Modul Alice: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::modul_alice,
        );

        let modul_bob_entry = common_components.create_text_entry_default(
            "Modul Bob: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::modul_bob,
        );


        // UI Struktur
        Flex::column()
            .with_flex_spacer(common_components.flex_space_default)
            .with_flex_child(modul_width_entry, 0.2)
            .with_flex_child(miller_rabin_entry, 0.2)
            .with_flex_child(basis_entry, 0.2)
            .with_flex_child(calculate_keypair_alice, 0.1)
            .with_flex_child(calculate_keypair_bob, 0.1)
            .with_flex_child(public_exponent_alice_entry, 0.2)
            .with_flex_child(modul_alice_entry, 0.2)
            .with_flex_child(public_exponent_bob_entry, 0.2)
            .with_flex_child(modul_bob_entry, 0.2)
            .with_flex_child(open_alice_button, 0.1)
            .with_flex_child(open_bob_button, 0.1)
            .expand()
    }
}
