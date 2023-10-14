use druid::{Env, Lens, lens, Widget, WidgetExt};
use druid::widget::{Container, Flex, ViewSwitcher};
use crate::gui::view::view::build_haupt_menu;
use crate::gui::view::view::build_alice_view;
use crate::gui::view::view::build_bob_view;

use druid::Data;

#[derive(Clone, Data)]
pub enum CurrentView {
    HauptMenu,
    Alice,
    Bob,
}

#[derive(Clone, Data)]
pub struct HauptMenuModel {
    pub(crate) eingabe_p1: String,
    pub(crate) eingabe_p2: String,
    pub(crate) eingabe_miller_rabin: String,
    pub(crate) ausgabe_oeff_schluessel: String,
}

#[derive(Clone, Data)]
pub struct AliceModel {
    pub(crate) eingabe_klartext: String,
    pub(crate) anzeige_signatur: String,
    pub(crate) status_signatur: bool,
    pub(crate) anzeige_geheimer_schluessel: String,
}

#[derive(Clone, Data)]
pub struct BobModel {
    pub(crate) eingabe_klartext: String,
    pub(crate) anzeige_signatur: String,
    pub(crate) status_signatur: bool,
    pub(crate) anzeige_geheimer_schluessel: String,
}

#[derive(Clone, Data)]
pub struct AppModel {
    pub current_view: CurrentView,
    pub haupt_menu_model: HauptMenuModel,
    pub alice_model: AliceModel,
    pub bob_model: BobModel,
}

impl Lens<AppModel, HauptMenuModel> for AppModel {
    fn with<V, F: FnOnce(&HauptMenuModel, &AppModel) -> V>(&self, data: &AppModel, f: F) -> V {
        f(&self.haupt_menu_model, data)
    }

    fn with_mut<V, F: FnOnce(&mut HauptMenuModel, &mut AppModel) -> V>(&mut self, data: &mut AppModel, f: F) -> V {
        f(&mut self.haupt_menu_model, data)
    }
}

impl Lens<AppModel, AliceModel> for AppModel {
    fn with<V, F: FnOnce(&AliceModel, &AppModel) -> V>(&self, data: &AppModel, f: F) -> V {
        f(&self.alice_model, data)
    }

    fn with_mut<V, F: FnOnce(&mut AliceModel, &mut AppModel) -> V>(&mut self, data: &mut AppModel, f: F) -> V {
        f(&mut self.alice_model, data)
    }
}

impl Lens<AppModel, BobModel> for AppModel {
    fn with<V, F: FnOnce(&BobModel, &AppModel) -> V>(&self, data: &AppModel, f: F) -> V {
        f(&self.bob_model, data)
    }

    fn with_mut<V, F: FnOnce(&mut BobModel, &mut AppModel) -> V>(&mut self, data: &mut AppModel, f: F) -> V {
        f(&mut self.bob_model, data)
    }
}


pub fn build_view() -> impl Widget<AppModel> {
    Flex::column()
        .with_child(
            Container::new(
                ViewSwitcher::new(
                    |data: &AppModel, _env| data.current_view.clone(),
                    |selector, _data, _env| {
                        match selector {
                            CurrentView::HauptMenu => Box::new(build_haupt_menu().lens(AppModel::haupt_menu_model)),
                            CurrentView::Alice => Box::new(build_alice_view().lens(AppModel::alice_model)),
                            CurrentView::Bob => Box::new(build_bob_view().lens(AppModel::bob_model)),
                        }
                    }
                )
            )
        )
}

