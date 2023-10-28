#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::traits::parity::Parity;
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_is_even() {
        let uint = BigUint::from(34563u32);
        assert_eq!(uint.is_even(), false);
        assert_eq!(BigUint::from(8564u32).is_even(), true);
    }

    #[test]
    fn test_is_uneven() {
        assert_eq!(BigUint::from(24390u32).is_odd(), false);
        assert_eq!(BigUint::from(1435099u32).is_odd(), true);
    }
}
