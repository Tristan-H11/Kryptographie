use bigdecimal::num_bigint::BigInt;
use log::debug;

use crate::encryption::math_functions::traits::logarithm::Logarithm;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    pub e: BigInt,
    pub n: BigInt,
    pub block_size: usize,
}

impl PublicKey {
    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    pub fn new(e: BigInt, n: BigInt, g_base: u32) -> PublicKey {
        // Maximale Blockbreite = log_g(n).
        let block_size = n.log(&g_base.into());
        debug!("Blocksize in der PublicKey-Erstellung: {}", block_size);
        PublicKey { e, n, block_size }
    }

    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `block_size` - Die Blockgröße.
    pub fn new_with_blocksize(e: BigInt, n: BigInt, block_size: usize) -> PublicKey {
        // Maximale Blockbreite = log_g(n).
        debug!("Blocksize in der PublicKey-Erstellung: {}", block_size);
        PublicKey { e, n, block_size }
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    pub d: BigInt,
    pub n: BigInt,
    pub block_size: usize,
}

impl PrivateKey {
    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `g_base` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    pub fn new(d: BigInt, n: BigInt, g_base: u32) -> PrivateKey {
        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        let block_size = n.log(&g_base.into()) + 1;
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }

    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    /// * `block_size` - Die Blockgröße.
    pub fn new_with_blocksize(d: BigInt, n: BigInt, block_size: usize) -> PrivateKey {
        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }
}

// TODO Tests (Konstruktor mit BlockSize bestimmung. Sonst keine Tests möglich, da keine Funktionen implementiert sind.)
