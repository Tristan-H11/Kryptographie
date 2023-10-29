use crate::gui::view::traits::common_view_builder::{ ViewBuilder};
use crate::gui::view::traits::alice_view::AliceViewBuilder;
use crate::gui::view::traits::bob_view::BobViewBuilder;
use crate::gui::view::traits::main_menu_view::MainMenuViewBuilder;
use crate::gui::model::model::{AliceModel, BobModel, MainMenuModel};
use druid::Widget;

// Stellen Sie Wrapper-Funktionen bereit, um die Methoden der View-Builder aufzurufen
pub(crate) fn build_haupt_menu() -> impl Widget<MainMenuModel> {
    MainMenuViewBuilder::build_view()
}

pub(crate) fn build_alice_view() -> impl Widget<AliceModel> {
    AliceViewBuilder::build_view()
}

pub(crate) fn build_bob_view() -> impl Widget<BobModel> {
    BobViewBuilder::build_view()
}