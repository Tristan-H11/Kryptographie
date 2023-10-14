

use druid::{Event, Env, Command, Selector};
use crate::gui::model::model::{AliceModel, AppModel, BobModel, CurrentView, HauptMenuModel};

pub const SWITCH_TO_ALICE: Selector = Selector::new("switch_to_alice");
pub const SWITCH_TO_BOB: Selector = Selector::new("switch_to_bob");
pub const SWITCH_TO_HAUPT_MENU: Selector = Selector::new("switch_to_haupt_menu");
pub const SOME_COMMAND: Selector = Selector::new("some_command");

fn haupt_menu_controller(event: &Event, data: &mut HauptMenuModel, _env: &Env) {
    match event {
        Event::Command(cmd) if cmd.is(SOME_COMMAND) => {
        }
        _ => {}
    }
}

fn alice_controller(event: &Event, data: &mut AliceModel, _env: &Env) {
}

fn bob_controller(event: &Event, data: &mut BobModel, _env: &Env) {
}

pub fn app_controller(event: &Event, data: &mut AppModel, _env: &Env) {
    match event {
        Event::Command(cmd) if cmd.is(SWITCH_TO_ALICE) => {
            data.current_view = CurrentView::Alice;
        }
        _ => (),
    }
}