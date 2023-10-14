use std::sync::Arc;
use druid::{AppLauncher, Env, EventCtx, LocalizedString, WindowDesc};
use crate::gui::{controller, gui, view};
use crate::gui::model::model::{AppState, CryptoOperations};
use crate::gui::view::alice_mask_view::AliceMaskView;
use crate::gui::view::main_mask_view::MainMaskView;

extern crate druid;


#[derive(Clone)]
pub struct MainMaskController {
    crypto_operations: CryptoOperations,
    view: MainMaskView,
}

impl MainMaskController {
    pub fn new() -> Self {
        Self {
            crypto_operations: CryptoOperations::new(),
            view: MainMaskView,
        }
    }
    pub fn handle_open_key_button_click(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        // todo -- berechnung öffnetlicher schlüssel

        // result to view
        data.open_key_result__e_a = format!("Berechneter Schlüssel für {}", data.miller_rabin_input);
    }

    pub fn generate_keys(&mut self) {
        self.crypto_operations.generate_keys();
        self.view.display_public_keys();
    }

    pub fn display_alice_mask(&self, ctx: &mut EventCtx, data: &AppState, _env: &Env) {
        ctx.window().close();
        let alice_mask = AliceMaskView::build_alice_ui_mask();
        let alice_window = WindowDesc::new(move || alice_mask)
            .title(LocalizedString::new("Alice Maske"))
            .window_size(gui::calculate_window_size());

        AppLauncher::with_window(alice_window)
            .launch(AppState::new())
            .expect("launch failed");
    }


    pub fn display_bob_mask(&self) {
        // todo -- realisierung anzeige bob mask
    }
}
