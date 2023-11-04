use bigdecimal::num_bigint::{BigInt, Sign};
use log::{debug, info};
use sha2::{Digest, Sha256};

use crate::encryption::math_functions::block_chiffre::{
    create_blocks_from_string_decrypt, create_blocks_from_string_encrypt,
    create_string_from_blocks_decrypt, create_string_from_blocks_encrypt,
};
use crate::encryption::math_functions::number_theory::fast_exponentiation;
use crate::encryption::math_functions::traits::logarithm::Logarithm;

///
/// Ein öffentlicher Schlüssel für RSA.
///
pub struct PublicKey {
    e: BigInt,
    n: BigInt,
    block_size: usize,
}

impl PublicKey {
    ///
    /// Erstellt eine neue Instanz des PublicKey.
    ///
    /// # Argumente
    ///
    /// * `e` - Der öffentliche Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    ///
    pub fn new(e: BigInt, n: BigInt, g_base: &BigInt) -> PublicKey {
        // Maximale Blockbreite = log_g(n).
        let block_size = n.log(g_base);
        debug!("Blocksize in der PublicKey-Erstellung: {}", block_size);
        PublicKey { e, n, block_size }
    }

    ///
    /// Gibt den öffentlichen Exponenten als String zurück.
    ///
    pub fn get_e_as_str(&self) -> String {
        self.e.to_str_radix(10)
    }

    ///
    /// Gibt den Modul als String zurück.
    ///
    pub fn get_n_as_str(&self) -> String {
        self.n.to_str_radix(10)
    }

    #[cfg(test)]
    pub fn get_e(&self) -> BigInt {
        self.e.clone()
    }

    #[cfg(test)]
    pub fn get_n(&self) -> BigInt {
        self.n.clone()
    }

    #[cfg(test)]
    pub fn get_block_size(&self) -> usize {
        self.block_size.clone()
    }

    ///
    /// Verschlüsselt eine Nachricht mit dem öffentlichen Schlüssel.
    ///
    /// # Argumente
    ///
    /// * `message` - Die zu verschlüsselnde Nachricht. **ACHTUNG**: Leerzeichen am Ende werden entfernt!
    /// * `base_length` - Die Basis, in der die Nachricht verschlüsselt werden soll.
    ///
    /// # Rückgabe
    ///
    /// * `String` - Die verschlüsselte Nachricht.
    ///
    pub(crate) fn encrypt(&self, message: &str, g_base: &BigInt) -> String {
        info!("Verschlüsseln mit blockgröße {}", self.block_size);

        let chunks =
            create_blocks_from_string_encrypt(message.trim_end(), self.block_size, true, g_base);
        let encrypted_chunks = chunks
            .iter()
            .map(|chunk| fast_exponentiation(chunk, &self.e, &self.n))
            .collect();

        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        create_string_from_blocks_encrypt(encrypted_chunks, self.block_size + 1, g_base)
    }

    pub(crate) fn verify(&self, _signature: &str, _message: &str) -> bool {
        // Nicht Signierte aber optional verschlüsselte Nachricht hashen
        let mut hasher = Sha256::new();
        hasher.update(_message.as_bytes());
        let hashed_message = hasher.finalize();

        // Hash in einen BigInt umwandeln
        let message_big_int = BigInt::from_bytes_be(Sign::Plus, &hashed_message);

        // Signatur vom Partner in BigInt umwandeln
        let signature_big_int = BigInt::parse_bytes(_signature.as_bytes(), 10)
            .expect("Die Signatur konnte nicht in einen BigInt umgewandelt werden");

        // Verifizierung durchführen: verifizierung = signatur ^ (öffentlicher key vom partner) mod n
        let verification = fast_exponentiation(&signature_big_int, &self.e, &self.n);

        // Überprüfen, ob die Verifizierung mit der gehashten Nachricht übereinstimmt
        verification == message_big_int
    }
}

///
/// Ein privater Schlüssel für RSA.
///
pub struct PrivateKey {
    d: BigInt,
    n: BigInt,
    block_size: usize,
}

impl PrivateKey {
    ///
    /// Erstellt eine neue Instanz des PrivateKey.
    ///
    /// # Argumente
    ///
    /// * `d` - Der private Exponent.
    /// * `n` - Das Produkt der beiden Primzahlen.
    ///
    pub fn new(d: BigInt, n: BigInt, g_base: &BigInt) -> PrivateKey {
        // Die Größe der verschlüsselten Blöcke ist immer um 1 größer als die Klartextgröße.
        let block_size = n.log(g_base) + 1;
        debug!("Blocksize in der PrivateKey-Erstellung: {}", block_size);
        PrivateKey { d, n, block_size }
    }

    ///
    /// Gibt den privaten Exponenten als String zurück.
    ///
    pub fn get_d_as_str(&self) -> String {
        self.d.to_str_radix(10)
    }

    #[cfg(test)]
    pub fn get_d(&self) -> BigInt {
        self.d.clone()
    }

    #[cfg(test)]
    pub fn get_n(&self) -> BigInt {
        self.n.clone()
    }

    #[cfg(test)]
    pub fn get_block_size(&self) -> usize {
        self.block_size.clone()
    }

    ///
    /// Entschlüsselt eine Nachricht mit dem privaten Schlüssel.
    ///
    /// # Argumente
    ///
    /// * `message` - Die zu entschlüsselnde Nachricht.
    /// * `base_length` - Die Basis, in der die Nachricht verschlüsselt wurde.
    ///
    /// # Rückgabe
    ///
    /// * `String` - Die entschlüsselte Nachricht.
    ///
    pub(crate) fn decrypt(&self, message: &str, g_base: &BigInt) -> String {
        info!("Entschlüsseln mit blockgröße {}", self.block_size);

        let chunks = create_blocks_from_string_decrypt(message, true, g_base, self.block_size);
        let decrypted_chunks = chunks
            .iter()
            .map(|chunk| fast_exponentiation(chunk, &self.d, &self.n))
            .collect();

        create_string_from_blocks_decrypt(decrypted_chunks, g_base)
    }

    pub(crate) fn sign(&self, _message: &str) -> String {
        // Nachricht hashen
        let mut hasher = Sha256::new();
        hasher.update(_message.as_bytes());
        let hashed_message = hasher.finalize();

        // Hash Nachricht in einen BigInt umwandeln
        let message_big_int = BigInt::from_bytes_be(Sign::Plus, &hashed_message);

        // Signatur berechnen: signatur = message^(eigener privater key) mod n
        let signature = fast_exponentiation(&message_big_int, &self.d, &self.n);

        // Signatur als String zurückgeben
        signature.to_str_radix(10)
    }
}
