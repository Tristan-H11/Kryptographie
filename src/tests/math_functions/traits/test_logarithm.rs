#[cfg(test)]
mod tests {
    use crate::big_i;
    use crate::encryption::math_functions::traits::logarithm::Logarithm;
    use bigdecimal::num_bigint::BigInt;

    #[test]
    fn test_log_base_g() {
        let x1 = big_i!(16);
        let base1 = big_i!(2);
        assert_eq!(x1.log(&base1), 4);

        let x2 = big_i!(6);
        let base2 = big_i!(4);
        assert_eq!(x2.log(&base2), 1);

        let x3 = big_i!(7);
        let base3 = big_i!(3);
        assert_eq!(x3.log(&base3), 1);
    }
}
