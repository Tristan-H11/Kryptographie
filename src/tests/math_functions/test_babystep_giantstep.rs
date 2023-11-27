#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::babystep_giantstep::{log_naiv, shanks};

    #[test]
    fn shanks_test() {
        let shanks_now = SystemTime::now();
        assert_eq!(
            shanks(&big_i!(8), &big_i!(555), &big_i!(677), false).unwrap(), //TODO UseFast einbauen
            big_i!(134)
        );
        assert_eq!(
            shanks(&big_i!(11), &big_i!(3), &big_i!(29), false).unwrap(), //TODO UseFast einbauen
            big_i!(17)
        );
        assert_eq!(
            shanks(&big_i!(10), &big_i!(25), &big_i!(97), false).unwrap(), //TODO UseFast einbauen
            big_i!(22)
        );
        assert_eq!(
            shanks(&big_i!(3), &big_i!(4), &big_i!(7), false).unwrap(), //TODO UseFast einbauen
            big_i!(4)
        );
        assert!(shanks(&big_i!(4), &big_i!(6), &big_i!(7), false).is_err()); //Da Base nicht primitive Wurzel! //TODO UseFast einbauen
        println!("Shanks-Tests in {:?}", shanks_now.elapsed().unwrap());
    }

    #[test]
    fn log_naiv_test() {
        let naiv_now = SystemTime::now();
        assert_eq!(
            log_naiv(&big_i!(8), &big_i!(555), &big_i!(677), false).unwrap(), //TODO UseFast einbauen
            big_i!(134)
        );
        assert_eq!(
            log_naiv(&big_i!(11), &big_i!(3), &big_i!(29), false).unwrap(), //TODO UseFast einbauen
            big_i!(17)
        );
        assert_eq!(
            log_naiv(&big_i!(10), &big_i!(25), &big_i!(97), false).unwrap(), //TODO UseFast einbauen
            big_i!(22)
        );
        assert_eq!(
            log_naiv(&big_i!(3), &big_i!(4), &big_i!(7), false).unwrap(), //TODO UseFast einbauen
            big_i!(4)
        );
        assert!(log_naiv(&big_i!(4), &big_i!(6), &big_i!(7), false).is_err()); //Da Base nicht primitive Wurzel! //TODO UseFast einbauen
        println!("Naiv-Tests in {:?}", naiv_now.elapsed().unwrap());
    }
}
