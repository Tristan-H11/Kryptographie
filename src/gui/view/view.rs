

use druid::{Widget, widget::{Button, Label, TextBox, Flex}, Lens, WidgetExt};
use crate::gui::model::model::{AppModel, CurrentView, HauptMenuModel};
use crate::gui::model::model::AliceModel;
use crate::gui::model::model::BobModel;
use crate::gui::controller::controller::SWITCH_TO_ALICE;


pub(crate) fn build_haupt_menu() -> impl Widget<HauptMenuModel> {
    Flex::column()
        .with_child(TextBox::new().lens(HauptMenuModel::eingabe_p1))
        .with_child(TextBox::new().lens(HauptMenuModel::eingabe_p2))
        .with_child(Button::new("Öffne Alice Ansicht").on_click(|_ctx, _data: &mut HauptMenuModel, _env| {
            // Senden Sie einen Command anstatt den View direkt zu ändern
            _ctx.submit_command(SWITCH_TO_ALICE);
        }))
}

pub(crate) fn build_alice_view() -> impl Widget<AliceModel> {
    Flex::column()
        .with_child(TextBox::new().lens(AliceModel::eingabe_klartext))
}

pub(crate) fn build_bob_view() -> impl Widget<BobModel> {
    Flex::column()
        .with_child(TextBox::new().lens(BobModel::eingabe_klartext))
}
