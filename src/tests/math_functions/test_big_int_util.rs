#[cfg(test)]
mod tests {
    use crate::rsa::math_functions::big_int_util::{divides, is_even, is_one, is_uneven, is_zero};
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
}
