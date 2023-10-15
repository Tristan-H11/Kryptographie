mod encryption;
mod gui;
mod tests;

use druid::{AppLauncher, WindowDesc};
use gui::gui::build_ui;
use gui::model::model::AppState;

fn main() {
    let main_window = WindowDesc::new(build_ui()).title("My Rust GUI App");
    let initial_state = AppState::default();
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
