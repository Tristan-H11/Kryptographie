use druid::{
    widget::{Button, Flex, Label, TextBox},
    Widget, WidgetExt,
};

// trait für gemeinsame Merkmale von Ansichten
pub trait ViewBuilder<Model> {
    fn build_view() -> Box<dyn Widget<Model>>;
}

//gemeinsam verwendete traits / strukturen
pub struct CommonViewComponents {
    pub fixed_width_entry_label: f64,
    pub fixed_width_textbox: f64,
    pub fixed_width_button: f64,
    pub spacer_size: f64,
}
impl CommonViewComponents {
    pub fn new() -> Self {
        let width_entry_label = 200.0;
        let width_textbox = 550.0;
        let width_button = width_entry_label + width_textbox;
        let spacer_size = 40.0;
        Self {
            fixed_width_entry_label: width_entry_label,
            fixed_width_textbox: width_textbox,
            fixed_width_button: width_button,
            spacer_size,
        }
    }

    pub fn create_text_entry<Model: druid::Data>(
        &self,
        label_text: &str,
        placeholder: &str,
        lens: impl druid::Lens<Model, String> + 'static,
    ) -> impl Widget<Model> {
        Flex::row()
            .with_child(Label::new(label_text).fix_width(self.fixed_width_entry_label))
            .with_child(
                TextBox::multiline()
                    .with_placeholder(placeholder)
                    .fix_width(self.fixed_width_textbox)
                    .lens(lens),
            )
    }

    pub fn create_button<Model: druid::Data>(
        &self,
        label_text: &str,
        command: impl Into<druid::Command> + Clone + 'static,
    ) -> impl Widget<Model> {
        let command_clone = command.clone();
        Button::new(label_text)
            .on_click(move |ctx, _data: &mut Model, _env| {
                ctx.submit_command(command_clone.clone().into());
            })
            .fix_width(self.fixed_width_button)
    }
}
