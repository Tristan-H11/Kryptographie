#[cfg(test)]
mod tests {
    use crate::big_u;
    use crate::encryption::math_functions::traits::increment::Increment;
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_increment() {
        assert_eq!(big_u!(3u32).increment(), big_u!(4u32));
        assert_eq!(big_u!(0u32).increment(), big_u!(1u32));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(big_u!(3u32).decrement(), big_u!(2u32));
        assert_eq!(big_u!(1u32).decrement(), big_u!(0u32));
    }

    #[test]
    fn test_increment_assign() {
        let mut uint = big_u!(3u32);
        uint.increment_assign();
        assert_eq!(uint, big_u!(4u32));
    }

    #[test]
    fn test_decrement_assign() {
        let mut uint = big_u!(3u32);
        uint.decrement_assign();
        assert_eq!(uint, big_u!(2u32));
    }
}
