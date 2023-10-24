mod encryption;
mod gui;
mod tests;

use druid::{AppLauncher, WindowDesc};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use gui::gui::build_ui;
use gui::model::model::AppState;

fn main() {

    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let main_window = WindowDesc::new(build_ui())
        .title("RSA-Simulator")
        .window_size((800., 700.))
        .resizable(false);
    let initial_state = AppState::default();
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
