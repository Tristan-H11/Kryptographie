#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigInt;
    use crate::big_i;
    use crate::encryption::math_functions::big_num_util::log_base_g;

    #[test]
    fn test_log_base_g() {
        let x1 = big_i!(16);
        let base1 = big_i!(2);
        assert_eq!(log_base_g(&x1, &base1), 4);

        let x2 = big_i!(6);
        let base2 = big_i!(4);
        assert_eq!(log_base_g(&x2, &base2), 4);

        let x3 = big_i!(7);
        let base3 = big_i!(3);
        assert_eq!(log_base_g(&x3, &base3), 3);

        let x4 = big_i!(0);
        let base4 = big_i!(0);
        assert_eq!(log_base_g(&x4, &base4), 6);
    }
}
