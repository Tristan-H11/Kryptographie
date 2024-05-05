use bigdecimal::num_bigint::BigInt;

/// Das Ergebnis des erweiterten Euklidischen Algorithmus.
#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_euclid_result_creation() {
        let ggt = BigInt::from(5);
        let x = BigInt::from(2);
        let y = BigInt::from(3);
        let result = ExtendedEuclidResult::new(ggt.clone(), x.clone(), y.clone());

        assert_eq!(result.ggt, ggt);
        assert_eq!(result.x, x);
        assert_eq!(result.y, y);
    }

    #[test]
    fn test_extended_euclid_result_equality() {
        let ggt = BigInt::from(5);
        let x = BigInt::from(2);
        let y = BigInt::from(3);
        let result1 = ExtendedEuclidResult::new(ggt.clone(), x.clone(), y.clone());
        let result2 = ExtendedEuclidResult::new(ggt.clone(), x.clone(), y.clone());

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_extended_euclid_result_inequality() {
        let ggt1 = BigInt::from(5);
        let x1 = BigInt::from(2);
        let y1 = BigInt::from(3);
        let ggt2 = BigInt::from(7);
        let x2 = BigInt::from(4);
        let y2 = BigInt::from(5);

        let result1 = ExtendedEuclidResult::new(ggt1.clone(), x1.clone(), y1.clone());
        let result2 = ExtendedEuclidResult::new(ggt2.clone(), x1.clone(), y1.clone());
        let result3 = ExtendedEuclidResult::new(ggt1.clone(), x2.clone(), y1.clone());
        let result4 = ExtendedEuclidResult::new(ggt1.clone(), x1.clone(), y2.clone());

        assert_ne!(result1, result2);
        assert_ne!(result1, result3);
        assert_ne!(result1, result4);
    }
}