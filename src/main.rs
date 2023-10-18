mod encryption;
mod gui;
mod tests;

use druid::{AppLauncher, WindowDesc};
use gui::gui::build_ui;
use gui::model::model::AppState;
use crate::encryption::math_functions::big_int_util::elsner_rand;

fn main() {
    print!("{}",elsner_rand(1.0,100.0));
}
