use druid::{Data, Lens};
use std::default::Default;

// Datenmodelle für Hauptmenü, Alice und Bob
#[derive(Clone, Data, Lens)]
pub struct HauptMenuModel {
    pub(crate) prime_number_one: String,
    pub(crate) prime_number_two: String,
    pub(crate) miller_rabin_iterations: String,
    pub(crate) public_key_alice: String,
    pub(crate) public_key_bob: String,
}

#[derive(Clone, Data, Lens)]
pub struct AliceModel {
    pub(crate) message_klartext: String,
    pub(crate) message_chiffre: String,
    pub(crate) signature: String,
    pub(crate) signature_status: bool,
    pub(crate) private_key: String,
}

#[derive(Clone, Data, Lens)]
pub struct BobModel {
    pub(crate) message_klartext: String,
    pub(crate) message_chiffre: String,
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
            current_view: View::HauptMenu,
            haupt_menu: HauptMenuModel {
                prime_number_one: String::new(),
                prime_number_two: String::new(),
                miller_rabin_iterations: String::new(),
                public_key_alice: String::new(),
                public_key_bob: String::new(),
            },
            alice: AliceModel {
                message_klartext: String::new(),
                message_chiffre: String::new(),
                signature: String::new(),
                signature_status: false,
                private_key: String::new(),
            },
            bob: BobModel {
                message_klartext: String::new(),
                message_chiffre: String::new(),
                signature: String::new(),
                signature_status: false,
                private_key: String::new(),
            },
        }
    }
}
