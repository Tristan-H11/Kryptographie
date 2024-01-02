use bigdecimal::num_bigint::BigInt;

/// Typ eines RSA-Schlüssels.
#[derive(PartialEq)]
pub enum RsaKeyType {
    Public,
    Private,
}

/// Ein RSA-Schlüssel.
#[derive(Clone, Debug)]
pub enum RsaKey {
    PublicKey(PublicKey),
    PrivateKey(PrivateKey),
}

impl RsaKey {
    /// Erzeugt einen neuen RSA-Schlüssel.
    pub fn new(key_type: RsaKeyType, exponent: BigInt, modulus: BigInt) -> Self {
        match key_type {
            RsaKeyType::Public => RsaKey::PublicKey(PublicKey {
                e: exponent,
                n: modulus,
            }),
            RsaKeyType::Private => RsaKey::PrivateKey(PrivateKey {
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
    pub fn key_type(&self) -> RsaKeyType {
        match self {
            RsaKey::PublicKey(_) => RsaKeyType::Public,
            RsaKey::PrivateKey(_) => RsaKeyType::Private,
        }
    }
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
