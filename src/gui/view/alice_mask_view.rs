
// src/gui/view/alice_mask_view.rs
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{Data, Lens, Widget, WidgetExt};
use crate::gui::model::model::AppState;

pub struct AliceMaskView;

impl AliceMaskView {
    pub fn build_alice_ui_mask() -> impl Widget<AppState> {
        let secret_key_label = Label::new("Geheimer Schlüssel:").padding(10.0);
        let alice_mask_open_button = Button::new("Alice Mask anzeigen")
            .padding(10.0);

        let mut flex = Flex::column();
        flex.add_child(secret_key_label);
        flex.add_child(alice_mask_open_button);
        flex
    }
}
