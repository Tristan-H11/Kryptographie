#[cfg(test)]
mod tests {
    use crate::rsa::math_functions::big_int_util::{decrement, divides, increment, is_even, is_one, is_uneven, is_zero, not_divides, random_in_range};
    use crate::rsa::math_functions::big_int_util::{
        divides, is_even, is_one, is_uneven, is_zero, not_divides,
    };
    use ibig::ubig;

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
}
