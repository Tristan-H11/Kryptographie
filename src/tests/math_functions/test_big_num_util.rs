#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigUint;

    use crate::encryption::math_functions::big_num_util::log_base_g;

    #[test]
    fn test_log_base_g() {
        let x1 = BigUint::from(16u32);
        let base1 = BigUint::from(2u32);
        assert_eq!(log_base_g(&x1, &base1), 4);

        let x2 = BigUint::from(256u32);
        let base2 = BigUint::from(4u32);
        assert_eq!(log_base_g(&x2, &base2), 4);

        let x3 = BigUint::from(27u32);
        let base3 = BigUint::from(3u32);
        assert_eq!(log_base_g(&x3, &base3), 3);

        let x4 = BigUint::from(1000000u32);
        let base4 = BigUint::from(10u32);
        assert_eq!(log_base_g(&x4, &base4), 6);
    }
}
