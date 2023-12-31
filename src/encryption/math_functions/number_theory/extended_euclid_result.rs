use bigdecimal::num_bigint::BigInt;

/// Das Ergebnis des erweiterten Euklidischen Algorithmus.
pub struct ExtendedEuclidResult {
    pub ggt: BigInt,
    pub x: BigInt,
    pub y: BigInt,
}

impl ExtendedEuclidResult {
    /// Erstellt eine neue Instanz des ExtendedEuclidResult.
    pub fn new(ggt: BigInt, x: BigInt, y: BigInt) -> ExtendedEuclidResult {
        ExtendedEuclidResult { ggt, x, y }
    }
}
