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
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_block_size_1024_55296() {
        let n = BigInt::from(2).pow(1024);
        let g_base = 55296;
        let public_key = PublicKey::new(BigInt::from(3), n.clone(), g_base);
        assert_eq!(public_key.block_size, 64);

        let private_key = PrivateKey::new(BigInt::from(3), n, g_base);
        assert_eq!(private_key.block_size, 65);
    }

    #[test]
    fn test_block_size_2048_55296() {
        let n = BigInt::from(2).pow(2048);
        let g_base = 55296;
        let public_key = PublicKey::new(BigInt::from(3), n.clone(), g_base);
        assert_eq!(public_key.block_size, 129);

        let private_key = PrivateKey::new(BigInt::from(3), n, g_base);
        assert_eq!(private_key.block_size, 130);
    }

    #[test]
    fn test_block_size_128_55296() {
        let n = BigInt::from(2).pow(128);
        let g_base = 55296;
        let public_key = PublicKey::new(BigInt::from(3), n.clone(), g_base);
        assert_eq!(public_key.block_size, 8);

        let private_key = PrivateKey::new(BigInt::from(3), n, g_base);
        assert_eq!(private_key.block_size, 9);
    }

    #[test]
    fn test_block_size_512_55296() {
        let n = BigInt::from(2).pow(512);
        let g_base = 55296;
        let public_key = PublicKey::new(BigInt::from(3), n.clone(), g_base);
        assert_eq!(public_key.block_size, 32);

        let private_key = PrivateKey::new(BigInt::from(3), n, g_base);
        assert_eq!(private_key.block_size, 33);
    }
}
