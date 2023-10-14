use crate::gui::model::model::AppState;
use crate::gui::gui_math::GuiMath;
extern crate druid;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{Data, Env, Lens, Widget, WidgetExt};
use crate::gui::controller;
use crate::gui::controller::main_mask_controller::MainMaskController;

#[derive(Clone)]
pub struct MainMaskView;

impl MainMaskView {

    pub fn display_public_keys(&self) {
        // todo -- anzeige öffentlicher schlüssel
    }
    pub fn build_main_ui_mask() -> impl Widget<AppState> {
        let miller_rabin_input = TextBox::new()
            .with_placeholder("Eingabe Schritte Miller-Rabin")
            .lens(AppState::miller_rabin_input)
            .fix_width(300.0)
            .padding(10.0);

        let length_p1_input = TextBox::new()
            .with_placeholder("Länge von p1")
            .lens(AppState::length_p1_input)
            .fix_width(300.0)
            .padding(10.0);

        let length_p2_input = TextBox::new()
            .with_placeholder("Länge von p2")
            .lens(AppState::length_p2_input)
            .fix_width(300.0)
            .padding(10.0);

        let culc_open_key_button = Button::new("Öffentlichen Schlüssel berechnen")
            .on_click(|_ctx, data: &mut AppState, _env| {
                //todo -- berechnung öffnetlicher schlüssel
                data.open_key_result__e_a = format!("Berechneter Schlüssel für {}", data.miller_rabin_input);
            })
            .padding(10.0);

        let show_open_key_button = Label::new(|data: &AppState, _env: &_| {
            format!("Öffentliche Schlüssel: {}", data.open_key_result__e_a)
        }).padding(10.0);

        let alice_mask_open_button = Button::new("Alice Mask anzeigen")
            .on_click(|ctx, data: &mut AppState, env: &Env| {
                if let Ok(main_controller) = data.main_controller.lock() {
                    main_controller.display_alice_mask(ctx, data, env);
                }
            })
            .padding(10.0);



        let bob_mask_open_button = Button::new("Bob Mask anzeigen")
            .on_click(|_ctx, data: &mut AppState, _env| {
                // todo -- anzeige bob mask
            })
            .padding(10.0);

        let mut flex = Flex::column();
        flex.add_child(miller_rabin_input);
        flex.add_spacer(10.0);
        flex.add_child(length_p1_input);
        flex.add_spacer(10.0);
        flex.add_child(length_p2_input);
        flex.add_spacer(10.0);
        flex.add_child(culc_open_key_button);
        flex.add_spacer(10.0);
        flex.add_child(show_open_key_button);
        flex.add_spacer(10.0);
        flex.add_child(alice_mask_open_button);
        flex.add_spacer(10.0);
        flex.add_child(bob_mask_open_button);
        flex.add_spacer(10.0);
        flex
    }
}