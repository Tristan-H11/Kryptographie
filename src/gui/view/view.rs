

use druid::{Widget, widget::{Button, Label, TextBox, Flex}, Lens, WidgetExt};
use crate::gui::model::model::{AliceModel, BobModel, HauptMenuModel, AppState, View};


pub(crate) fn build_haupt_menu() -> impl druid::Widget<HauptMenuModel> {
    druid::widget::Flex::column()
        .with_child(druid::widget::TextBox::new().lens(HauptMenuModel::eingabe_p1))
        .with_child(druid::widget::TextBox::new().lens(HauptMenuModel::eingabe_p2))
        .with_child(druid::widget::Button::new("Öffne Alice Ansicht").on_click(|_ctx, _data: &mut HauptMenuModel, _env| {
            // Senden Sie einen Command anstatt den View direkt zu ändern
            //_ctx.submit_command(SWITCH_TO_ALICE);
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
