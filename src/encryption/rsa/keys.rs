use bigdecimal::num_bigint::BigInt;
use crate::encryption::asymmetric_key_type::AsymmetricKeyType;


/// Ein RSA-Schlüssel.
#[derive(Clone, Debug)]
pub enum RsaKey {
    PublicKey(PublicKey),
    PrivateKey(PrivateKey),
}

#[derive(Clone, Debug)]
pub struct PublicKey {
    pub e: BigInt,
    pub n: BigInt,
}

#[derive(Clone, Debug)]
pub struct PrivateKey {
    pub d: BigInt,
    pub n: BigInt,
}

impl RsaKey {
    /// Erzeugt einen neuen RSA-Schlüssel.
    pub fn new(key_type: AsymmetricKeyType, exponent: BigInt, modulus: BigInt) -> Self {
        match key_type {
            AsymmetricKeyType::Public => RsaKey::PublicKey(PublicKey {
                e: exponent,
                n: modulus,
            }),
            AsymmetricKeyType::Private => RsaKey::PrivateKey(PrivateKey {
                d: exponent,
                n: modulus,
            }),
        }
    }

    /// Gibt den Exponenten des Schlüssels zurück.
    pub fn exponent(&self) -> &BigInt {
        match self {
            RsaKey::PublicKey(key) => &key.e,
            RsaKey::PrivateKey(key) => &key.d,
        }
    }

    /// Gibt das Modulus des Schlüssels zurück.
    pub fn modulus(&self) -> &BigInt {
        match self {
            RsaKey::PublicKey(key) => &key.n,
            RsaKey::PrivateKey(key) => &key.n,
        }
    }

    /// Gibt den Typ des Schlüssels zurück.
    pub fn key_type(&self) -> AsymmetricKeyType {
        match self {
            RsaKey::PublicKey(_) => AsymmetricKeyType::Public,
            RsaKey::PrivateKey(_) => AsymmetricKeyType::Private,
        }
    }
}
