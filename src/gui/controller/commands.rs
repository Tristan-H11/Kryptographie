use druid::Selector;

pub const SWITCH_TO_ALICE: Selector = Selector::new("switch-to-alice");
pub const SWITCH_TO_BOB: Selector = Selector::new("switch-to-bob");
pub const SWITCH_TO_HAUPTMENU: Selector = Selector::new("switch-to-hauptmenu");
pub const UPDATE_PUBLIC_KEY: Selector<String> = Selector::new("update-public-key");
pub const CALCULATE_PUBLIC_KEY: Selector = Selector::new("calculate-public-key");
pub const ENCRYPT: Selector = Selector::new("encrypt");
pub const SIGN: Selector = Selector::new("sign");
pub const DECRYPT: Selector = Selector::new("decrypt");
pub const SEND_MESSAGE: Selector = Selector::new("send-message");
pub const CLEAR: Selector = Selector::new("clear");