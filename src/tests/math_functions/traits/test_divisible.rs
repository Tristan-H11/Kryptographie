#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigUint;
    use crate::encryption::math_functions::traits::divisible::Divisible;

    #[test]
    fn test_divides() {
        assert_eq!(BigUint::from(8u32).is_divisible_by(&BigUint::from(4u32)), true);
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
        assert_eq!(BigUint::from(8u32).is_not_divisible_by(&BigUint::from(4u32)), false);
        assert_eq!(
            BigUint::from(89893457u32).is_not_divisible_by(&BigUint::from(1u32)),
            false
        );
        assert_eq!(
            BigUint::from(89893457u32).is_not_divisible_by(&BigUint::from(657831u32)),
            true
        );
    }
}