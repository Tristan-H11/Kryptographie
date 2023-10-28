#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigUint;
    use crate::big_u;

    use crate::encryption::math_functions::big_num_util::log_base_g;

    #[test]
    fn test_log_base_g() {
        let x1 = big_u!(16u32);
        let base1 = big_u!(2u32);
        assert_eq!(log_base_g(&x1, &base1), 4);

        let x2 = big_u!(256u32);
        let base2 = big_u!(4u32);
        assert_eq!(log_base_g(&x2, &base2), 4);

        let x3 = big_u!(27u32);
        let base3 = big_u!(3u32);
        assert_eq!(log_base_g(&x3, &base3), 3);

        let x4 = big_u!(1000000u32);
        let base4 = big_u!(10u32);
        assert_eq!(log_base_g(&x4, &base4), 6);
    }
}
