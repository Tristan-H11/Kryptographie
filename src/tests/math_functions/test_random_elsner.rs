#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::random_elsner::RandomElsner;
    use crate::encryption::math_functions::traits::divisible::Divisible;

    #[test]
    fn test_random_elsner() {
        let a: BigInt = 1u32.into();
        let b: BigInt = 997u32.into();

        let mut random = RandomElsner::new(&big_i!(13));

        assert_eq!(random.take(&a, &b), 604u32.into());
        assert_eq!(random.take(&a, &b), 211u32.into());
        assert_eq!(random.take(&a, &b), 815u32.into());
        assert_eq!(random.take(&a, &b), 421u32.into());
        assert_eq!(random.take(&a, &b), 28u32.into());
        assert_eq!(random.take(&a, &b), 632u32.into());
        assert_eq!(random.take(&a, &b), 239u32.into());
        assert_eq!(random.take(&a, &b), 842u32.into());
        assert_eq!(random.take(&a, &b), 449u32.into());
        assert_eq!(random.take(&a, &b), 56u32.into());

        let a: BigInt = 500u32.into();
        let b: BigInt = 6000u32.into();

        random = RandomElsner::new(&big_i!(40));

        for _ in 1..500 {
            let random = random.take(&a, &b);
            assert!(random >= a && random <= b);
        }
    }

    #[test]
    fn test_take_uneven() {
        let a: BigInt = 500u32.into();
        let b: BigInt = 6000u32.into();

        let mut random = RandomElsner::new(&big_i!(23));

        for _ in 1..500 {
            let random = random.take_uneven(&a, &b);
            assert!(random >= a && random <= b);
            assert!(random.is_not_divisible_by(&BigInt::from(2)));
        }
    }
}
