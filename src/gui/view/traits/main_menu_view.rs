use druid::widget::SizedBox;
use druid::{widget::Flex, WidgetExt};

use crate::gui::controller::commands::{
    CALCULATE_KEYPAIR_ALICE, CALCULATE_KEYPAIR_BOB, SWITCH_TO_ALICE, SWITCH_TO_BOB,
};
use crate::gui::model::model::MainMenuModel;
use crate::gui::view::traits::common_view_builder::{
    Alignment, ButtonSize, CommonViewComponentsDefault, EntrySize, ViewBuilder,
};

// trait für Hauptmenü
pub struct MainMenuViewBuilder;

impl ViewBuilder<MainMenuModel> for MainMenuViewBuilder {
    fn build_view() -> SizedBox<MainMenuModel> {
        let common_components = CommonViewComponentsDefault::new(); // Gemeinsame Komponenten instanziieren

        let cust_e_size_var_1 = EntrySize {
            width: 250.0,
            height: 25.0,
        };
        let cust_e_size_var_2 = EntrySize {
            width: 250.0,
            height: 50.0,
        };
        let cust_e_size_var_3 = EntrySize {
            width: 1200.0,
            height: 75.0,
        };

        let cust_b_size_var_1 = ButtonSize {
            width: 400.0,
            height: 50.0,
        };

        // Entry-Felder
        let modul_width_entry = common_components.create_entry_static(
            "Breite des Moduls n: ",
            "z.B. 4096",
            false,
            MainMenuModel::modul_width,
            Some(cust_e_size_var_1),
            None,
            Some(Alignment::Left),
        );

        let miller_rabin_entry = common_components.create_entry_static(
            "Miller-Rabin Iterationen: ",
            "z.B. 100",
            false,
            MainMenuModel::miller_rabin_iterations,
            Some(cust_e_size_var_1),
            None,
            Some(Alignment::Left),
        );

        let basis_entry = common_components.create_entry_static(
            "Basis für Kalkulationen: ",
            "Der Wert des höchsten Unicode- \n Zeichens exklusive, z.B. 55926",
            false,
            MainMenuModel::basis_length,
            Some(cust_e_size_var_2),
            None,
            Some(Alignment::Left),
        );

        let random_seed_entry = common_components.create_entry_static(
            "Random Seed: ",
            "Seed für die Erzeugung der \n Primzahlen. Keine Quadratzahl! Z.B. 13",
            false,
            MainMenuModel::random_seed,
            Some(cust_e_size_var_2),
            None,
            Some(Alignment::Left),
        );

        let public_exponent_bob_entry = common_components.create_entry_static(
            "Öffentlicher Exponent Bob: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::public_exponent_bob,
            Some(cust_e_size_var_3),
            None,
            None,
        );

        let public_exponent_alice_entry = common_components.create_entry_static(
            "Öffentlicher Exponent Alice: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::public_exponent_alice,
            Some(cust_e_size_var_3),
            None,
            None,
        );

        let modul_alice_entry = common_components.create_entry_static(
            "Modul Alice: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::modul_alice,
            Some(cust_e_size_var_3),
            None,
            None,
        );

        let modul_bob_entry = common_components.create_entry_static(
            "Modul Bob: ",
            "Wird automatisch berechnet",
            true,
            MainMenuModel::modul_bob,
            Some(cust_e_size_var_3),
            None,
            None,
        );

        // Button
        let calculate_keypair_alice = common_components.create_button_static(
            "Schlüsselpaar <public, private> Alice berechnen",
            CALCULATE_KEYPAIR_ALICE,
            Some(cust_b_size_var_1),
            None,
            None,
        );

        let calculate_keypair_bob = common_components.create_button_static(
            "Schlüsselpaar <public, private> Bob berechnen",
            CALCULATE_KEYPAIR_BOB,
            Some(cust_b_size_var_1),
            None,
            None,
        );

        let open_alice_button = common_components.create_button_static(
            "Alice-Ansicht öffnen",
            SWITCH_TO_ALICE,
            None,
            None,
            None,
        );

        let open_bob_button = common_components.create_button_static(
            "Bob-Ansicht öffnen",
            SWITCH_TO_BOB,
            None,
            None,
            None,
        );

        // UI Struktur
        Flex::column()
            .with_flex_spacer(common_components.flex_space_default)
            .with_flex_child(modul_width_entry, 0.2)
            .with_flex_child(miller_rabin_entry, 0.2)
            .with_flex_child(basis_entry, 0.2)
            .with_flex_child(random_seed_entry, 0.2)
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
