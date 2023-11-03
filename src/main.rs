mod encryption;
mod gui;
mod tests;

use crate::gui::gui::start_gui;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_colors(true)
        .init()
        .unwrap();

    start_gui();
}
