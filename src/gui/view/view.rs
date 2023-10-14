use druid::{Command, Widget, widget::{Button, Label, TextBox, Flex}, Lens, WidgetExt, Selector};
use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel, AppState, View};

// Definieren Sie einen benutzerdefinierten Selector
pub const SWITCH_TO_ALICE: druid::Selector = druid::Selector::new("switch-to-alice");

fn build_haupt_menu() -> impl Widget<HauptMenuModel> {
    druid::widget::Flex::column()
        .with_child(druid::widget::TextBox::new().lens(HauptMenuModel::eingabe_p1))
        .with_child(druid::widget::TextBox::new().lens(HauptMenuModel::eingabe_p2))
        .with_child(druid::widget::Button::new("Öffne Alice Ansicht").on_click(|ctx, _data: &mut HauptMenuModel, _env| {
            // Send the SwitchToAlice command using the custom Selector
            ctx.submit_command(SWITCH_TO_ALICE);
        }))
}

pub(crate) fn build_alice_view() -> impl druid::Widget<AliceModel> {
    druid::widget::Flex::column()
        .with_child(druid::widget::TextBox::new().lens(AliceModel::eingabe_klartext))
}

pub(crate) fn build_bob_view() -> impl druid::Widget<BobModel> {
    druid::widget::Flex::column()
        .with_child(druid::widget::TextBox::new().lens(BobModel::eingabe_klartext))
}

