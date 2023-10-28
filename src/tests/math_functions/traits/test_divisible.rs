#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::traits::divisible::Divisible;
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_divides() {
        assert_eq!(
            BigUint::from(8u32).is_divisible_by(&BigUint::from(4u32)),
            true
        );
        assert_eq!(
            BigUint::from(89893457u32).is_divisible_by(&BigUint::from(1u32)),
            true
        );
        assert_eq!(
            BigUint::from(89893457u32).is_divisible_by(&BigUint::from(657831u32)),
            false
        );
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(
            BigUint::from(8u32).is_not_divisible_by(&BigUint::from(4u32)),
            false
        );
        assert_eq!(
            BigUint::from(89893457u32).is_not_divisible_by(&BigUint::from(1u32)),
            false
        );
        assert_eq!(
            BigUint::from(89893457u32).is_not_divisible_by(&BigUint::from(657831u32)),
            true
        );
    }

    #[test]
    fn test_half() {
        assert_eq!(BigUint::from(8u32).half(), BigUint::from(4u32));
        assert_eq!(BigUint::from(1u32).half(), BigUint::from(0u32));
    }

    #[test]
    fn test_half_assign() {
        let mut uint = BigUint::from(8u32);
        uint.half_assign();
        assert_eq!(uint, BigUint::from(4u32));

        let mut uint = BigUint::from(1u32);
        uint.half_assign();
        assert_eq!(uint, BigUint::from(0u32));
    }

    #[test]
    fn test_double() {
        assert_eq!(BigUint::from(8u32).double(), BigUint::from(16u32));
        assert_eq!(BigUint::from(1u32).double(), BigUint::from(2u32));
    }

    #[test]
    fn test_double_assign() {
        let mut uint = BigUint::from(8u32);
        uint.double_assign();
        assert_eq!(uint, BigUint::from(16u32));

        let mut uint = BigUint::from(1u32);
        uint.double_assign();
        assert_eq!(uint, BigUint::from(2u32));
    }
}
