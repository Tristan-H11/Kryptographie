#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::traits::parity::Parity;

    #[test]
    fn test_is_even() {
        let big_int: BigInt = big_i!(34563u32);
        assert_eq!(big_int.is_even(), false);
        assert_eq!(big_i!(8564u32).is_even(), true);
    }

    #[test]
    fn test_is_uneven() {
        assert_eq!(big_i!(24390u32).is_odd(), false);
        assert_eq!(big_i!(1435099u32).is_odd(), true);
    }
}
