use crate::encryption::encryption_types::{Decryptor, EncryptionScheme, Encryptor, Key};
use crate::math_core::number_theory::number_theory_service::NumberTheoryService;

use std::fmt::Debug;
use crate::math_core::number_theory_with_prng_service::NumberTheoryWithPrngService;

/// Ein asymmetrisches Verschlüsselungsschema.
pub trait AsymmetricEncryptionScheme: EncryptionScheme {}

/// Ein asymmetrischer Schlüssel für das asymmetrische Verschlüsselungsschema.
pub trait AsymmetricKey<T: AsymmetricEncryptionScheme>: Key<T> {}

/// Ein öffentlicher Schlüssel für das asymmetrische Verschlüsselungsschema.
pub trait PublicKey<T: AsymmetricEncryptionScheme>: AsymmetricKey<T> {}

/// Ein privater Schlüssel für das asymmetrische Verschlüsselungsschema.
pub trait PrivateKey<T: AsymmetricEncryptionScheme>: AsymmetricKey<T> {}

/// Ein Schlüssel zum Entschlüsseln für das asymmetrische Verschlüsselungsschema.
pub trait AsymmetricDecryptionKey<T: AsymmetricEncryptionScheme>: PrivateKey<T> {}

/// Ein Schlüssel zum Verschlüsseln für das asymmetrische Verschlüsselungsschema.
pub trait AsymmetricEncryptionKey<T: AsymmetricEncryptionScheme>: PublicKey<T> {}

/// Ein Schlüssel zum Signieren für das asymmetrische Verschlüsselungsschema.
pub trait SignatureKey<T: AsymmetricEncryptionScheme>: PrivateKey<T> {}

/// Ein Schlüssel zum Verifizieren für das asymmetrische Verschlüsselungsschema.
pub trait VerificationKey<T: AsymmetricEncryptionScheme>: PublicKey<T> {}

/// Ein Schlüsselpaar für das asymmetrische Verschlüsselungsschema.
///
/// # Typen
/// * `Public` - Der Typ des öffentlichen Schlüssels.
/// * `Private` - Der Typ des privaten Schlüssels.
/// * `Scheme` - Das asymmetrische Verschlüsselungsschema.
///
/// # Methoden
/// * `public` - Gibt den öffentlichen Schlüssel zurück.
/// * `private` - Gibt den privaten Schlüssel zurück.
pub trait AsymmetricKeyPair<Public, Private, Scheme>
where
    Public: AsymmetricEncryptionKey<Scheme>,
    Private: AsymmetricDecryptionKey<Scheme>,
    Scheme: AsymmetricEncryptionScheme,
{
    fn public(&self) -> Public;
    fn private(&self) -> Private;
}

/// Ein Schlüsselgenerator für das asymmetrische Verschlüsselungsschema.
///
/// # Typen
/// * `Public` - Der Typ des öffentlichen Schlüssels.
/// * `Private` - Der Typ des privaten Schlüssels.
/// * `Scheme` - Das asymmetrische Verschlüsselungsschema.
///
/// # Methoden
/// * `generate_keypair` - Generiert ein Schlüsselpaar für das asymmetrische Verschlüsselungsschema.
pub trait KeyGenerator<Public, Private, Scheme>
where
    Public: AsymmetricEncryptionKey<Scheme>,
    Private: AsymmetricDecryptionKey<Scheme>,
    Scheme: AsymmetricEncryptionScheme,
{
    type KeyPair: AsymmetricKeyPair<Public, Private, Scheme>;
    /// Generiert ein Schlüsselpaar für das asymmetrische Verschlüsselungsschema.
    ///
    /// # Argumente
    /// * `config` - Die Konfiguration für den Schlüsselgenerierungsvorgang.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    fn generate_keypair(config: &impl KeyGenWithPrimeConfig) -> Self::KeyPair;
}

/// Die Konfiguration für die Schlüsselgenerierung für ein Verschlüsselungsschema, welches Primzahlen verwendet.
pub trait KeyGenWithPrimeConfig: Debug {
    /// Typischerweise die Größe des Schlüssels oder des Moduls in Bits.
    fn characteristic(&self) -> u32;
    /// Die Anzahl der Iterationen für den Miller-Rabin-Test bei der Generierung von Primzahlen.
    fn miller_rabin_iterations(&self) -> u32;
    /// Der Seed für die gleichverteilte Zufallszahlerzeugung.
    fn random_seed(&self) -> u32;
    /// Der Service für die Zahlentheorie.
    fn number_theory_service(&self) -> NumberTheoryService;
}

/// Ein Verschlüsseler für das asymmetrische Verschlüsselungsschema.
pub trait AsymmetricEncryptor<T: AsymmetricEncryptionScheme>: Encryptor<T> {
    /// Verschlüsselt den gegebenen Klartext mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `key` - Der Schlüssel zum Verschlüsseln.
    /// * `plaintext` - Der Klartext, der verschlüsselt werden soll.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// Der verschlüsselte Chiffretext.
    fn encrypt(
        key: &Self::Key,
        plaintext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output;
}

/// Ein Entschlüsseler für das asymmetrische Verschlüsselungsschema.
pub trait AsymmetricDecryptor<T: AsymmetricEncryptionScheme>: Decryptor<T> {
    /// Entschlüsselt den gegebenen Chiffretext mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `key` - Der Schlüssel zum Entschlüsseln.
    /// * `ciphertext` - Der Chiffretext, der entschlüsselt werden soll.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// Der entschlüsselte Klartext.
    fn decrypt(
        key: &Self::Key,
        ciphertext: &Self::Input,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output;
}

/// Ein Signierer für das asymmetrische Verschlüsselungsschema.
pub trait Signer<T: AsymmetricEncryptionScheme> {
    type Input: ?Sized;
    type Output;
    type Key: SignatureKey<T>;
    /// Signiert die gegebene Nachricht mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `key` - Der Schlüssel zum Signieren.
    /// * `message` - Die Nachricht, die signiert werden soll.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// Die Signatur der Nachricht.
    fn sign(key: &Self::Key, message: &Self::Input, service: &NumberTheoryWithPrngService) -> Self::Output;
}

/// Ein Verifizierer für das asymmetrische Verschlüsselungsschema.
pub trait Verifier<T: AsymmetricEncryptionScheme> {
    type Signature: ?Sized;
    type Message: ?Sized;
    type Output;
    type Key: VerificationKey<T>;
    /// Überprüft die gegebene Signatur für die gegebene Nachricht mit dem gegebenen Schlüssel.
    ///
    /// # Argumente
    /// * `key` - Der Schlüssel zum Überprüfen.
    /// * `signature` - Die Signatur, die überprüft werden soll.
    /// * `message` - Die Nachricht, die überprüft werden soll.
    /// * `service` - Der Service für die Zahlentheorie.
    ///
    /// # Rückgabe
    /// `true`, wenn die Signatur korrekt ist, ansonsten `false`.
    fn verify(
        key: &Self::Key,
        signature: &Self::Signature,
        message: &Self::Message,
        service: &NumberTheoryWithPrngService,
    ) -> Self::Output;
}
