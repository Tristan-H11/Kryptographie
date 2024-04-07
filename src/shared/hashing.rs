use bigdecimal::num_bigint::{BigInt, Sign};
use log::debug;
use sha2::{Digest, Sha256};

/// Diese Methode berechnet den SHA256-Hash einer Nachricht.
///
/// # Argumente
/// * `message` - Die Nachricht.
///
/// # Rückgabe
/// * `BigInt` - Der Hash.
pub fn sha256(message: &str) -> BigInt {
    debug!("Hashen der Nachricht {} mit SHA256", message);
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    // Hash Nachricht in einen BigInt umwandeln
    let message_big_int = BigInt::from_bytes_be(Sign::Plus, &hashed_message);
    message_big_int
}