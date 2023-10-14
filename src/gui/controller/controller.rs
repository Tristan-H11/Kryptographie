use druid::{Env, Event};
use crate::gui::model::model::{AppState, View};

pub struct Controller;

impl Controller {
    pub fn handle_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        match app_state.current_view {
            View::HauptMenu => self.handle_haupt_menu_event(event, app_state, _env),
            View::Alice => self.handle_alice_event(event, app_state, _env),
            View::Bob => self.handle_bob_event(event, app_state, _env),
        }
    }

    fn handle_haupt_menu_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        // todo Implementiere hier event logik für hauptmenü
    }

    fn handle_alice_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        // todo Implementiere hier event logik für alice
    }

    fn handle_bob_event(&mut self, event: &Event, app_state: &mut AppState, _env: &Env) {
        // todo Implementiere hier event logik für bob
    }
}
