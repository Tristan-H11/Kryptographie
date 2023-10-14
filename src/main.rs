mod rsa;
mod tests;
mod gui;

use druid::{AppLauncher, WindowDesc};
use gui::model::model::AppState;
use gui::gui::build_ui;

fn main() {
    let main_window = WindowDesc::new(build_ui()).title("My Rust GUI App");
    let initial_state = AppState::default();
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
