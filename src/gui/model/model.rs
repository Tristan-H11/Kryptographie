use druid::{Data, Lens};
use std::default::Default;

// Datenmodelle für Hauptmenü, Alice und Bob
#[derive(Clone, Data, Lens)]
pub struct HauptMenuModel {
    pub(crate) eingabe_p1: String,
    pub(crate) eingabe_p2: String,
    pub(crate) eingabe_miller_rabin: String,
    pub(crate) ausgabe_oeff_schluessel: String,
}

#[derive(Clone, Data, Lens)]
pub struct AliceModel {
    pub(crate) eingabe_klartext: String,
    pub(crate) anzeige_signatur: String,
    pub(crate) status_signatur: bool,
    pub(crate) anzeige_geheimer_schluessel: String,
}

#[derive(Clone, Data, Lens)]
pub struct BobModel {
    pub(crate) eingabe_klartext: String,
    pub(crate) anzeige_signatur: String,
    pub(crate) status_signatur: bool,
    pub(crate) anzeige_geheimer_schluessel: String,
}

// Datenmodell für die gesamte App
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_view: View,
    pub haupt_menu: HauptMenuModel,
    pub alice: AliceModel,
    pub bob: BobModel,
}

// Enum für verschiedene Ansichten
#[derive(Clone, Data, PartialEq)]
pub enum View {
    HauptMenu,
    Alice,
    Bob,
}

enum CustomCommand {
    SwitchToAlice,
    SwitchToBob,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_view: View::HauptMenu, // Legen Sie den standardmäßigen aktuellen View fest
            haupt_menu: HauptMenuModel {
                eingabe_p1: String::new(),
                eingabe_p2: String::new(),
                eingabe_miller_rabin: String::new(),
                ausgabe_oeff_schluessel: String::new(),
            },
            alice: AliceModel {
                eingabe_klartext: String::new(),
                anzeige_signatur: String::new(),
                status_signatur: false,
                anzeige_geheimer_schluessel: String::new(),
            },
            bob: BobModel {
                eingabe_klartext: String::new(),
                anzeige_signatur: String::new(),
                status_signatur: false,
                anzeige_geheimer_schluessel: String::new(),
            },
        }
    }
}