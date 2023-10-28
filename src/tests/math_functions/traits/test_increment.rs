#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::traits::increment::Increment;
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_increment() {
        assert_eq!(BigUint::from(3u32).increment(), BigUint::from(4u32));
        assert_eq!(BigUint::from(0u32).increment(), BigUint::from(1u32));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(BigUint::from(3u32).decrement(), BigUint::from(2u32));
        assert_eq!(BigUint::from(1u32).decrement(), BigUint::from(0u32));
    }

    #[test]
    fn test_increment_assign() {
        let mut uint = BigUint::from(3u32);
        uint.increment_assign();
        assert_eq!(uint, BigUint::from(4u32));
    }

    #[test]
    fn test_decrement_assign() {
        let mut uint = BigUint::from(3u32);
        uint.decrement_assign();
        assert_eq!(uint, BigUint::from(2u32));
    }
}
