#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::traits::divisible::Divisible;
    use bigdecimal::num_bigint::BigUint;
    use crate::big_u;

    #[test]
    fn test_divides() {
        assert_eq!(
            big_u!(8u32).is_divisible_by(&big_u!(4u32)),
            true
        );
        assert_eq!(
            big_u!(89893457u32).is_divisible_by(&big_u!(1u32)),
            true
        );
        assert_eq!(
            big_u!(89893457u32).is_divisible_by(&big_u!(657831u32)),
            false
        );
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(
            big_u!(8u32).is_not_divisible_by(&big_u!(4u32)),
            false
        );
        assert_eq!(
            big_u!(89893457u32).is_not_divisible_by(&big_u!(1u32)),
            false
        );
        assert_eq!(
            big_u!(89893457u32).is_not_divisible_by(&big_u!(657831u32)),
            true
        );
    }

    #[test]
    fn test_half() {
        assert_eq!(big_u!(8u32).half(), big_u!(4u32));
        assert_eq!(big_u!(1u32).half(), big_u!(0u32));
    }

    #[test]
    fn test_half_assign() {
        let mut uint = big_u!(8u32);
        uint.half_assign();
        assert_eq!(uint, big_u!(4u32));

        let mut uint = big_u!(1u32);
        uint.half_assign();
        assert_eq!(uint, big_u!(0u32));
    }

    #[test]
    fn test_double() {
        assert_eq!(big_u!(8u32).double(), big_u!(16u32));
        assert_eq!(big_u!(1u32).double(), big_u!(2u32));
    }

    #[test]
    fn test_double_assign() {
        let mut uint = big_u!(8u32);
        uint.double_assign();
        assert_eq!(uint, big_u!(16u32));

        let mut uint = big_u!(1u32);
        uint.double_assign();
        assert_eq!(uint, big_u!(2u32));
    }
}
