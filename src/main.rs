mod rsa;
mod tests;
mod gui;

use druid::{AppLauncher, Data, Lens, LocalizedString, WindowDesc};
use gui::model::model::AppState;

fn main() {
    let main_window = WindowDesc::new(gui::gui::build_ui)
        .title(LocalizedString::new("Hauptmenü").with_placeholder("Hauptmenü"));

    let initial_state = AppState::default();

    let app = AppLauncher::with_window(main_window);
    app.launch(initial_state).expect("Failed to launch application");
}
