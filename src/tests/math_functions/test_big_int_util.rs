#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::big_int_util::{
        decrement, divides, increment, is_even, is_one,
        is_uneven, is_zero, not_divides
    };
    use bigdecimal::num_bigint::BigUint;
    #[test]
    fn test_is_even() {
        assert_eq!(is_even(&BigUint::from(34563u32)), false);
        assert_eq!(is_even(&BigUint::from(8564u32)), true);
    }

    #[test]
    fn test_is_uneven() {
        assert_eq!(is_uneven(&BigUint::from(24390u32)), false);
        assert_eq!(is_uneven(&BigUint::from(1435099u32)), true);
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(&BigUint::from(0u32)), true);
        assert_eq!(is_zero(&BigUint::from(1u32)), false);
    }

    #[test]
    fn test_is_one() {
        assert_eq!(is_one(&BigUint::from(0u32)), false);
        assert_eq!(is_one(&BigUint::from(1u32)), true);
    }

    #[test]
    fn test_divides() {
        assert_eq!(divides(&BigUint::from(4u32), &BigUint::from(8u32)), true);
        assert_eq!(
            divides(&BigUint::from(1u32), &BigUint::from(89893457u32)),
            true
        );
        assert_eq!(
            divides(&BigUint::from(134505u32), &BigUint::from(89893457u32)),
            false
        );
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(
            not_divides(&BigUint::from(4u32), &BigUint::from(8u32)),
            false
        );
        assert_eq!(
            not_divides(&BigUint::from(1u32), &BigUint::from(89893457u32)),
            false
        );
        assert_eq!(
            not_divides(&BigUint::from(134505u32), &BigUint::from(89893457u32)),
            true
        );
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment(&BigUint::from(3u32)), BigUint::from(4u32));
        assert_eq!(increment(&BigUint::from(0u32)), BigUint::from(1u32));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(decrement(&BigUint::from(3u32)), BigUint::from(2u32));
        assert_eq!(decrement(&BigUint::from(1u32)), BigUint::from(0u32));
    }

}
