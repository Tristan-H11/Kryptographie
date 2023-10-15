use druid::{Data, Lens};
use std::default::Default;

// Datenmodelle für Hauptmenü, Alice und Bob
#[derive(Clone, Data, Lens)]
pub struct HauptMenuModel {
    pub(crate) prime_number_one: String,
    pub(crate) prime_number_two: String,
    pub(crate) miller_rabin_iterations: String,
    pub(crate) ausgabe_oeff_schluessel: String, // TODO @Lucas: Braucht man das??
    pub(crate) public_key_alice: String,
    pub(crate) public_key_bob: String,
}

#[derive(Clone, Data, Lens)]
pub struct AliceModel {
    pub(crate) message: String,
    pub(crate) signature: String,
    pub(crate) signature_status: bool,
    pub(crate) private_key: String,
}

#[derive(Clone, Data, Lens)]
pub struct BobModel {
    pub(crate) message: String,
    pub(crate) signature: String,
    pub(crate) signature_status: bool,
    pub(crate) private_key: String,
}

// Datenmodell für die gesamte App -- quasi die ebene über den einzelnen Datenmodellen
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

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_view: View::HauptMenu, // Festlegen, dass Hauptmenü die start-view ist
            haupt_menu: HauptMenuModel {
                prime_number_one: String::new(),
                prime_number_two: String::new(),
                miller_rabin_iterations: String::new(),
                ausgabe_oeff_schluessel: String::new(),
                public_key_alice: "".to_string(),
                public_key_bob: "".to_string(),
            },
            alice: AliceModel {
                message: String::new(),
                signature: String::new(),
                signature_status: false,
                private_key: String::new(),
            },
            bob: BobModel {
                message: String::new(),
                signature: String::new(),
                signature_status: false,
                private_key: String::new(),
            },
        }
    }
}
