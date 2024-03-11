use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SingleStringResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct UseFastQuery {
    pub use_fast: bool,
}
