#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use crate::encryption::math_functions::random_elsner::RandomElsner;
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_random_elsner() {
        let mut random = RandomElsner {
            sqrt_m: BigDecimal::from(13u32).sqrt().unwrap(),
            n: 0u32.into(),
        };

        let a: BigUint = 1u32.into();
        let b: BigUint = 997u32.into();

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
    }
}