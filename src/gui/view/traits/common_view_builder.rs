use druid::{
    widget::{Button, Flex, Label, TextBox},
    Widget, WidgetExt,
};
use druid::widget::SizedBox;

pub trait ViewBuilder<Model> {
    fn build_view() -> SizedBox<Model>;
}

pub struct CommonViewComponentsDefault {
    pub padding_default: f64,
    pub flex_space_default: f64,
    pub left_space_default: f64,
    pub right_space_default: f64,
}

impl CommonViewComponentsDefault {
    pub fn new() -> Self {
        let flex_space = 0.025;
        let left_space = 0.1;
        let right_space = 0.8;
        let padding = 10.0;
        Self {
            flex_space_default: flex_space,
            padding_default: padding,
            left_space_default: left_space,
            right_space_default: right_space,
        }
    }

    pub fn create_text_entry_default<Model: druid::Data>(
        &self,
        label_text: &str,
        placeholder: &str,
        disable_editing: bool,
        lens: impl druid::Lens<Model, String> + 'static,
    ) -> impl Widget<Model> {
        Flex::row()
            .with_flex_spacer(self.flex_space_default)
            .with_flex_child(
                Label::new(label_text)
                    .expand(),
                (1.0 - self.flex_space_default) * self.left_space_default,
            )
            .with_flex_child(
                TextBox::multiline()
                    .with_placeholder(placeholder)
                    .expand()
                    // Deaktiviert die Textbox, wenn disable_editing true ist
                    .disabled_if({
                        let disable = disable_editing.clone();
                        move |_, _| disable
                    })
                    .lens(lens),
                (1.0 - self.flex_space_default) * self.right_space_default,
            )
            .with_flex_spacer(self.flex_space_default)
            .padding(self.padding_default)
    }

    pub fn create_button_default<Model: druid::Data>(
        &self,
        label_text: &str,
        command: impl Into<druid::Command> + Clone + 'static,
    ) -> impl Widget<Model> {
        let command_clone = command.clone();
        Flex::row()
            .with_flex_spacer(self.flex_space_default)
            .with_flex_spacer( (1.0 - self.flex_space_default) * self.left_space_default)
            .with_flex_child(
                Button::new(label_text)
                    .on_click(move |ctx, _data: &mut Model, _env| {
                        ctx.submit_command(command_clone.clone().into());
                    })
                    .expand(),
                (1.0 - self.flex_space_default) * self.right_space_default,
            )
            .with_flex_spacer(self.flex_space_default)
            .padding(self.padding_default)
    }
}
