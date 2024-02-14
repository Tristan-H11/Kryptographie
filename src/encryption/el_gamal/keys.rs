use bigdecimal::num_bigint::BigInt;

///
/// Ein öffentlicher ElGamal-Schlüssel für das ElGamal-Kryptosystem in primen Restklassengruppen.
/// Besteht aus dem Modulus p, dem Generator g und dem öffentlichen Wert y.
///
#[derive(Clone, Debug)]
pub struct PublicKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

///
/// Ein privater ElGamal-Schlüssel für das ElGamal-Kryptosystem in primen Restklassengruppen.
/// Besteht aus dem Modulus p und dem Geheimwert x.
#[derive(Clone, Debug)]
pub struct PrivateKey {
    pub p: BigInt,
    pub x: BigInt,
}
