mod encryption;
mod gui;
mod tests;

use druid::{AppLauncher, Color, FontDescriptor, theme, WindowDesc};
use gui::gui::build_ui;
use gui::model::model::AppState;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use crate::gui::gui::start_gui;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    start_gui()
}
