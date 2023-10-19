mod encryption;
mod gui;
mod tests;

use crate::encryption::math_functions::big_int_util::elsner_rand;
use druid::{AppLauncher, WindowDesc};
use gui::gui::build_ui;
use gui::model::model::AppState;

fn main() {
    for n in 1..100 {
        println!("{}", elsner_rand(1.0, 10000.0));
    }
}
