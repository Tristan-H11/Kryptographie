use druid::{UnitPoint, widget::{Button, Flex, Label, TextBox}, Widget, WidgetExt};
use druid::widget::SizedBox;

pub trait ViewBuilder<Model> {
    fn build_view() -> SizedBox<Model>;
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
pub struct EntrySize {
    pub width: f64,
    pub height: f64,
}
impl Default for EntrySize {
    fn default() -> Self {
        Self { width: 200.0, height: 50.0 }
    }
}

#[derive(Clone, Copy)]
pub struct ButtonSize {
    pub width: f64,
    pub height: f64,
}
impl Default for ButtonSize {
    fn default() -> Self {
        Self { width: 250.0, height: 50.0 }
    }
}


#[derive(Clone, Copy)]
pub struct Spacing {
    pub spacing: f64,
}

impl Default for Spacing {
    fn default() -> Self {
        Self { spacing: 10.0 }
    }
}

pub struct CommonViewComponentsDefault {
    pub flex_hight_spacing: Spacing,
    pub flex_space_default: f64,
    pub left_space_default: f64,
    pub right_space_default: f64,
    pub alignment_default: Alignment,
}

impl CommonViewComponentsDefault {
    pub fn new() -> Self {
        let spacing = Spacing::default();
        let flex_space = 0.025;
        let left_space = 0.1;
        let right_space = 0.8;
        let alignment = Alignment::Center;
        Self {
            flex_hight_spacing: spacing,
            flex_space_default: flex_space,
            left_space_default: left_space,
            right_space_default: right_space,
            alignment_default: alignment,
        }
    }
    fn alignment_to_unitpoint(alignment: Alignment) -> UnitPoint {
        match alignment {
            Alignment::Left => UnitPoint::LEFT,
            Alignment::Center => UnitPoint::CENTER,
            Alignment::Right => UnitPoint::RIGHT,
        }
    }

    // static is with "with.child()" and dynamic is with "with_flex_child()"
    pub fn create_entry_static<Model: druid::Data>(
        &self,
        label_text: &str,
        placeholder: &str,
        disable_editing: bool,
        lens: impl druid::Lens<Model, String> + 'static,
        size: Option<EntrySize>,
        spacing: Option<f64>,
        alignment: Option<Alignment>,
    ) -> impl Widget<Model> {
        let size = size.unwrap_or_default();
        let spacing = spacing.unwrap_or(self.flex_hight_spacing.spacing);
        let alignment = alignment.unwrap_or(self.alignment_default);
        Flex::column()
            .with_child(
                Label::new(label_text),
            )
            .with_child(
                TextBox::multiline()
                    .with_placeholder(placeholder)
                    .fix_width(size.width)
                    .fix_height(size.height)
                    .disabled_if({
                        let disable = disable_editing.clone();
                        move |_, _| disable
                    })
                    .lens(lens),
            )
            .padding(spacing)
            .align_horizontal(Self::alignment_to_unitpoint(alignment))

    }

    pub fn create_entry_dynamic<Model: druid::Data>(
        &self,
        label_text: &str,
        placeholder: &str,
        disable_editing: bool,
        lens: impl druid::Lens<Model, String> + 'static,
        size: Option<EntrySize>,
        spacing: Option<f64>,
        alignment: Option<Alignment>,
    ) -> impl Widget<Model> {
        let size = size.unwrap_or_default();
        let spacing = spacing.unwrap_or(self.flex_hight_spacing.spacing);
        let alignment = alignment.unwrap_or(self.alignment_default);
        Flex::column()
            .with_flex_child(
                Label::new(label_text),
                0.1,
            )
            .with_flex_child(
                TextBox::multiline()
                    .with_placeholder(placeholder)
                    .fix_width(size.width)
                    .fix_height(size.height)
                    .disabled_if({
                        let disable = disable_editing.clone();
                        move |_, _| disable
                    })
                    .lens(lens),
                0.1,
            )
            .padding(spacing)
            .align_horizontal(Self::alignment_to_unitpoint(alignment))
    }

    pub fn create_button_static<Model: druid::Data>(
        &self,
        label_text: &str,
        command: impl Into<druid::Command> + Clone + 'static,
        size: Option<ButtonSize>,
        spacing: Option<f64>,
        alignment: Option<Alignment>,
    ) -> impl Widget<Model> {
        let command_clone = command.clone();
        let size = size.unwrap_or_default();
        let spacing = spacing.unwrap_or(self.flex_hight_spacing.spacing);
        let alignment = alignment.unwrap_or(self.alignment_default);
        Flex::row()
            .with_child(
                Button::new(label_text)
                    .on_click(move |ctx, _data: &mut Model, _env| {
                        ctx.submit_command(command_clone.clone().into());
                    })
                    .fix_width(size.width)
                    .fix_height(size.height),
            )
            .padding(spacing)
            .align_horizontal(Self::alignment_to_unitpoint(alignment))
    }

    pub fn create_button_dynamic<Model: druid::Data>(
        &self,
        label_text: &str,
        command: impl Into<druid::Command> + Clone + 'static,
        size: Option<ButtonSize>,
        spacing: Option<f64>,
        alignment: Option<Alignment>,
    ) -> impl Widget<Model> {
        let command_clone = command.clone();
        let size = size.unwrap_or_default();
        let spacing = spacing.unwrap_or(self.flex_hight_spacing.spacing);
        let alignment = alignment.unwrap_or(self.alignment_default);
        Flex::row()
            .with_flex_child(
                Button::new(label_text)
                    .on_click(move |ctx, _data: &mut Model, _env| {
                        ctx.submit_command(command_clone.clone().into());
                    })
                    .fix_width(size.width)
                    .fix_height(size.height),
                1.0,
            )
            .padding(spacing)
            .align_horizontal(Self::alignment_to_unitpoint(alignment))
    }
}