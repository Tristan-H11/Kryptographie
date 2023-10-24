#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::big_int_util::{
        char_to_u32, decrement, divides, elsner_rand, increment, is_even, is_one, is_uneven,
        is_zero, not_divides, random_in_range, u32_to_char, ubig_to_u32,
    };
    use ibig::{ubig, UBig};

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(&ubig!(34563)), false);
        assert_eq!(is_even(&ubig!(8564)), true);
    }

    #[test]
    fn test_is_uneven() {
        assert_eq!(is_uneven(&ubig!(24390)), false);
        assert_eq!(is_uneven(&ubig!(1435099)), true);
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(&ubig!(0)), true);
        assert_eq!(is_zero(&ubig!(1)), false);
    }

    #[test]
    fn test_is_one() {
        assert_eq!(is_one(&ubig!(0)), false);
        assert_eq!(is_one(&ubig!(1)), true);
    }

    #[test]
    fn test_divides() {
        assert_eq!(divides(&ubig!(4), &ubig!(8)), true);
        assert_eq!(divides(&ubig!(1), &ubig!(89893457)), true);
        assert_eq!(divides(&ubig!(134505), &ubig!(89893457)), false);
    }

    #[test]
    fn test_not_divides() {
        assert_eq!(not_divides(&ubig!(4), &ubig!(8)), false);
        assert_eq!(not_divides(&ubig!(1), &ubig!(89893457)), false);
        assert_eq!(not_divides(&ubig!(134505), &ubig!(89893457)), true);
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment(&ubig!(3)), ubig!(4));
        assert_eq!(increment(&ubig!(0)), ubig!(1));
    }

    #[test]
    fn test_decrement() {
        assert_eq!(decrement(&ubig!(3)), ubig!(2));
        assert_eq!(decrement(&ubig!(1)), ubig!(0));
    }

    #[test]
    fn test_random_number_in_range() {
        let high_num = ubig!(3453).pow(564);
        let random = random_in_range(&high_num);
        assert!(random <= high_num && random >= ubig!(2))
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
        assert_eq!(char_to_u32('a'), 0);
        assert_eq!(char_to_u32('b'), 1);
        assert_eq!(char_to_u32('z'), 25);
        assert_eq!(char_to_u32('A'), 26);
        assert_eq!(char_to_u32('B'), 27);
        assert_eq!(char_to_u32('Z'), 51);
        assert_eq!(char_to_u32('0'), 52);
        assert_eq!(char_to_u32('1'), 53);
        assert_eq!(char_to_u32('9'), 61);
    }
    #[test]
    #[should_panic(expected = "Ungültiges Zeichen: ß")]
    fn test_char_to_u32_invalid() {
        char_to_u32('ß');
    }

    #[test]
    fn test_u32_to_char() {
        assert_eq!(u32_to_char(0), 'a');
        assert_eq!(u32_to_char(25), 'z');
        assert_eq!(u32_to_char(26), 'A');
        assert_eq!(u32_to_char(51), 'Z');
        assert_eq!(u32_to_char(52), '0');
        assert_eq!(u32_to_char(61), '9');
        assert_eq!(u32_to_char(62), '.');
        assert_eq!(u32_to_char(63), ',');
    }

    #[test]
    fn test_ubig_to_u32() {
        let value = UBig::from(12345_u64);
        let result = ubig_to_u32(&value);
        assert_eq!(result, 12345);
    }
}
