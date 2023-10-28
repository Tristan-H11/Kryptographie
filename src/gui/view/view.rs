use crate::gui::view::traits::view_builder::{MainMenuViewBuilder, AliceViewBuilder, BobViewBuilder, ViewBuilder};
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