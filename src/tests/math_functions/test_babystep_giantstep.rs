#[cfg(test)]
mod tests {
    use crate::big_i;
    use crate::encryption::math_functions::babystep_giantstep::{log_naiv, shanks};
    use bigdecimal::num_bigint::BigInt;
    use std::time::SystemTime;

    #[test]
    fn shanks_test() {
        let shanks_now = SystemTime::now();
        assert_eq!(
            shanks(&big_i!(8), &big_i!(555), &big_i!(677)).unwrap(),
            big_i!(134)
        );
        assert_eq!(
            shanks(&big_i!(11), &big_i!(3), &big_i!(29)).unwrap(),
            big_i!(17)
        );
        assert_eq!(
            shanks(&big_i!(10), &big_i!(25), &big_i!(97)).unwrap(),
            big_i!(22)
        );
        assert_eq!(
            shanks(&big_i!(3), &big_i!(4), &big_i!(7)).unwrap(),
            big_i!(4)
        );
        assert!(shanks(&big_i!(4), &big_i!(6), &big_i!(7)).is_err()); //Da Base nicht primitive Wurzel!
        println!("Shanks-Tests in {:?}", shanks_now.elapsed().unwrap());
    }

    #[test]
    fn log_naiv_test() {
        let naiv_now = SystemTime::now();
        assert_eq!(
            log_naiv(&big_i!(8), &big_i!(555), &big_i!(677)).unwrap(),
            big_i!(134)
        );
        assert_eq!(
            log_naiv(&big_i!(11), &big_i!(3), &big_i!(29)).unwrap(),
            big_i!(17)
        );
        assert_eq!(
            log_naiv(&big_i!(10), &big_i!(25), &big_i!(97)).unwrap(),
            big_i!(22)
        );
        assert_eq!(
            log_naiv(&big_i!(3), &big_i!(4), &big_i!(7)).unwrap(),
            big_i!(4)
        );
        assert!(log_naiv(&big_i!(4), &big_i!(6), &big_i!(7)).is_err()); //Da Base nicht primitive Wurzel!
        println!("Naiv-Tests in {:?}", naiv_now.elapsed().unwrap());
    }
}
