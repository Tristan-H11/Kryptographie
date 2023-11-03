#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::number_theory::{
        extended_euclid, fast_exponentiation, miller_rabin, modulo_inverse,
    };

    #[test]
    fn fast_exponentiation_happy_flow() {
        let base = &big_i!(561563);
        let exponent = &big_i!(1300);
        let modul = &big_i!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), big_i!(205));

        let base = &big_i!(56156334590832345);
        let exponent = &big_i!(109458390583094852904812340);
        let modul = &big_i!(564234859);
        assert_eq!(
            fast_exponentiation(base, exponent, modul),
            big_i!(558376785)
        );
    }

    #[test]
    fn fast_exponentiation_exponent_one() {
        let base = &big_i!(561563);
        let exponent = &big_i!(1);
        let modul = &big_i!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), big_i!(383));
    }

    #[test]
    fn fast_exponentiation_exponent_zero() {
        let base = &big_i!(561563);
        let exponent = &big_i!(0);
        let modul = &big_i!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), big_i!(1));
    }

    #[test]
    fn fast_exponentiation_base_zero() {
        let base = &big_i!(0);
        let exponent = &big_i!(561563);
        let modul = &big_i!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), big_i!(0));
    }

    #[test]
    fn fast_exponentiation_modul_one() {
        let base = &big_i!(3459860);
        let exponent = &big_i!(561563);
        let modul = &big_i!(1);
        assert_eq!(fast_exponentiation(base, exponent, modul), big_i!(0));
    }

    #[test]
    fn fast_exponentiation_big_numbers() {
        let base = &big_i!(3459860).pow(50);
        let exponent = &big_i!(561563).pow(50);
        let modul = &big_i!(345902).pow(50);
        assert_eq!(
            fast_exponentiation(base, exponent, modul),
            BigInt::from_str("8408769600151667634624424658533698267981486479206207837400880482584240253227309957477768628293816726387378101618036878012339908404810884160600885622896014991205830046127587869666780551862829518403374323613697514544975024376847169484921172903282824411000834600056510593848311808").unwrap()
        );

        let base = &big_i!(5345890).pow(50);
        let exponent = &big_i!(561563).pow(50);
        let modul = &big_i!(402).pow(453);
        assert_eq!(
            fast_exponentiation(base, exponent, modul),
            BigInt::from_str("3865812286670140244135659583582784422868607182053532234892119221318009519049840848928181595205903918350994052685502954280587622821049262028969781448838683538880584872202936321233747460151097175519490133290457481138503925299942681667894197628591512134856836351617429125260375231408269314105167853219462996964862842118589984098650391290176408417243048103776854044821474824889320850667507890395769983356345061882029969478502315374312501247186250351754934295084246986742329795390786311078628530440754856479205141409578653316171690967468840184450509388030981031737577810850620779725958417192948561835194773695941361612225803217565675522961705627046360944634654621872586535332182764197222888226270388954006295237642124544864715066176310557964933132099186352875757363791307327674799516376108738026266118794617758595269081482227829599835454778768141217737588053896979935711190022064516393010329090283156674631737792250605934457811116131051558041088439411741976788375428610551855415643131350851177642819664715803674680663207355557044121133492467414028969595889934382173724212977872023208326609954389561973252556941619801122899324611797790397387249251896640501121328429738301895459647520768").unwrap()
        );

        assert_eq!(
            fast_exponentiation(&big_i!(37), &big_i!(2), &big_i!(89)),
            big_i!(34)
        )
    }

    #[test]
    fn modulo_inverse_test() {
        //assert_eq!(modulo_inverse(&big_i!(1), &big_i!(3)).unwrap(), big_i!(1));
        assert_eq!(modulo_inverse(&big_i!(5), &big_i!(11)).unwrap(), big_i!(9));
        assert_eq!(
            modulo_inverse(&big_i!(315), &big_i!(661643)).unwrap(),
            big_i!(342374)
        );
        assert_eq!(
            modulo_inverse(
                &BigInt::from_str("485398853520739824211578869461").unwrap(),
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap()
            )
            .unwrap(),
            BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap()
        );
        assert!(modulo_inverse(&big_i!(78), &big_i!(99)).is_err());
    }

    #[test]
    fn extended_euclid_test() {
        assert_eq!(
            extended_euclid(&big_i!(6), &big_i!(11)),
            (big_i!(1), big_i!(2), big_i!(-1))
        );
        assert_eq!(
            extended_euclid(&big_i!(78), &big_i!(99)),
            (big_i!(3), big_i!(14), big_i!(-11))
        );
        assert_eq!(
            extended_euclid(&big_i!(315), &big_i!(661643)),
            (big_i!(1), big_i!(-319269), big_i!(152))
        );
        assert_eq!(
            extended_euclid(&big_i!(315), &big_i!(661643)),
            (big_i!(1), big_i!(-319269), big_i!(152))
        );
        assert_eq!(
            extended_euclid(
                &BigInt::from_str("485398853520739824211578869461").unwrap(),
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap()
            ),
            (
                big_i!(1),
                BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap(),
                big_i!(-43732645957409398462249346726)
            )
        );
    }

    #[test]
    fn miller_rabin_test() {
        assert_eq!(miller_rabin(&big_i!(11), 40, &big_i!(11)), true);
        assert_eq!(miller_rabin(&big_i!(8727030382015287123761), 40, &big_i!(11)), false);
        assert_eq!(miller_rabin(&big_i!(2459872438590349034582), 40, &big_i!(11)), false);
        assert_eq!(miller_rabin(&big_i!(2211), 40, &big_i!(11)), false);
        assert_eq!(
            miller_rabin(
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap(),
                40,
                &big_i!(11)
            ),
            true
        );
    }
}
