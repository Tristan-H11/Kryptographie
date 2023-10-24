#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::big_int_util::{
        char_to_u16, decrement, divides, elsner_rand, elsner_rand, increment, is_even, is_one,
        is_uneven, is_zero, not_divides, random_in_range, u16_to_char, ubig_to_u16,
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

    #[test]
    fn test_random_number_in_range() {
        let high_num = BigUint::from(3453u32).pow(564);
        let random = random_in_range(&high_num);
        assert!(random <= high_num && random >= BigUint::from(2u32))
    }

    #[test]
    fn test_elsner_rand() {
        let a = 1000.0;
        let b = 10000.0;
        let random = elsner_rand(a, b);
        assert!(random >= a && random <= b)
    }

    #[test]
    fn test_char_to_u32() {
        assert_eq!(char_to_u16('a'), 0);
        assert_eq!(char_to_u16('b'), 1);
        assert_eq!(char_to_u16('z'), 25);
        assert_eq!(char_to_u16('A'), 26);
        assert_eq!(char_to_u16('B'), 27);
        assert_eq!(char_to_u16('Z'), 51);
        assert_eq!(char_to_u16('0'), 52);
        assert_eq!(char_to_u16('1'), 53);
        assert_eq!(char_to_u16('9'), 61);
    }
    #[test]
    #[should_panic(expected = "Ungültiges Zeichen: ß")]
    fn test_char_to_u32_invalid() {
        char_to_u16('ß');
    }

    #[test]
    fn test_u32_to_char() {
        assert_eq!(u16_to_char(0), 'a');
        assert_eq!(u16_to_char(25), 'z');
        assert_eq!(u16_to_char(26), 'A');
        assert_eq!(u16_to_char(51), 'Z');
        assert_eq!(u16_to_char(52), '0');
        assert_eq!(u16_to_char(61), '9');
        assert_eq!(u16_to_char(62), '.');
        assert_eq!(u16_to_char(63), ',');
    }

    #[test]
    fn test_ubig_to_u32() {
        let value = BigUint::from(12345u64);
        let result = ubig_to_u16(&value);
        assert_eq!(result, 12345);
    }
}
