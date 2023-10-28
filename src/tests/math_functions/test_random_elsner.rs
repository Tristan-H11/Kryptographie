#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::random_elsner::RandomElsner;
    use bigdecimal::num_bigint::{BigInt, BigUint};
    use bigdecimal::{BigDecimal, One};

    #[test]
    fn test_random_elsner() {
        let a: BigUint = 1u32.into();
        let b: BigUint = 997u32.into();

        let mut random = RandomElsner {
            sqrt_m: BigDecimal::from(13u32).sqrt().unwrap(),
            n: 0u32.into(),
            a: a.clone(),
            range: BigDecimal::from(BigInt::from(b - a + BigUint::one())),
        };

        assert_eq!(random.take(), 604u32.into());
        assert_eq!(random.take(), 211u32.into());
        assert_eq!(random.take(), 815u32.into());
        assert_eq!(random.take(), 421u32.into());
        assert_eq!(random.take(), 28u32.into());
        assert_eq!(random.take(), 632u32.into());
        assert_eq!(random.take(), 239u32.into());
        assert_eq!(random.take(), 842u32.into());
        assert_eq!(random.take(), 449u32.into());
        assert_eq!(random.take(), 56u32.into());

        let a: BigUint = 500u32.into();
        let b: BigUint = 6000u32.into();

        random = RandomElsner::new(&a, &b);

        for _ in 1..500 {
            let random = random.take();
            assert!(random >= a && random <= b);
        }
    }
}
